use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ParallelBuild {
    num_workers: usize,
    build_graph: BuildGraph,
    progress: Arc<ProgressTracker>,
}

struct BuildGraph {
    nodes: HashMap<PathBuf, BuildNode>,
    edges: HashMap<PathBuf, Vec<PathBuf>>,
}

struct BuildNode {
    deps: Vec<PathBuf>,
    built: bool,
    failed: bool,
}

struct ProgressTracker {
    total: AtomicUsize,
    completed: AtomicUsize,
    failed: AtomicUsize,
}

#[derive(Debug)]
pub enum BuildResult {
    Success(PathBuf),
    Skipped(PathBuf),
    Failed(PathBuf, String),
}

impl ParallelBuild {
    pub fn new(num_workers: Option<usize>) -> Self {
        let workers = num_workers.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        });
        ParallelBuild {
            num_workers: workers,
            build_graph: BuildGraph {
                nodes: HashMap::new(),
                edges: HashMap::new(),
            },
            progress: Arc::new(ProgressTracker {
                total: AtomicUsize::new(0),
                completed: AtomicUsize::new(0),
                failed: AtomicUsize::new(0),
            }),
        }
    }

    pub fn add_file(&mut self, path: PathBuf, deps: Vec<PathBuf>) {
        self.build_graph.nodes.insert(path.clone(), BuildNode {
            deps: deps.clone(),
            built: false,
            failed: false,
        });
        for dep in deps {
            self.build_graph.edges.entry(dep)
                .or_default()
                .push(path.clone());
        }
    }

    pub fn add_files(&mut self, files: &[(PathBuf, Vec<PathBuf>)]) {
        for (path, deps) in files {
            self.add_file(path.clone(), deps.clone());
        }
    }

    fn get_leaf_nodes(&self) -> Vec<PathBuf> {
        self.build_graph.nodes.iter()
            .filter(|(_, node)| node.deps.is_empty() && !node.built && !node.failed)
            .map(|(path, _)| path.clone())
            .collect()
    }

    fn get_ready_nodes(&self) -> Vec<PathBuf> {
        self.build_graph.nodes.iter()
            .filter(|(_, node)| {
                if node.built || node.failed {
                    return false;
                }
                node.deps.iter().all(|dep| {
                    self.build_graph.nodes.get(dep)
                        .map(|n| n.built)
                        .unwrap_or(true)
                })
            })
            .map(|(path, _)| path.clone())
            .collect()
    }

    pub fn build_all<F>(&self, build_fn: F) -> Vec<BuildResult>
    where F: Fn(&Path) -> Result<(), String> + Send + Sync + 'static
    {
        let total = self.build_graph.nodes.len();
        self.progress.total.store(total, Ordering::SeqCst);

        let results = Arc::new(Mutex::new(Vec::new()));
        let graph = Arc::new(Mutex::new(self.build_graph.clone_inner()));
        let progress = self.progress.clone();
        let build_fn = Arc::new(build_fn);

        let pool_size = self.num_workers.min(total);
        let mut handles = Vec::new();

        for _ in 0..pool_size {
            let results = Arc::clone(&results);
            let graph = Arc::clone(&graph);
            let progress = Arc::clone(&progress);
            let build_fn = Arc::clone(&build_fn);

            handles.push(thread::spawn(move || {
                loop {
                    let path = {
                        let mut g = graph.lock().unwrap();
                        let ready = Self::get_ready_nodes_from(&g);
                        if ready.is_empty() && g.values().all(|n| n.built || n.failed) {
                            return;
                        }
                        match ready.into_iter().next() {
                            Some(p) => {
                                if let Some(node) = g.get_mut(&p) {
                                    if node.built || node.failed {
                                        continue;
                                    }
                                }
                                p
                            }
                            None => {
                                thread::yield_now();
                                continue;
                            }
                        }
                    };

                    let mut to_build = false;
                    {
                        let mut g = graph.lock().unwrap();
                        if let Some(node) = g.get_mut(&path) {
                            if !node.built && !node.failed {
                                node.built = true; // claim it
                                to_build = true;
                            }
                        }
                    }

                    if to_build {
                        let result = build_fn(&path);
                        let mut g = graph.lock().unwrap();
                        let br = match result {
                            Ok(()) => {
                                if let Some(node) = g.get_mut(&path) {
                                    node.built = true;
                                }
                                progress.completed.fetch_add(1, Ordering::SeqCst);
                                BuildResult::Success(path.clone())
                            }
                            Err(e) => {
                                if let Some(node) = g.get_mut(&path) {
                                    node.failed = true;
                                }
                                progress.failed.fetch_add(1, Ordering::SeqCst);
                                BuildResult::Failed(path.clone(), e)
                            }
                        };
                        results.lock().unwrap().push(br);
                    }
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        Arc::try_unwrap(results).unwrap().into_inner().unwrap()
    }

    fn get_ready_nodes_from(nodes: &HashMap<PathBuf, BuildNode>) -> Vec<PathBuf> {
        nodes.iter()
            .filter(|(_, node)| {
                if node.built || node.failed { return false; }
                node.deps.iter().all(|dep| {
                    nodes.get(dep).map(|n| n.built).unwrap_or(true)
                })
            })
            .map(|(path, _)| path.clone())
            .collect()
    }

    pub fn build_sequential<F>(&self, build_fn: F) -> Vec<BuildResult>
    where F: Fn(&Path) -> Result<(), String>
    {
        let mut results = Vec::new();
        let mut graph = self.build_graph.clone_inner();
        let total = graph.len();

        loop {
            let ready = Self::get_ready_nodes_from(&graph);
            if ready.is_empty() {
                break;
            }
            for path in &ready {
                match build_fn(path) {
                    Ok(()) => {
                        if let Some(node) = graph.get_mut(path) {
                            node.built = true;
                        }
                        results.push(BuildResult::Success(path.clone()));
                    }
                    Err(e) => {
                        if let Some(node) = graph.get_mut(path) {
                            node.failed = true;
                        }
                        results.push(BuildResult::Failed(path.clone(), e));
                    }
                }
            }
        }
        results
    }

    pub fn get_progress(&self) -> (usize, usize, usize) {
        (
            self.progress.total.load(Ordering::SeqCst),
            self.progress.completed.load(Ordering::SeqCst),
            self.progress.failed.load(Ordering::SeqCst),
        )
    }

    pub fn worker_count(&self) -> usize {
        self.num_workers
    }

    pub fn dependency_graph(&self) -> &HashMap<PathBuf, Vec<PathBuf>> {
        &self.build_graph.edges
    }
}

impl BuildNode {
    fn clone_inner(&self) -> Self {
        BuildNode {
            deps: self.deps.clone(),
            built: self.built,
            failed: self.failed,
        }
    }
}

impl BuildGraph {
    fn clone_inner(&self) -> HashMap<PathBuf, BuildNode> {
        self.nodes.iter().map(|(k, v)| (k.clone(), v.clone_inner())).collect()
    }
}

pub fn parallel_compile<F>(
    files: &[(PathBuf, Vec<PathBuf>)],
    build_fn: F,
    workers: Option<usize>,
) -> Vec<BuildResult>
where F: Fn(&Path) -> Result<(), String> + Send + Sync + 'static
{
    let mut builder = ParallelBuild::new(workers);
    builder.add_files(files);
    builder.build_all(build_fn)
}
