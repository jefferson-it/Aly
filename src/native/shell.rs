mod shell_lib {
    use std::collections::HashMap;
    use std::io::Read;
    use std::process::{Child, Command, Stdio};
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    use crate::native::create_object::Object;
    use crate::native::std::{arg, split_args};
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::native::platform::{shell_cmd, which_cmd};
    use crate::validators::str::{put_quoted_str, remove_quoted_str};

    fn shell_command(cmd: &str) -> std::process::Command {
        let (sh, flag) = shell_cmd();
        let mut c = std::process::Command::new(sh);
        c.arg(flag).arg(cmd);
        c
    }

    fn which_command(cmd: &str) -> std::process::Command {
        let mut c = std::process::Command::new(which_cmd());
        c.arg(cmd);
        c
    }

    // ── Active child processes ──────────────────────────────────────────
    static PROCESSES: Lazy<Mutex<HashMap<u32, Child>>> = Lazy::new(|| Mutex::new(HashMap::new()));

    fn next_pid() -> u32 {
        static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));
        let mut c = COUNTER.lock().unwrap();
        let pid = *c;
        *c += 1;
        pid
    }

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str(s))
    }

    fn ok_int(i: i32) -> Box<dyn Validator> {
        Box::new(ValueData::Int(i))
    }

    fn ok_bool(b: bool) -> Box<dyn Validator> {
        Box::new(ValueData::Bool(b))
    }

    #[allow(dead_code)]
    fn ok_none() -> Box<dyn Validator> {
        Box::new(ValueData::String("None".to_owned()))
    }

    fn ok_vec(v: Vec<ValueData>) -> Box<dyn Validator> {
        Box::new(ValueData::Vec(Vector::new(v)))
    }

    // ====================================================================
    // shell.exec(cmd) -> string
    // Execute a command via sh -c and return stdout (or stderr).
    // ====================================================================
    pub fn shell_exec(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let output = match shell_command(&cmd).output() {
            Ok(o) => o,
            Err(e) => return ok_str(format!("error: {}", e)),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !stdout.is_empty() {
            ok_str(stdout.trim_end().to_string())
        } else {
            ok_str(stderr.trim_end().to_string())
        }
    }

    // ====================================================================
    // shell.exec_lines(cmd) -> vector of strings (one per line)
    // ====================================================================
    pub fn shell_exec_lines(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let output = match shell_command(&cmd).output() {
            Ok(o) => o,
            Err(e) => return ok_vec(vec![ValueData::String(format!("error: {}", e))]),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<ValueData> = stdout
            .lines()
            .map(|l| ValueData::String(l.to_string()))
            .collect();

        ok_vec(lines)
    }

    // ====================================================================
    // shell.exec_status(cmd) -> object { code: int, stdout: string, stderr: string }
    // ====================================================================
    pub fn shell_exec_status(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let output = match shell_command(&cmd).output() {
            Ok(o) => o,
            Err(e) => {
                let mut map = linked_hash_map::LinkedHashMap::new();
                map.insert("code".to_string(), ValueData::Int(-1));
                map.insert("stdout".to_string(), ValueData::String(String::new()));
                map.insert("stderr".to_string(), ValueData::String(format!("error: {}", e)));
                return Box::new(ValueData::Object(Object::from_map(map)));
            }
        };

        let mut map = linked_hash_map::LinkedHashMap::new();
        map.insert(
            "code".to_string(),
            ValueData::Int(output.status.code().unwrap_or(-1)),
        );
        map.insert(
            "stdout".to_string(),
            ValueData::String(String::from_utf8_lossy(&output.stdout).trim_end().to_string()),
        );
        map.insert(
            "stderr".to_string(),
            ValueData::String(String::from_utf8_lossy(&output.stderr).trim_end().to_string()),
        );
        Box::new(ValueData::Object(Object::from_map(map)))
    }

    // ====================================================================
    // shell.spawn(cmd) -> int (pid)
    // Spawn a process in the background (non-blocking).
    // ====================================================================
    pub fn shell_spawn(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let child = match Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("shell.spawn: erro ao executar '{}': {}", cmd, e);
                return ok_int(-1);
            }
        };

        let pid = next_pid();
        PROCESSES.lock().unwrap().insert(pid, child);
        ok_int(pid as i32)
    }

    // ====================================================================
    // shell.wait(pid) -> int (exit code)
    // Block until the process finishes and return its exit code.
    // ====================================================================
    pub fn shell_wait(x: String) -> Box<dyn Validator> {
        let pid_val = arg(&split_args(&x, 1), 0)
            .trim()
            .parse::<u32>()
            .unwrap_or(0);

        let mut procs = PROCESSES.lock().unwrap();
        if let Some(child) = procs.get_mut(&pid_val) {
            match child.wait() {
                Ok(status) => {
                    let code = status.code().unwrap_or(-1);
                    procs.remove(&pid_val);
                    ok_int(code)
                }
                Err(e) => {
                    eprintln!("shell.wait: erro ao esperar pid {}: {}", pid_val, e);
                    ok_int(-1)
                }
            }
        } else {
            eprintln!("shell.wait: pid {} não encontrado", pid_val);
            ok_int(-1)
        }
    }

    // ====================================================================
    // shell.wait_output(pid) -> object { code: int, stdout: string, stderr: string }
    // Wait for process and collect all output.
    // ====================================================================
    pub fn shell_wait_output(x: String) -> Box<dyn Validator> {
        let pid_val = arg(&split_args(&x, 1), 0)
            .trim()
            .parse::<u32>()
            .unwrap_or(0);

        let mut procs = PROCESSES.lock().unwrap();
        if let Some(child) = procs.get_mut(&pid_val) {
            // Take stdout/stderr before wait
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            match child.wait() {
                Ok(status) => {
                    let code = status.code().unwrap_or(-1);
                    procs.remove(&pid_val);

                    let stdout_str = stdout
                        .map(|mut h| {
                            let mut buf = String::new();
                            h.read_to_string(&mut buf).unwrap_or(0);
                            buf.trim_end().to_string()
                        })
                        .unwrap_or_default();

                    let stderr_str = stderr
                        .map(|mut h| {
                            let mut buf = String::new();
                            h.read_to_string(&mut buf).unwrap_or(0);
                            buf.trim_end().to_string()
                        })
                        .unwrap_or_default();

                    let mut map = linked_hash_map::LinkedHashMap::new();
                    map.insert("code".to_string(), ValueData::Int(code));
                    map.insert("stdout".to_string(), ValueData::String(stdout_str));
                    map.insert("stderr".to_string(), ValueData::String(stderr_str));
                    Box::new(ValueData::Object(Object::from_map(map)))
                }
                Err(e) => {
                    procs.remove(&pid_val);
                    let mut map = linked_hash_map::LinkedHashMap::new();
                    map.insert("code".to_string(), ValueData::Int(-1));
                    map.insert("stdout".to_string(), ValueData::String(String::new()));
                    map.insert("stderr".to_string(), ValueData::String(format!("error: {}", e)));
                    Box::new(ValueData::Object(Object::from_map(map)))
                }
            }
        } else {
            let mut map = linked_hash_map::LinkedHashMap::new();
            map.insert("code".to_string(), ValueData::Int(-1));
            map.insert("stdout".to_string(), ValueData::String(String::new()));
            map.insert("stderr".to_string(), ValueData::String(format!("pid {} não encontrado", pid_val)));
            Box::new(ValueData::Object(Object::from_map(map)))
        }
    }

    // ====================================================================
    // shell.kill(pid) -> bool
    // Kill a background process.
    // ====================================================================
    pub fn shell_kill(x: String) -> Box<dyn Validator> {
        let pid_val = arg(&split_args(&x, 1), 0)
            .trim()
            .parse::<u32>()
            .unwrap_or(0);

        let mut procs = PROCESSES.lock().unwrap();
        if let Some(child) = procs.get_mut(&pid_val) {
            match child.kill() {
                Ok(_) => {
                    procs.remove(&pid_val);
                    ok_bool(true)
                }
                Err(e) => {
                    eprintln!("shell.kill: erro ao matar pid {}: {}", pid_val, e);
                    ok_bool(false)
                }
            }
        } else {
            ok_bool(false)
        }
    }

    // ====================================================================
    // shell.pid_exists(pid) -> bool
    // ====================================================================
    pub fn shell_pid_exists(x: String) -> Box<dyn Validator> {
        let pid_val = arg(&split_args(&x, 1), 0)
            .trim()
            .parse::<u32>()
            .unwrap_or(0);

        let procs = PROCESSES.lock().unwrap();
        ok_bool(procs.contains_key(&pid_val))
    }

    // ====================================================================
    // shell.pipe(cmds) -> string
    // Pipe a list of commands: ["cat file.txt", "grep foo", "wc -l"]
    // Equivalent to: cat file.txt | grep foo | wc -l
    // ====================================================================
    pub fn shell_pipe(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let raw = arg(&args, 0);

        // Parse the command list — accepts Aly vector literal or comma-separated
        let cmds: Vec<String> = if raw.starts_with('[') {
            // Aly vector literal: ["cmd1", "cmd2"]
            let inner = raw.trim_start_matches('[').trim_end_matches(']').trim();
            inner
                .split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            // Comma-separated: "cmd1", "cmd2"
            raw.split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        };

        if cmds.is_empty() {
            return ok_str(String::new());
        }

        let mut prev_output: Option<std::process::ChildStdout> = None;

        for (i, cmd) in cmds.iter().enumerate() {
            let is_last = i == cmds.len() - 1;

            let mut builder = Command::new("sh");
            builder.arg("-c").arg(cmd);

            if let Some(out) = prev_output.take() {
                builder.stdin(out);
            }

            if is_last {
                builder.stdout(Stdio::piped());
                builder.stderr(Stdio::piped());

                match builder.output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        return ok_str(stdout.trim_end().to_string());
                    }
                    Err(e) => return ok_str(format!("pipe error: {}", e)),
                }
            } else {
                builder.stdout(Stdio::piped());
                builder.stderr(Stdio::null());

                match builder.spawn() {
                    Ok(mut child) => {
                        prev_output = child.stdout.take();
                    }
                    Err(e) => return ok_str(format!("pipe error: {}", e)),
                }
            }
        }

        ok_str(String::new())
    }

    // ====================================================================
    // shell.pipe_lines(cmds) -> vector of strings
    // Same as pipe but returns lines as a vector.
    // ====================================================================
    pub fn shell_pipe_lines(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let raw = arg(&args, 0);

        let cmds: Vec<String> = if raw.starts_with('[') {
            let inner = raw.trim_start_matches('[').trim_end_matches(']').trim();
            inner
                .split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            raw.split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        };

        if cmds.is_empty() {
            return ok_vec(vec![]);
        }

        let mut prev_output: Option<std::process::ChildStdout> = None;

        for (i, cmd) in cmds.iter().enumerate() {
            let is_last = i == cmds.len() - 1;

            let mut builder = Command::new("sh");
            builder.arg("-c").arg(cmd);

            if let Some(out) = prev_output.take() {
                builder.stdin(out);
            }

            if is_last {
                builder.stdout(Stdio::piped());
                builder.stderr(Stdio::piped());

                match builder.output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        let lines: Vec<ValueData> = stdout
                            .lines()
                            .map(|l| ValueData::String(l.to_string()))
                            .collect();
                        return ok_vec(lines);
                    }
                    Err(e) => return ok_vec(vec![ValueData::String(format!("pipe error: {}", e))]),
                }
            } else {
                builder.stdout(Stdio::piped());
                builder.stderr(Stdio::null());

                match builder.spawn() {
                    Ok(mut child) => {
                        prev_output = child.stdout.take();
                    }
                    Err(e) => return ok_vec(vec![ValueData::String(format!("pipe error: {}", e))]),
                }
            }
        }

        ok_vec(vec![])
    }

    // ====================================================================
    // shell.run_in_shell(cmd, shell_path) -> string
    // Execute using a specific shell (e.g. /bin/bash, /bin/zsh).
    // ====================================================================
    pub fn shell_run_in_shell(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let cmd = arg(&args, 0);
        let shell_path = if args.len() > 1 && !args[1].trim().is_empty() {
            arg(&args, 1)
        } else {
            // Default to user's $SHELL or /bin/sh
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
        };

        let output = match Command::new(&shell_path)
            .arg("-c")
            .arg(&cmd)
            .output()
        {
            Ok(o) => o,
            Err(e) => return ok_str(format!("error: {}", e)),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        ok_str(stdout.trim_end().to_string())
    }

    // ====================================================================
    // shell.env_get(name) -> string
    // ====================================================================
    pub fn shell_env_get(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match std::env::var(&name) {
            Ok(v) => ok_str(v),
            Err(_) => ok_str(String::new()),
        }
    }

    // ====================================================================
    // shell.env_set(name, value) -> bool
    // ====================================================================
    pub fn shell_env_set(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let name = arg(&args, 0);
        let value = arg(&args, 1);

        std::env::set_var(&name, &value);
        ok_bool(true)
    }

    // ====================================================================
    // shell.env_remove(name) -> bool
    // ====================================================================
    pub fn shell_env_remove(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        std::env::remove_var(&name);
        ok_bool(true)
    }

    // ====================================================================
    // shell.env_list() -> object { KEY: value, ... }
    // ====================================================================
    pub fn shell_env_list(_x: String) -> Box<dyn Validator> {
        let mut map = linked_hash_map::LinkedHashMap::new();
        for (key, value) in std::env::vars() {
            map.insert(key, ValueData::String(value));
        }
        Box::new(ValueData::Object(Object::from_map(map)))
    }

    // ====================================================================
    // shell.cd(path) -> bool
    // Change the current working directory of the process.
    // ====================================================================
    pub fn shell_cd(x: String) -> Box<dyn Validator> {
        let path = arg(&split_args(&x, 1), 0);

        let target = if path.is_empty() {
            dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."))
        } else if path.starts_with('~') {
            let home = dirs::home_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            std::path::PathBuf::from(path.replacen('~', &home, 1))
        } else {
            std::path::PathBuf::from(&path)
        };

        match std::env::set_current_dir(&target) {
            Ok(_) => ok_bool(true),
            Err(e) => {
                eprintln!("shell.cd: erro ao mudar para '{}': {}", target.display(), e);
                ok_bool(false)
            }
        }
    }

    // ====================================================================
    // shell.pwd() -> string
    // ====================================================================
    pub fn shell_pwd(_x: String) -> Box<dyn Validator> {
        match std::env::current_dir() {
            Ok(p) => ok_str(p.display().to_string()),
            Err(_) => ok_str(String::new()),
        }
    }

    // ====================================================================
    // shell.which(cmd) -> string (path to the command, or empty)
    // ====================================================================
    pub fn shell_which(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let output = Command::new("which")
            .arg(&cmd)
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        ok_str(output)
    }

    // ====================================================================
    // shell.is_command(cmd) -> bool
    // Check if a command exists in PATH.
    // ====================================================================
    pub fn shell_is_command(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let found = Command::new("which")
            .arg(&cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        ok_bool(found)
    }

    // ====================================================================
    // shell.signal(pid, signal_name) -> bool
    // Send a signal to a process (e.g. "SIGTERM", "SIGKILL", "SIGHUP").
    // Uses kill -s on Unix.
    // ====================================================================
    pub fn shell_signal(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let pid_str = arg(&args, 0);
        let signal = arg(&args, 1);

        let output = Command::new("kill")
            .args(["-s", &signal, &pid_str])
            .output();

        match output {
            Ok(o) => ok_bool(o.status.success()),
            Err(_) => ok_bool(false),
        }
    }

    // ====================================================================
    // shell.process_list(cmd_filter) -> vector of objects
    // List running processes. Optional filter string applied to command.
    // Each object: { pid: int, user: string, cpu: string, mem: string, command: string }
    // ====================================================================
    pub fn shell_process_list(x: String) -> Box<dyn Validator> {
        let filter = arg(&split_args(&x, 1), 0);

        let output = match Command::new("ps").args(["aux"]).output() {
            Ok(o) => o,
            Err(_) => return ok_vec(vec![]),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let mut result = vec![];

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 11 {
                continue;
            }

            let cmd = parts[10..].join(" ");

            if !filter.is_empty() && !cmd.contains(&filter) {
                continue;
            }

            let mut map = linked_hash_map::LinkedHashMap::new();
            map.insert(
                "pid".to_string(),
                ValueData::Int(parts[1].parse::<i32>().unwrap_or(0)),
            );
            map.insert("user".to_string(), ValueData::String(parts[0].to_string()));
            map.insert("cpu".to_string(), ValueData::String(parts[2].to_string()));
            map.insert("mem".to_string(), ValueData::String(parts[3].to_string()));
            map.insert("command".to_string(), ValueData::String(cmd));

            result.push(ValueData::Object(Object::from_map(map)));
        }

        ok_vec(result)
    }

    // ====================================================================
    // shell.exit(code) -> never
    // Exit the Aly process with the given code.
    // ====================================================================
    pub fn shell_exit(x: String) -> Box<dyn Validator> {
        let code = arg(&split_args(&x, 1), 0)
            .trim()
            .parse::<i32>()
            .unwrap_or(0);
        std::process::exit(code);
    }

    // ====================================================================
    // shell.pty(cmd) -> string
    // Execute a command with pseudo-TTY allocation (for programs that
    // need a terminal, like top, less, etc). Uses script command.
    // ====================================================================
    pub fn shell_pty(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);

        let output = Command::new("script")
            .args(["-q", "-c", &cmd, "/dev/null"])
            .output();

        match output {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout).to_string();
                // Strip ANSI escape sequences
                let clean = strip_ansi(&stdout);
                ok_str(clean.trim_end().to_string())
            }
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    fn strip_ansi(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                // Skip until a letter (the command terminator)
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    // ====================================================================
    // shell.thread_spawn(function_name) -> int (thread id)
    // Spawn a OS thread to execute code concurrently.
    // ====================================================================
    pub fn shell_thread_spawn(x: String) -> Box<dyn Validator> {
        let func_name = arg(&split_args(&x, 1), 0);
        
        let _handle = std::thread::spawn(move || {
            let run = crate::aly::get_runtime();
            let fake = crate::lexer::Lexer::new(crate::tokens::Tokens::Reference, func_name, 0);
            if let Ok(var) = run.get_var(fake) {
                if let ValueData::Function(lexers) = var.get_value() {
                    let mut lexers_clone = lexers.clone();
                    let mut v: Box<dyn Validator> = Box::new(ValueData::String("None".to_owned()));
                    crate::runtime::interpreter::exec(&mut lexers_clone, &mut v);
                }
            }
        });

        Box::new(ValueData::Int(0))
    }

    // ====================================================================
    // shell.thread_pool(funcs_list) -> vector of int (thread count)
    // Spawn multiple threads in parallel and wait for all of them to finish.
    // ====================================================================
    pub fn shell_thread_pool(x: String) -> Box<dyn Validator> {
        let raw = arg(&split_args(&x, 1), 0);
        let funcs: Vec<String> = if raw.starts_with('[') {
            let inner = raw.trim_start_matches('[').trim_end_matches(']').trim();
            inner
                .split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            raw.split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        };

        let mut handles = vec![];
        for func_name in funcs {
            let h = std::thread::spawn(move || {
                let run = crate::aly::get_runtime();
                let fake = crate::lexer::Lexer::new(crate::tokens::Tokens::Reference, func_name, 0);
                if let Ok(var) = run.get_var(fake) {
                    if let ValueData::Function(lexers) = var.get_value() {
                        let mut lexers_clone = lexers.clone();
                        let mut v: Box<dyn Validator> = Box::new(ValueData::String("None".to_owned()));
                        crate::runtime::interpreter::exec(&mut lexers_clone, &mut v);
                    }
                }
            });
            handles.push(h);
        }

        for h in handles {
            let _ = h.join();
        }

        ok_bool(true)
    }

    // ====================================================================
    // shell.multi_exec(cmds_list) -> vector of strings
    // Execute multiple shell commands in parallel processes and collect outputs.
    // ====================================================================
    pub fn shell_multi_exec(x: String) -> Box<dyn Validator> {
        let raw = arg(&split_args(&x, 1), 0);
        let cmds: Vec<String> = if raw.starts_with('[') {
            let inner = raw.trim_start_matches('[').trim_end_matches(']').trim();
            inner
                .split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            raw.split(',')
                .map(|s| remove_quoted_str(s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        };

        let mut children = vec![];
        for cmd in cmds {
            let child = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();
            children.push((cmd, child));
        }

        let mut results = vec![];
        for (_cmd, child_res) in children {
            match child_res {
                Ok(child) => {
                    let output = child.wait_with_output();
                    match output {
                        Ok(o) => {
                            let stdout = String::from_utf8_lossy(&o.stdout).trim_end().to_string();
                            results.push(ValueData::String(stdout));
                        }
                        Err(e) => results.push(ValueData::String(format!("error: {}", e))),
                    }
                }
                Err(e) => results.push(ValueData::String(format!("error: {}", e))),
            }
        }

        ok_vec(results)
    }
}

pub use shell_lib::*;
