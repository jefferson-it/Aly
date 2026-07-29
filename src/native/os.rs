mod os_lib {
    use std::collections::HashMap;

    use linked_hash_map::LinkedHashMap;

    use crate::native::create_object::Object;
    use crate::native::std::split_args;
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::{put_quoted_str, remove_quoted_str};

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str(s))
    }

    fn ok_int(i: i32) -> Box<dyn Validator> {
        Box::new(ValueData::Int(i))
    }

    #[allow(dead_code)]
    fn ok_bool(b: bool) -> Box<dyn Validator> {
        Box::new(ValueData::Bool(b))
    }

    fn ok_vec(v: Vec<ValueData>) -> Box<dyn Validator> {
        Box::new(ValueData::Vec(Vector::new(v)))
    }

    fn ok_obj(map: LinkedHashMap<String, ValueData>) -> Box<dyn Validator> {
        Box::new(ValueData::Object(Object::from_map(map)))
    }

    fn safe_i32(val: i64) -> i32 {
        i32::try_from(val).unwrap_or(i32::MAX)
    }

    pub fn os_platform(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::OS.to_string())
    }

    pub fn os_arch(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::ARCH.to_string())
    }

    pub fn os_type(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::FAMILY.to_string())
    }

    pub fn os_release(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let mut uts = unsafe { std::mem::zeroed::<libc::utsname>() };
            if unsafe { libc::uname(&mut uts) } == 0 {
                let release = unsafe { std::ffi::CStr::from_ptr(uts.release.as_ptr()) }
                    .to_string_lossy()
                    .to_string();
                return ok_str(release);
            }
        }
        let release = std::process::Command::new("uname")
            .arg("-r")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout).ok()
                } else {
                    None
                }
            })
            .unwrap_or_default()
            .trim()
            .to_string();
        ok_str(release)
    }

    pub fn os_hostname(_x: String) -> Box<dyn Validator> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let mut buf = vec![0u8; 256];
            if unsafe {
                libc::gethostname(
                    buf.as_mut_ptr() as *mut std::ffi::c_char,
                    buf.len(),
                )
            } == 0
            {
                let hostname =
                    unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const std::ffi::c_char) }
                        .to_string_lossy()
                        .to_string();
                return ok_str(hostname);
            }
        }
        let hostname = std::process::Command::new("hostname")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout).ok()
                } else {
                    None
                }
            })
            .unwrap_or_default()
            .trim()
            .to_string();
        ok_str(hostname)
    }

    pub fn os_homedir(_x: String) -> Box<dyn Validator> {
        let home = dirs::home_dir()
            .map(|p| p.display().to_string())
            .or_else(|| std::env::var("HOME").ok())
            .or_else(|| std::env::var("USERPROFILE").ok())
            .unwrap_or_default();
        ok_str(home)
    }

    pub fn os_tmpdir(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::temp_dir().display().to_string())
    }

    pub fn os_pid(_x: String) -> Box<dyn Validator> {
        ok_int(std::process::id() as i32)
    }

    pub fn os_ppid(_x: String) -> Box<dyn Validator> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let ppid = unsafe { libc::getppid() };
            return ok_int(ppid);
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        ok_int(-1)
    }

    pub fn os_endianness(_x: String) -> Box<dyn Validator> {
        if cfg!(target_endian = "little") {
            ok_str("LE".to_string())
        } else {
            ok_str("BE".to_string())
        }
    }

    pub fn os_eol(_x: String) -> Box<dyn Validator> {
        if cfg!(target_os = "windows") {
            ok_str("\r\n".to_string())
        } else {
            ok_str("\n".to_string())
        }
    }

    pub fn os_uptime(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let mut info = unsafe { std::mem::zeroed::<libc::sysinfo>() };
            if unsafe { libc::sysinfo(&mut info) } == 0 {
                return ok_int(safe_i32(info.uptime as i64));
            }
        }
        let uptime_str = std::fs::read_to_string("/proc/uptime").unwrap_or_default();
        let seconds = uptime_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|s| safe_i32(s as i64))
            .unwrap_or(0);
        ok_int(seconds)
    }

    pub fn os_totalmem(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let mut info = unsafe { std::mem::zeroed::<libc::sysinfo>() };
            if unsafe { libc::sysinfo(&mut info) } == 0 {
                let total: i64 = (info.totalram as i64) * (info.mem_unit as i64);
                return ok_int(safe_i32(total));
            }
        }
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                let kb: i64 = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                return ok_int(safe_i32(kb * 1024));
            }
        }
        ok_int(0)
    }

    pub fn os_freemem(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let mut info = unsafe { std::mem::zeroed::<libc::sysinfo>() };
            if unsafe { libc::sysinfo(&mut info) } == 0 {
                let free: i64 = (info.freeram as i64) * (info.mem_unit as i64);
                return ok_int(safe_i32(free));
            }
        }
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        for line in meminfo.lines() {
            if line.starts_with("MemAvailable:") {
                let kb: i64 = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                return ok_int(safe_i32(kb * 1024));
            }
        }
        ok_int(0)
    }

    pub fn os_usedmem(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let mut info = unsafe { std::mem::zeroed::<libc::sysinfo>() };
            if unsafe { libc::sysinfo(&mut info) } == 0 {
                let total: i64 = (info.totalram as i64) * (info.mem_unit as i64);
                let free: i64 = (info.freeram as i64) * (info.mem_unit as i64);
                return ok_int(safe_i32(total - free));
            }
        }
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        let mut total_kb = 0i64;
        let mut free_kb = 0i64;
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if line.starts_with("MemAvailable:") {
                free_kb = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            }
        }
        ok_int(safe_i32((total_kb - free_kb) * 1024))
    }

    pub fn os_loadavg(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let mut info = unsafe { std::mem::zeroed::<libc::sysinfo>() };
            if unsafe { libc::sysinfo(&mut info) } == 0 {
                return ok_vec(vec![
                    ValueData::Float((info.loads[0] as f32) / 65536.0),
                    ValueData::Float((info.loads[1] as f32) / 65536.0),
                    ValueData::Float((info.loads[2] as f32) / 65536.0),
                ]);
            }
        }
        let load_str = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
        let parts: Vec<&str> = load_str.split_whitespace().collect();
        if parts.len() >= 3 {
            return ok_vec(vec![
                ValueData::Float(parts[0].parse::<f32>().unwrap_or(0.0)),
                ValueData::Float(parts[1].parse::<f32>().unwrap_or(0.0)),
                ValueData::Float(parts[2].parse::<f32>().unwrap_or(0.0)),
            ]);
        }
        ok_vec(vec![
            ValueData::Float(0.0),
            ValueData::Float(0.0),
            ValueData::Float(0.0),
        ])
    }

    pub fn os_cpus(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
            let stat = std::fs::read_to_string("/proc/stat").unwrap_or_default();

            let mut cpu_times: HashMap<String, Vec<i64>> = HashMap::new();
            for line in stat.lines() {
                if line.starts_with("cpu") && !line.starts_with("cpu ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 {
                        let name = parts[0].to_string();
                        let times: Vec<i64> =
                            parts[1..5].iter().filter_map(|s| s.parse().ok()).collect();
                        cpu_times.insert(name, times);
                    }
                }
            }

            let mut processors: Vec<ValueData> = vec![];
            let mut current_model = String::new();
            let mut current_speed = 0.0f32;
            let mut current_processor = -1i32;

            for line in cpuinfo.lines() {
                if let Some(idx) = line.find(':') {
                    let key = line[..idx].trim();
                    let val = line[idx + 1..].trim();
                    match key {
                        "processor" => {
                            if current_processor >= 0 {
                                let cpu_name = format!("cpu{}", current_processor);
                                let times =
                                    cpu_times.get(&cpu_name).cloned().unwrap_or_default();
                                let mut times_map = LinkedHashMap::new();
                                if times.len() >= 4 {
                                    times_map
                                        .insert("user".to_string(), ValueData::Int(safe_i32(times[0])));
                                    times_map
                                        .insert("nice".to_string(), ValueData::Int(safe_i32(times[1])));
                                    times_map
                                        .insert("sys".to_string(), ValueData::Int(safe_i32(times[2])));
                                    times_map
                                        .insert("idle".to_string(), ValueData::Int(safe_i32(times[3])));
                                }

                                let mut map = LinkedHashMap::new();
                                map.insert(
                                    "model".to_string(),
                                    ValueData::String(current_model.clone()),
                                );
                                map.insert(
                                    "speed".to_string(),
                                    ValueData::Float(current_speed),
                                );
                                map.insert(
                                    "times".to_string(),
                                    ValueData::Object(Object::from_map(times_map)),
                                );
                                processors.push(ValueData::Object(Object::from_map(map)));
                            }
                            current_processor = val.parse().unwrap_or(-1);
                            current_model = String::new();
                            current_speed = 0.0;
                        }
                        "model name" | "cpu" | "Processor" => {
                            current_model = val.to_string();
                        }
                        "cpu MHz" | "BogoMIPS" => {
                            if let Ok(speed) = val.parse::<f32>() {
                                current_speed = speed;
                            }
                        }
                        _ => {}
                    }
                }
            }

            if current_processor >= 0 {
                let cpu_name = format!("cpu{}", current_processor);
                let times = cpu_times.get(&cpu_name).cloned().unwrap_or_default();
                let mut times_map = LinkedHashMap::new();
                if times.len() >= 4 {
                    times_map
                        .insert("user".to_string(), ValueData::Int(safe_i32(times[0])));
                    times_map
                        .insert("nice".to_string(), ValueData::Int(safe_i32(times[1])));
                    times_map
                        .insert("sys".to_string(), ValueData::Int(safe_i32(times[2])));
                    times_map
                        .insert("idle".to_string(), ValueData::Int(safe_i32(times[3])));
                }

                let mut map = LinkedHashMap::new();
                map.insert(
                    "model".to_string(),
                    ValueData::String(current_model),
                );
                map.insert(
                    "speed".to_string(),
                    ValueData::Float(current_speed),
                );
                map.insert(
                    "times".to_string(),
                    ValueData::Object(Object::from_map(times_map)),
                );
                processors.push(ValueData::Object(Object::from_map(map)));
            }

            return ok_vec(processors);
        }

        #[allow(unreachable_code)]
        ok_vec(vec![])
    }

    pub fn os_network_interfaces(_x: String) -> Box<dyn Validator> {
        #[cfg(target_os = "linux")]
        {
            let net_dev = std::fs::read_to_string("/proc/net/dev").unwrap_or_default();
            let mut interfaces = vec![];

            for line in net_dev.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 10 {
                    continue;
                }

                let name = parts[0].trim_end_matches(':');
                if name == "lo" {
                    continue;
                }

                let mac_path = format!("/sys/class/net/{}/address", name);
                let mac = std::fs::read_to_string(&mac_path)
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                let mut map = LinkedHashMap::new();
                map.insert("name".to_string(), ValueData::String(name.to_string()));
                map.insert("mac".to_string(), ValueData::String(mac));
                map.insert(
                    "rx_bytes".to_string(),
                    ValueData::Int(safe_i32(parts[1].parse::<i64>().unwrap_or(0))),
                );
                map.insert(
                    "tx_bytes".to_string(),
                    ValueData::Int(safe_i32(parts[9].parse::<i64>().unwrap_or(0))),
                );

                interfaces.push(ValueData::Object(Object::from_map(map)));
            }

            return ok_vec(interfaces);
        }

        #[allow(unreachable_code)]
        ok_vec(vec![])
    }

    pub fn os_userinfo(_x: String) -> Box<dyn Validator> {
        let mut map = LinkedHashMap::new();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let uid = unsafe { libc::getuid() };
            let gid = unsafe { libc::getgid() };
            map.insert("uid".to_string(), ValueData::Int(uid as i32));
            map.insert("gid".to_string(), ValueData::Int(gid as i32));

            let mut pwd = unsafe { std::mem::zeroed::<libc::passwd>() };
            let mut buf = vec![0u8; 4096];
            let mut result: *mut libc::passwd = std::ptr::null_mut();
            if unsafe {
                libc::getpwuid_r(
                    uid,
                    &mut pwd,
                    buf.as_mut_ptr() as *mut std::ffi::c_char,
                    buf.len(),
                    &mut result,
                )
            } == 0
                && !result.is_null()
            {
                let username = unsafe { std::ffi::CStr::from_ptr(pwd.pw_name) }
                    .to_string_lossy()
                    .to_string();
                let shell = unsafe { std::ffi::CStr::from_ptr(pwd.pw_shell) }
                    .to_string_lossy()
                    .to_string();
                let dir = unsafe { std::ffi::CStr::from_ptr(pwd.pw_dir) }
                    .to_string_lossy()
                    .to_string();
                map.insert("username".to_string(), ValueData::String(username));
                map.insert("shell".to_string(), ValueData::String(shell));
                map.insert("homedir".to_string(), ValueData::String(dir));
            }
        }

        if !map.contains_key("username") {
            let user = std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .or_else(|_| std::env::var("LOGNAME"))
                .unwrap_or_default();
            map.insert("username".to_string(), ValueData::String(user));
        }

        if !map.contains_key("homedir") {
            let home = dirs::home_dir()
                .map(|p| p.display().to_string())
                .or_else(|| std::env::var("HOME").ok())
                .or_else(|| std::env::var("USERPROFILE").ok())
                .unwrap_or_default();
            map.insert("homedir".to_string(), ValueData::String(home));
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            map.insert("uid".to_string(), ValueData::Int(-1));
            map.insert("gid".to_string(), ValueData::Int(-1));
            map.insert("shell".to_string(), ValueData::String(String::new()));
        }

        ok_obj(map)
    }

    pub fn os_syscall(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 4);
        let num = args
            .get(0)
            .map(|a| remove_quoted_str(a.clone()).parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        let a1 = args
            .get(1)
            .map(|a| remove_quoted_str(a.clone()).parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        let a2 = args
            .get(2)
            .map(|a| remove_quoted_str(a.clone()).parse::<i64>().unwrap_or(0))
            .unwrap_or(0);
        let a3 = args
            .get(3)
            .map(|a| remove_quoted_str(a.clone()).parse::<i64>().unwrap_or(0))
            .unwrap_or(0);

        #[cfg(target_os = "linux")]
        let result = unsafe { libc::syscall(num, a1, a2, a3) as i64 };

        #[cfg(not(target_os = "linux"))]
        let result = -1i64;

        ok_int(safe_i32(result))
    }

    pub fn os_exec(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let cmd = arg_to_string(&args, 0);

        let output = match std::process::Command::new("sh")
            .args(["-c", &cmd])
            .output()
        {
            Ok(o) => o,
            Err(_) => return ok_str(String::new()),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if !stderr.is_empty() && stdout.is_empty() {
            return ok_str(stderr);
        }

        ok_str(stdout)
    }

    fn arg_to_string(args: &[String], n: usize) -> String {
        args.get(n)
            .map(|a| remove_quoted_str(a.clone()))
            .unwrap_or_default()
    }
}

pub use os_lib::*;
