use super::{CompileOptions, compile_with_options};
use std::path::{Path, PathBuf};
use super::parallel::{ParallelBuild, BuildResult, parallel_compile};

pub fn compile_multiple_files(
    files: &[String],
    options: &CompileOptions,
    workers: Option<usize>,
) -> Result<Vec<BuildResult>, String> {
    let mut builder = ParallelBuild::new(workers);
    
    for file in files {
        let path = Path::new(file);
        builder.add_file(path.to_path_buf(), Vec::new());
    }
    
    let options = options.clone();
    let compile_fn = move |path: &Path| -> Result<(), String> {
        let mut opts = options.clone();
        opts.input_file = path.to_string_lossy().to_string();
        
        // Generate output name from input
        if let Some(stem) = path.file_stem() {
            let ext = opts.emit_format.default_extension();
            if ext.is_empty() {
                opts.output_file = Some(stem.to_string_lossy().to_string());
            } else {
                opts.output_file = Some(format!("{}.{}", stem.to_string_lossy(), ext));
            }
        }
        
        compile_with_options(path.to_str().unwrap(), &opts)
    };
    
    let results = builder.build_all(compile_fn);
    
    Ok(results)
}

pub fn compile_project(
    project_dir: &str,
    options: &CompileOptions,
    workers: Option<usize>,
) -> Result<Vec<BuildResult>, String> {
    let project_path = Path::new(project_dir);
    
    let mut aly_files = Vec::new();
    find_aly_files(project_path, &mut aly_files)?;
    
    if aly_files.is_empty() {
        return Err("No .aly files found in project directory".to_string());
    }
    
    compile_multiple_files(&aly_files, options, workers)
}

fn find_aly_files(dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            find_aly_files(&path, files)?;
        } else if path.extension().map(|e| e == "aly").unwrap_or(false) {
            files.push(path.to_string_lossy().to_string());
        }
    }
    
    Ok(())
}

pub fn parallel_compile_files(
    files: &[String],
    options: &CompileOptions,
    workers: Option<usize>,
) -> Vec<BuildResult> {
    let options = options.clone();
    let files_with_deps: Vec<(PathBuf, Vec<PathBuf>)> = files.iter()
        .map(|f| (Path::new(f).to_path_buf(), Vec::new()))
        .collect();
    
    parallel_compile(&files_with_deps, move |path| {
        let mut opts = options.clone();
        opts.input_file = path.to_string_lossy().to_string();
        
        if let Some(stem) = path.file_stem() {
            let ext = opts.emit_format.default_extension();
            if ext.is_empty() {
                opts.output_file = Some(stem.to_string_lossy().to_string());
            } else {
                opts.output_file = Some(format!("{}.{}", stem.to_string_lossy(), ext));
            }
        }
        
        compile_with_options(path.to_str().unwrap(), &opts)
    }, workers)
}