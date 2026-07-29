mod system_lib {
    use std::collections::HashMap;
    use std::process::{Command, Stdio};
    use std::sync::Mutex;

    use linked_hash_map::LinkedHashMap;

    use crate::native::create_object::Object;
    use crate::native::std::{arg, split_args};
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(ValueData::String(s))
    }

    fn ok_bool(b: bool) -> Box<dyn Validator> {
        Box::new(ValueData::Bool(b))
    }

    fn ok_vec(v: Vec<ValueData>) -> Box<dyn Validator> {
        Box::new(ValueData::Vec(Vector::new(v)))
    }

    fn ok_obj(map: LinkedHashMap<String, ValueData>) -> Box<dyn Validator> {
        Box::new(ValueData::Object(Object::from_map(map)))
    }

    fn exec_cmd(shell: &str, cmd: &str) -> Result<String, String> {
        let output = Command::new(shell).arg("-c").arg(cmd).output().map_err(|e| e.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !stdout.is_empty() { Ok(stdout) } else { Ok(stderr) }
    }

    fn exec_cmd_full(shell: &str, cmd: &str) -> Result<(i32, String, String), String> {
        let output = Command::new(shell).arg("-c").arg(cmd).output().map_err(|e| e.to_string())?;
        let code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Ok((code, stdout, stderr))
    }

    // ── Bash ──────────────────────────────────────────────────────────────

    pub fn bash_exec(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);
        match exec_cmd("bash", &cmd) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn bash_run_script(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let path = arg(&args, 0);
        let extra = if args.len() > 1 && !args[1].is_empty() { args[1].clone() } else { String::new() };
        let cmd = format!("bash '{}' {}", path, extra);
        match exec_cmd("bash", &cmd) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn bash_eval(x: String) -> Box<dyn Validator> {
        let expr = arg(&split_args(&x, 1), 0);
        let cmd = format!("echo $(({}))", expr);
        match exec_cmd("bash", &cmd) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    // ── PowerShell ────────────────────────────────────────────────────────

    pub fn powershell_exec(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);
        match exec_cmd("pwsh", &format!("-Command \"{}\"", cmd.replace('"', "\\\""))) {
            Ok(s) => ok_str(s),
            Err(_) => match exec_cmd("powershell", &format!("-Command \"{}\"", cmd.replace('"', "\\\""))) {
                Ok(s) => ok_str(s),
                Err(e) => ok_str(format!("error: {}", e)),
            }
        }
    }

    pub fn powershell_run_script(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let path = arg(&args, 0);
        let extra = if args.len() > 1 && !args[1].is_empty() { args[1].clone() } else { String::new() };
        let cmd = format!("-File '{}' {}", path, extra);
        let shell = if Command::new("pwsh").arg("--version").output().is_ok() { "pwsh" } else { "powershell" };
        match exec_cmd(shell, &cmd) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn powershell_eval(x: String) -> Box<dyn Validator> {
        let expr = arg(&split_args(&x, 1), 0);
        let cmd = format!("-Command \"& {{ {} }}\"", expr.replace('"', "\\\""));
        let shell = if Command::new("pwsh").arg("--version").output().is_ok() { "pwsh" } else { "powershell" };
        match exec_cmd(shell, &cmd) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    // ── Batch (.bat) ──────────────────────────────────────────────────────

    pub fn batch_exec(x: String) -> Box<dyn Validator> {
        let cmd = arg(&split_args(&x, 1), 0);
        match exec_cmd("cmd", &format!("/c {}", cmd)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn batch_run_script(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let path = arg(&args, 0);
        let extra = if args.len() > 1 && !args[1].is_empty() { args[1].clone() } else { String::new() };
        let cmd = format!("/c \"{}\" {}", path, extra);
        match exec_cmd("cmd", &cmd) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn batch_eval(x: String) -> Box<dyn Validator> {
        let expr = arg(&split_args(&x, 1), 0);
        let cmd = format!("/c cmd /v:on /c \"set /a result={} & echo !result!\"", expr);
        match exec_cmd("cmd", &cmd) {
            Ok(s) => ok_str(s.trim().to_string()),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    // ── Daemons ───────────────────────────────────────────────────────────

    use once_cell::sync::Lazy;

    static DAEMON_PIDS: Lazy<Mutex<HashMap<String, u32>>> = Lazy::new(|| Mutex::new(HashMap::new()));

    pub fn daemon_start(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let name = arg(&args, 0);
        let cmd = arg(&args, 1);
        let logfile = if args.len() > 2 && !args[2].is_empty() { arg(&args, 2) } else { format!("/tmp/{}.log", name) };

        let stdout = match std::fs::File::create(&logfile) {
            Ok(f) => Stdio::from(f),
            Err(_) => Stdio::null(),
        };
        let child = match Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdout(stdout)
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => return ok_str(format!("error: {}", e)),
        };

        let pid = child.id();
        DAEMON_PIDS.lock().unwrap().insert(name.clone(), pid);

        let mut map = LinkedHashMap::new();
        map.insert("name".to_string(), ValueData::String(name));
        map.insert("pid".to_string(), ValueData::Int(pid as i32));
        map.insert("log".to_string(), ValueData::String(logfile));
        ok_obj(map)
    }

    pub fn daemon_stop(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        let mut daemons = DAEMON_PIDS.lock().unwrap();
        if let Some(&pid) = daemons.get(&name) {
            let _ = Command::new("kill").arg(pid.to_string()).output();
            daemons.remove(&name);
            ok_bool(true)
        } else {
            ok_bool(false)
        }
    }

    pub fn daemon_status(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        let daemons = DAEMON_PIDS.lock().unwrap();
        if let Some(&pid) = daemons.get(&name) {
            let alive = Command::new("kill").args(["-0", &pid.to_string()]).output()
                .map(|o| o.status.success()).unwrap_or(false);
            let mut map = LinkedHashMap::new();
            map.insert("name".to_string(), ValueData::String(name.clone()));
            map.insert("pid".to_string(), ValueData::Int(pid as i32));
            map.insert("running".to_string(), ValueData::Bool(alive));
            ok_obj(map)
        } else {
            let mut map = LinkedHashMap::new();
            map.insert("name".to_string(), ValueData::String(name));
            map.insert("pid".to_string(), ValueData::Int(-1));
            map.insert("running".to_string(), ValueData::Bool(false));
            ok_obj(map)
        }
    }

    pub fn daemon_restart(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let name = arg(&args, 0);
        let cmd = arg(&args, 1);
        let logfile = if args.len() > 2 && !args[2].is_empty() { arg(&args, 2) } else { format!("/tmp/{}.log", name) };

        daemon_stop(format!("\"{}\"", name));
        daemon_start(format!("\"{}\", \"{}\", \"{}\"", name, cmd, logfile))
    }

    // ── System Services (systemd) ─────────────────────────────────────────

    pub fn service_create(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 5);
        let name = arg(&args, 0);
        let exec_path = arg(&args, 1);
        let description = if args.len() > 2 && !args[2].is_empty() { arg(&args, 2) } else { name.clone() };
        let after = if args.len() > 3 && !args[3].is_empty() { arg(&args, 3) } else { "network.target".to_string() };
        let user = if args.len() > 4 && !args[4].is_empty() { arg(&args, 4) } else { String::new() };

        let unit = format!(
            "[Unit]\nDescription={}\nAfter={}\n\n[Service]\nExecStart={}\nRestart=always\n{}\n\n[Install]\nWantedBy=multi-user.target\n",
            description, after, exec_path,
            if user.is_empty() { String::new() } else { format!("User={}\n", user) }
        );

        let path = format!("/etc/systemd/system/{}.service", name);
        match std::fs::write(&path, &unit) {
            Ok(_) => {
                let mut map = LinkedHashMap::new();
                map.insert("path".to_string(), ValueData::String(path));
                map.insert("content".to_string(), ValueData::String(unit));
                ok_obj(map)
            },
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn service_start(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("systemctl", &format!("start {}", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn service_stop(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("systemctl", &format!("stop {}", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn service_restart(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("systemctl", &format!("restart {}", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn service_status(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        let (code, stdout, stderr) = match exec_cmd_full("systemctl", &format!("is-active {}", name)) {
            Ok(v) => v,
            Err(e) => {
                let mut map = LinkedHashMap::new();
                map.insert("name".to_string(), ValueData::String(name));
                map.insert("active".to_string(), ValueData::Bool(false));
                map.insert("error".to_string(), ValueData::String(e));
                return ok_obj(map);
            }
        };
        let active = code == 0;
        let mut map = LinkedHashMap::new();
        map.insert("name".to_string(), ValueData::String(name));
        map.insert("active".to_string(), ValueData::Bool(active));
        map.insert("status".to_string(), ValueData::String(if active { stdout } else { stderr }));
        ok_obj(map)
    }

    pub fn service_enable(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("systemctl", &format!("enable {}", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn service_disable(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("systemctl", &format!("disable {}", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn service_list(x: String) -> Box<dyn Validator> {
        let filter = arg(&split_args(&x, 1), 0);
        let extra = if filter.is_empty() { String::new() } else { format!(" | grep '{}'", filter) };
        let cmd = format!("systemctl list-units --type=service --no-legend{}", extra);
        match exec_cmd("bash", &cmd) {
            Ok(s) => {
                let lines: Vec<ValueData> = s.lines().map(|l| ValueData::String(l.to_string())).collect();
                ok_vec(lines)
            }
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    // ── Drivers & Devices ─────────────────────────────────────────────────

    pub fn driver_load(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let path_or_name = arg(&args, 0);
        let params = if args.len() > 1 && !args[1].is_empty() { args[1].clone() } else { String::new() };

        let (prog, full_cmd) = if path_or_name.ends_with(".ko") {
            ("insmod", format!("insmod '{}' {}", path_or_name, params))
        } else {
            ("modprobe", format!("modprobe '{}' {}", path_or_name, params))
        };

        match exec_cmd_full(prog, &full_cmd) {
            Ok((code, stdout, stderr)) => {
                let mut map = LinkedHashMap::new();
                map.insert("success".to_string(), ValueData::Bool(code == 0));
                map.insert("code".to_string(), ValueData::Int(code));
                map.insert("output".to_string(), ValueData::String(if stdout.is_empty() { stderr } else { stdout }));
                ok_obj(map)
            }
            Err(e) => {
                let mut map = LinkedHashMap::new();
                map.insert("success".to_string(), ValueData::Bool(false));
                map.insert("error".to_string(), ValueData::String(e));
                ok_obj(map)
            }
        }
    }

    pub fn driver_unload(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("modprobe", &format!("-r '{}'", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn driver_list_devices(_x: String) -> Box<dyn Validator> {
        let sysfs = std::fs::read_dir("/sys/bus").ok();
        let mut devices = vec![];

        if let Some(buses) = sysfs {
            for bus in buses.flatten() {
                let bus_name = bus.file_name().to_string_lossy().to_string();
                let devices_dir = bus.path().join("devices");
                if let Ok(entries) = std::fs::read_dir(&devices_dir) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let mut map = LinkedHashMap::new();
                        map.insert("bus".to_string(), ValueData::String(bus_name.clone()));
                        map.insert("device".to_string(), ValueData::String(name));
                        // Try to read modalias or driver
                        let modalias = std::fs::read_to_string(entry.path().join("modalias")).unwrap_or_default();
                        if !modalias.is_empty() {
                            map.insert("modalias".to_string(), ValueData::String(modalias.trim().to_string()));
                        }
                        let driver_link = entry.path().join("driver");
                        if driver_link.exists() {
                            if let Ok(target) = std::fs::read_link(&driver_link) {
                                if let Some(driver_name) = target.file_name() {
                                    map.insert("driver".to_string(), ValueData::String(driver_name.to_string_lossy().to_string()));
                                }
                            }
                        }
                        devices.push(ValueData::Object(Object::from_map(map)));
                    }
                }
            }
        }

        ok_vec(devices)
    }

    pub fn driver_sysfs_read(x: String) -> Box<dyn Validator> {
        let path = arg(&split_args(&x, 1), 0);
        match std::fs::read_to_string(&path) {
            Ok(content) => ok_str(content.trim().to_string()),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn driver_sysfs_write(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let path = arg(&args, 0);
        let value = arg(&args, 1);
        match std::fs::write(&path, &value) {
            Ok(_) => ok_bool(true),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    // ── Kernel ────────────────────────────────────────────────────────────

    pub fn kernel_version(_x: String) -> Box<dyn Validator> {
        match exec_cmd("uname", "-r") {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn kernel_list_modules(_x: String) -> Box<dyn Validator> {
        let content = std::fs::read_to_string("/proc/modules").unwrap_or_default();
        let modules: Vec<ValueData> = content.lines().filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let mut map = LinkedHashMap::new();
                map.insert("name".to_string(), ValueData::String(parts[0].to_string()));
                map.insert("size".to_string(), ValueData::Int(parts[1].parse::<i32>().unwrap_or(0)));
                map.insert("used".to_string(), ValueData::Int(parts[2].parse::<i32>().unwrap_or(0)));
                Some(ValueData::Object(Object::from_map(map)))
            } else {
                None
            }
        }).collect();
        ok_vec(modules)
    }

    pub fn kernel_load_module(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let name = arg(&args, 0);
        let params = if args.len() > 1 && !args[1].is_empty() { args[1].clone() } else { String::new() };
        match exec_cmd_full("modprobe", &format!("'{}' {}", name, params)) {
            Ok((_code, stdout, stderr)) => ok_str(if stdout.is_empty() { stderr } else { stdout }),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn kernel_unload_module(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("modprobe", &format!("-r '{}'", name)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn kernel_module_info(x: String) -> Box<dyn Validator> {
        let name = arg(&split_args(&x, 1), 0);
        match exec_cmd("modinfo", &format!("'{}'", name)) {
            Ok(s) => {
                let mut map = LinkedHashMap::new();
                for line in s.lines() {
                    if let Some(idx) = line.find(':') {
                        let key = line[..idx].trim().to_string();
                        let val = line[idx+1..].trim().to_string();
                        map.insert(key, ValueData::String(val));
                    }
                }
                ok_obj(map)
            }
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn kernel_dmesg(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let lines = arg(&args, 0).parse::<usize>().unwrap_or(50);
        match exec_cmd("dmesg", &format!("--level=info,warn,err --kernel | tail -n {}", lines)) {
            Ok(s) => {
                let entries: Vec<ValueData> = s.lines().map(|l| ValueData::String(l.to_string())).collect();
                ok_vec(entries)
            }
            Err(_) => {
                // fallback without --level
                match exec_cmd("dmesg", &format!("| tail -n {}", lines)) {
                    Ok(s) => {
                        let entries: Vec<ValueData> = s.lines().map(|l| ValueData::String(l.to_string())).collect();
                        ok_vec(entries)
                    }
                    Err(e) => ok_str(format!("error: {}", e)),
                }
            }
        }
    }

    pub fn kernel_sysctl_get(x: String) -> Box<dyn Validator> {
        let key = arg(&split_args(&x, 1), 0);
        match std::fs::read_to_string(format!("/proc/sys/{}", key.replace('.', "/"))) {
            Ok(v) => ok_str(v.trim().to_string()),
            Err(_) => match exec_cmd("sysctl", &format!("-n '{}'", key)) {
                Ok(v) => ok_str(v.trim().to_string()),
                Err(e) => ok_str(format!("error: {}", e)),
            }
        }
    }

    pub fn kernel_sysctl_set(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let key = arg(&args, 0);
        let value = arg(&args, 1);
        match std::fs::write(format!("/proc/sys/{}", key.replace('.', "/")), &value) {
            Ok(_) => ok_bool(true),
            Err(_) => match exec_cmd("sysctl", &format!("-w '{}={}'", key, value)) {
                Ok(s) => ok_str(s),
                Err(e) => ok_str(format!("error: {}", e)),
            }
        }
    }

    // ── Bootloaders (GRUB) ────────────────────────────────────────────────

    pub fn bootloader_list_entries(_x: String) -> Box<dyn Validator> {
        let cfg_paths = vec![
            "/boot/grub/grub.cfg",
            "/boot/grub2/grub.cfg",
            "/boot/efi/EFI/grub/grub.cfg",
        ];

        let mut entries = vec![];
        for path in &cfg_paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                let mut in_entry = false;
                let mut current = LinkedHashMap::new();
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("menuentry '") || trimmed.starts_with("menuentry \"") {
                        if in_entry {
                            entries.push(ValueData::Object(Object::from_map(current.clone())));
                        }
                        in_entry = true;
                        current = LinkedHashMap::new();
                        let quote = if trimmed.contains("'") { '\'' } else { '"' };
                        if let Some(start) = trimmed.find(quote) {
                            if let Some(end) = trimmed[start+1..].find(quote) {
                                let title = &trimmed[start+1..start+1+end];
                                current.insert("title".to_string(), ValueData::String(title.to_string()));
                            }
                        }
                    } else if in_entry && trimmed.starts_with('}') {
                        entries.push(ValueData::Object(Object::from_map(current.clone())));
                        in_entry = false;
                    }
                }
                if !entries.is_empty() {
                    break;
                }
            }
        }

        if entries.is_empty() {
            // Fallback: try grubby --info=ALL
            if let Ok(out) = exec_cmd("grubby", "--info=ALL") {
                for block in out.split("\n\n") {
                    let mut map = LinkedHashMap::new();
                    for line in block.lines() {
                        if let Some(idx) = line.find('=') {
                            let key = line[..idx].trim().to_string();
                            let val = line[idx+1..].trim().trim_matches('"').to_string();
                            map.insert(key, ValueData::String(val));
                        }
                    }
                    if !map.is_empty() {
                        entries.push(ValueData::Object(Object::from_map(map)));
                    }
                }
            }
        }

        ok_vec(entries)
    }

    pub fn bootloader_set_default(x: String) -> Box<dyn Validator> {
        let entry = arg(&split_args(&x, 1), 0);
        // Try grub-set-default first, then grubby
        let result = exec_cmd("grub-set-default", &format!("'{}'", entry))
            .or_else(|_| exec_cmd("grubby", &format!("--set-default='{}'", entry)));
        match result {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn bootloader_get_default(_x: String) -> Box<dyn Validator> {
        let result = std::fs::read_to_string("/boot/grub/grubenv")
            .ok()
            .and_then(|s| s.lines().find(|l| l.starts_with("saved_entry=")).map(|l| l[12..].to_string()))
            .or_else(|| exec_cmd("grubby", "--default-kernel").ok())
            .unwrap_or_default();
        ok_str(result)
    }

    pub fn bootloader_install(x: String) -> Box<dyn Validator> {
        let device = arg(&split_args(&x, 1), 0);
        if device.is_empty() {
            return ok_str("error: device required (e.g. /dev/sda)".to_string());
        }
        match exec_cmd("grub-install", &format!("'{}'", device)) {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }

    pub fn bootloader_update(_x: String) -> Box<dyn Validator> {
        let result = exec_cmd("grub-mkconfig", "-o /boot/grub/grub.cfg")
            .or_else(|_| exec_cmd("grub2-mkconfig", "-o /boot/grub2/grub.cfg"))
            .or_else(|_| exec_cmd("update-grub", ""));
        match result {
            Ok(s) => ok_str(s),
            Err(e) => ok_str(format!("error: {}", e)),
        }
    }
}

pub use system_lib::*;
