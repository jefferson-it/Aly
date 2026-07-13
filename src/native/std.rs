// Aly standard library: native utility modules for filesystem, string
// manipulation and system access. Every function has the native signature
// `fn(String) -> Box<dyn Validator>`, receiving its arguments as a single
// comma-separated string (as assembled by `function_run`).

mod std_lib {
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::{put_quoted_str, remove_quoted_str};

    // Split a native argument string into up to `max` parts on top-level commas.
    // Quoted segments keep their commas. When `max` is 0 there is no limit.
    pub fn split_args(x: &str, max: usize) -> Vec<String> {
        let mut parts = vec![];
        let mut current = String::new();
        let mut quote: Option<char> = None;

        for ch in x.chars() {
            match quote {
                Some(q) => {
                    if ch == q {
                        quote = None;
                    }
                    current.push(ch);
                }
                None => {
                    if ch == '"' || ch == '\'' {
                        quote = Some(ch);
                        current.push(ch);
                    } else if ch == ',' && (max == 0 || parts.len() + 1 < max) {
                        parts.push(current.trim().to_string());
                        current = String::new();
                    } else {
                        current.push(ch);
                    }
                }
            }
        }

        parts.push(current.trim().to_string());
        parts
    }

    // Convenience: return the nth argument already unquoted, or empty.
    pub fn arg(args: &[String], n: usize) -> String {
        args.get(n)
            .map(|a| remove_quoted_str(a.clone()))
            .unwrap_or_default()
    }

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str(s))
    }

    fn ok_bool(b: bool) -> Box<dyn Validator> {
        Box::new(ValueData::Bool(b))
    }

    fn ok_int(i: i32) -> Box<dyn Validator> {
        Box::new(ValueData::Int(i as i64))
    }

    // ---------------------------------------------------------------------
    // Filesystem module (fs.*)
    // ---------------------------------------------------------------------

    // fs.read(path) -> file contents as string
    pub fn fs_read(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let path = arg(&args, 0);

        match fs::read_to_string(&path) {
            Ok(content) => ok_str(content),
            Err(err) => {
                eprintln!("RuntimeError [fs.read]: erro ao ler '{}': {}", path, err);
                ok_str(String::new())
            }
        }
    }

    // fs.write(path, content) -> bool (success)
    pub fn fs_write(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let path = arg(&args, 0);
        let content = arg(&args, 1);

        ok_bool(fs::write(&path, content).is_ok())
    }

    // fs.append(path, content) -> bool (success)
    pub fn fs_append(x: String) -> Box<dyn Validator> {
        use std::io::Write;

        let args = split_args(&x, 2);
        let path = arg(&args, 0);
        let content = arg(&args, 1);

        let result = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .and_then(|mut f| f.write_all(content.as_bytes()));

        ok_bool(result.is_ok())
    }

    // fs.exists(path) -> bool
    pub fn fs_exists(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let path = arg(&args, 0);

        ok_bool(Path::new(&path).exists())
    }

    // fs.remove(path) -> bool (success). Works for files and directories.
    pub fn fs_remove(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let path = arg(&args, 0);
        let p = Path::new(&path);

        let result = if p.is_dir() {
            fs::remove_dir_all(p)
        } else {
            fs::remove_file(p)
        };

        ok_bool(result.is_ok())
    }

    // fs.mkdir(path) -> bool (success). Creates parent directories as needed.
    pub fn fs_mkdir(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let path = arg(&args, 0);

        ok_bool(fs::create_dir_all(&path).is_ok())
    }

    // fs.list(path) -> vector of entry names
    pub fn fs_list(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let path = arg(&args, 0);

        let entries = match fs::read_dir(&path) {
            Ok(rd) => rd,
            Err(err) => {
                eprintln!("RuntimeError [fs.list]: erro ao listar '{}': {}", path, err);
                return Box::new(ValueData::Vec(Vector::new(vec![])));
            }
        };

        let mut names = vec![];
        for entry in entries.flatten() {
            names.push(ValueData::String(entry.file_name().to_string_lossy().to_string()));
        }

        Box::new(ValueData::Vec(Vector::new(names)))
    }

    // fs.is_dir(path) -> bool
    pub fn fs_is_dir(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let path = arg(&args, 0);

        ok_bool(Path::new(&path).is_dir())
    }

    // ---------------------------------------------------------------------
    // String module (str.*)
    // ---------------------------------------------------------------------

    // str.upper(s) -> uppercase
    pub fn str_upper(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        ok_str(arg(&args, 0).to_uppercase())
    }

    // str.lower(s) -> lowercase
    pub fn str_lower(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        ok_str(arg(&args, 0).to_lowercase())
    }

    // str.trim(s) -> trimmed
    pub fn str_trim(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        ok_str(arg(&args, 0).trim().to_string())
    }

    // str.contains(s, needle) -> bool
    pub fn str_contains(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        ok_bool(arg(&args, 0).contains(&arg(&args, 1)))
    }

    // str.replace(s, from, to) -> string
    pub fn str_replace(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let s = arg(&args, 0);
        let from = arg(&args, 1);
        let to = arg(&args, 2);
        ok_str(s.replace(&from, &to))
    }

    // str.split(s, sep) -> vector of parts
    pub fn str_split(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let s = arg(&args, 0);
        let sep = arg(&args, 1);

        let parts: Vec<ValueData> = if sep.is_empty() {
            s.chars().map(|c| ValueData::String(c.to_string())).collect()
        } else {
            s.split(&sep).map(|p| ValueData::String(p.to_string())).collect()
        };

        Box::new(ValueData::Vec(Vector::new(parts)))
    }

    // str.starts_with(s, prefix) -> bool
    pub fn str_starts_with(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        ok_bool(arg(&args, 0).starts_with(&arg(&args, 1)))
    }

    // str.ends_with(s, suffix) -> bool
    pub fn str_ends_with(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        ok_bool(arg(&args, 0).ends_with(&arg(&args, 1)))
    }

    // str.index_of(s, needle) -> int (byte index, or -1)
    pub fn str_index_of(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let s = arg(&args, 0);
        let needle = arg(&args, 1);

        match s.find(&needle) {
            Some(idx) => ok_int(idx as i32),
            None => ok_int(-1),
        }
    }

    // str.repeat(s, n) -> string
    pub fn str_repeat(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let s = arg(&args, 0);
        let n = arg(&args, 1).trim().parse::<usize>().unwrap_or(0);
        ok_str(s.repeat(n))
    }

    // ---------------------------------------------------------------------
    // System module (sys.*)
    // ---------------------------------------------------------------------

    // sys.env(name) -> value or empty string
    pub fn sys_env(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let name = arg(&args, 0);
        ok_str(std::env::var(&name).unwrap_or_default())
    }

    // sys.args() -> vector of process arguments
    pub fn sys_args(_x: String) -> Box<dyn Validator> {
        let args: Vec<ValueData> = std::env::args()
            .map(ValueData::String)
            .collect();
        Box::new(ValueData::Vec(Vector::new(args)))
    }

    // sys.time() -> unix timestamp in seconds
    pub fn sys_time(_x: String) -> Box<dyn Validator> {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        ok_int(secs as i32)
    }

    // sys.platform() -> OS name (e.g. "linux", "windows", "macos")
    pub fn sys_platform(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::OS.to_string())
    }

    // sys.cwd() -> current working directory
    pub fn sys_cwd(_x: String) -> Box<dyn Validator> {
        let cwd = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        ok_str(cwd)
    }

    // sys.exit(code) -> never returns
    pub fn sys_exit(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let code = arg(&args, 0).trim().parse::<i32>().unwrap_or(0);
        std::process::exit(code);
    }

    // ---------------------------------------------------------------------
    // Hardware/Low-level emulation module (hw.*)
    // ---------------------------------------------------------------------
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Simulated Memory Management
    static SIM_MEMORY: Lazy<Mutex<HashMap<usize, Vec<u8>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
    static SIM_MEM_ADDR: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0x1000));

    // Simulated Registers
    static SIM_REGISTERS: Lazy<Mutex<HashMap<String, i32>>> = Lazy::new(|| {
        let mut reg = HashMap::new();
        reg.insert("eax".to_string(), 0);
        reg.insert("ebx".to_string(), 0);
        reg.insert("ecx".to_string(), 0);
        reg.insert("edx".to_string(), 0);
        reg.insert("esp".to_string(), 0x7FFF);
        reg.insert("ebp".to_string(), 0x7FFF);
        reg.insert("eip".to_string(), 0);
        Mutex::new(reg)
    });

    // Simulated I/O Ports
    static SIM_IO_PORTS: Lazy<Mutex<HashMap<u16, u8>>> = Lazy::new(|| Mutex::new(HashMap::new()));

    // hw.alloc(size) -> Address (int)
    pub fn hw_alloc(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let size = arg(&args, 0).trim().parse::<usize>().unwrap_or(0);
        
        let mut next_addr = SIM_MEM_ADDR.lock().unwrap();
        let addr = *next_addr;
        *next_addr += size;
        
        let mut mem = SIM_MEMORY.lock().unwrap();
        mem.insert(addr, vec![0; size]);
        
        ok_int(addr as i32)
    }

    // hw.free(address) -> bool (success)
    pub fn hw_free(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let addr = arg(&args, 0).trim().parse::<usize>().unwrap_or(0);
        
        let mut mem = SIM_MEMORY.lock().unwrap();
        ok_bool(mem.remove(&addr).is_some())
    }

    // hw.read_mem(address, offset) -> int (byte value)
    pub fn hw_read_mem(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let addr = arg(&args, 0).trim().parse::<usize>().unwrap_or(0);
        let offset = arg(&args, 1).trim().parse::<usize>().unwrap_or(0);
        
        let mem = SIM_MEMORY.lock().unwrap();
        if let Some(block) = mem.get(&addr) {
            if offset < block.len() {
                return ok_int(block[offset] as i32);
            }
        }
        
        eprintln!(
            "RuntimeError [hw.read_mem]: Segmentation Fault: leitura de endereço inválido {:#x} com offset {}",
            addr, offset
        );
        ok_int(-1)
    }

    // hw.write_mem(address, offset, value) -> bool
    pub fn hw_write_mem(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let addr = arg(&args, 0).trim().parse::<usize>().unwrap_or(0);
        let offset = arg(&args, 1).trim().parse::<usize>().unwrap_or(0);
        let val = arg(&args, 2).trim().parse::<u8>().unwrap_or(0);
        
        let mut mem = SIM_MEMORY.lock().unwrap();
        if let Some(block) = mem.get_mut(&addr) {
            if offset < block.len() {
                block[offset] = val;
                return ok_bool(true);
            }
        }
        
        eprintln!(
            "RuntimeError [hw.write_mem]: Segmentation Fault: escrita em endereço inválido {:#x} com offset {}",
            addr, offset
        );
        ok_bool(false)
    }

    // hw.read_reg(name) -> int
    pub fn hw_read_reg(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let name = arg(&args, 0).trim().to_lowercase();
        
        let regs = SIM_REGISTERS.lock().unwrap();
        match regs.get(&name) {
            Some(val) => ok_int(*val),
            None => {
                eprintln!("RuntimeError [hw.read_reg]: registrador '{}' não existe.", name);
                ok_int(0)
            }
        }
    }

    // hw.write_reg(name, value) -> bool
    pub fn hw_write_reg(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let name = arg(&args, 0).trim().to_lowercase();
        let val = arg(&args, 1).trim().parse::<i32>().unwrap_or(0);
        
        let mut regs = SIM_REGISTERS.lock().unwrap();
        if regs.contains_key(&name) {
            regs.insert(name, val);
            ok_bool(true)
        } else {
            eprintln!("RuntimeError [hw.write_reg]: registrador '{}' não existe.", name);
            ok_bool(false)
        }
    }

    // hw.out_port(port, value) -> bool
    pub fn hw_out_port(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let port = arg(&args, 0).trim().parse::<u16>().unwrap_or(0);
        let val = arg(&args, 1).trim().parse::<u8>().unwrap_or(0);
        
        let mut ports = SIM_IO_PORTS.lock().unwrap();
        ports.insert(port, val);
        ok_bool(true)
    }

    // hw.in_port(port) -> int
    pub fn hw_in_port(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let port = arg(&args, 0).trim().parse::<u16>().unwrap_or(0);
        
        let ports = SIM_IO_PORTS.lock().unwrap();
        ok_int(*ports.get(&port).unwrap_or(&0) as i32)
    }

    // hw.trigger_interrupt(num) -> bool
    pub fn hw_trigger_interrupt(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let num = arg(&args, 0).trim().parse::<u8>().unwrap_or(0);
        
        println!("[CPU INTERRUPT] Received hardware interrupt signal {:#x}", num);
        
        match num {
            0x80 => {
                let regs = SIM_REGISTERS.lock().unwrap();
                let eax = *regs.get("eax").unwrap_or(&0);
                let ebx = *regs.get("ebx").unwrap_or(&0);
                println!("[CPU INTERRUPT 0x80] Syscall ID (eax): {}, Argument (ebx): {}", eax, ebx);
            }
            _ => {}
        }
        
        ok_bool(true)
    }

    // hw.cpu_step(instruction) -> bool
    pub fn hw_cpu_step(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let inst = arg(&args, 0);
        
        let parts: Vec<&str> = inst.split_whitespace().collect();
        if parts.is_empty() {
            return ok_bool(false);
        }
        
        let op = parts[0].to_uppercase();
        let mut regs = SIM_REGISTERS.lock().unwrap();
        
        match op.as_str() {
            "MOV" => {
                if parts.len() < 3 {
                    eprintln!("SyntaxError [hw.cpu_step]: MOV requer destino e fonte.");
                    return ok_bool(false);
                }
                let dest = parts[1].to_lowercase();
                let src = parts[2].to_lowercase();

                let val = if let Ok(num) = src.parse::<i32>() {
                    num
                } else if let Some(r_val) = regs.get(&src) {
                    *r_val
                } else {
                    eprintln!("SyntaxError [hw.cpu_step]: fonte inválida '{}' para MOV.", src);
                    return ok_bool(false);
                };

                if regs.contains_key(&dest) {
                    regs.insert(dest, val);
                } else {
                    eprintln!("SyntaxError [hw.cpu_step]: registrador destino inválido '{}' para MOV.", dest);
                    return ok_bool(false);
                }
            }
            "ADD" => {
                if parts.len() < 3 {
                    eprintln!("SyntaxError [hw.cpu_step]: ADD requer destino e fonte.");
                    return ok_bool(false);
                }
                let dest = parts[1].to_lowercase();
                let src = parts[2].to_lowercase();

                let val = if let Ok(num) = src.parse::<i32>() {
                    num
                } else if let Some(r_val) = regs.get(&src) {
                    *r_val
                } else {
                    eprintln!("SyntaxError [hw.cpu_step]: fonte inválida '{}' para ADD.", src);
                    return ok_bool(false);
                };

                if let Some(dest_val) = regs.get_mut(&dest) {
                    *dest_val += val;
                } else {
                    eprintln!("SyntaxError [hw.cpu_step]: registrador destino inválido '{}' para ADD.", dest);
                    return ok_bool(false);
                }
            }
            "SUB" => {
                if parts.len() < 3 {
                    eprintln!("SyntaxError [hw.cpu_step]: SUB requer destino e fonte.");
                    return ok_bool(false);
                }
                let dest = parts[1].to_lowercase();
                let src = parts[2].to_lowercase();

                let val = if let Ok(num) = src.parse::<i32>() {
                    num
                } else if let Some(r_val) = regs.get(&src) {
                    *r_val
                } else {
                    eprintln!("SyntaxError [hw.cpu_step]: fonte inválida '{}' para SUB.", src);
                    return ok_bool(false);
                };

                if let Some(dest_val) = regs.get_mut(&dest) {
                    *dest_val -= val;
                } else {
                }
            }
            }
            _ => {
                eprintln!("RuntimeError [hw.cpu_step]: instrução CPU desconhecida '{}'.", op);
            }
        }
        
        ok_bool(true)
    }
}

pub use std_lib::*;
