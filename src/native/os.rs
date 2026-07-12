mod os_lib {
    use std::collections::HashMap;

    use crate::native::create_object::Object;
    use crate::native::std::split_args;
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::{put_quoted_str, remove_quoted_str};

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str(s))
    }

    fn ok_int(i: i64) -> Box<dyn Validator> {
        Box::new(ValueData::Int(i))
    }

    // os.platform() -> string
    pub fn os_platform(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::OS.to_string())
    }

    // os.arch() -> string
    pub fn os_arch(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::ARCH.to_string())
    }

    // os.type() -> string
    pub fn os_type(_x: String) -> Box<dyn Validator> {
        ok_str(std::env::consts::FAMILY.to_string())
    }

    // os.release() -> string
    pub fn os_release(_x: String) -> Box<dyn Validator> {
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

    // os.hostname() -> string
    pub fn os_hostname(_x: String) -> Box<dyn Validator> {
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

    // os.homedir() -> string
    pub fn os_homedir(_x: String) -> Box<dyn Validator> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_default();
        ok_str(home)
    }

    // os.uptime() -> int (seconds)
    pub fn os_uptime(_x: String) -> Box<dyn Validator> {
        // System uptime from /proc/uptime
        let uptime_str = std::fs::read_to_string("/proc/uptime").unwrap_or_default();
        let seconds = uptime_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|s| s as i64)
            .unwrap_or(0);
        ok_int(seconds)
    }

    // os.totalmem() -> int (bytes)
    pub fn os_totalmem(_x: String) -> Box<dyn Validator> {
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                let kb: i64 = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                return ok_int(kb * 1024);
            }
        }
        ok_int(0)
    }

    // os.freemem() -> int (bytes)
    pub fn os_freemem(_x: String) -> Box<dyn Validator> {
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        for line in meminfo.lines() {
            if line.starts_with("MemAvailable:") {
                let kb: i64 = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                return ok_int(kb * 1024);
            }
        }
        ok_int(0)
    }

    // os.cpus() -> vector of objects
    pub fn os_cpus(_x: String) -> Box<dyn Validator> {
        let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        let mut cpus = vec![];
        let mut current: Option<Object> = None;

        for line in cpuinfo.lines() {
            if line.is_empty() {
                if let Some(obj) = current.take() {
                    cpus.push(ValueData::Object(obj));
                }
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let value = line[pos + 1..].trim();
                if current.is_none() {
                    current = Some(Object::new(vec![], HashMap::new()));
                }
                if let Some(ref mut obj) = current {
                    match key {
                        "model name" => {
                            obj.set_item(
                                "model".to_owned(),
                                ValueData::String(value.to_string()),
                            );
                        }
                        "cpu MHz" => {
                            if let Ok(mhz) = value.parse::<f64>() {
                                obj.set_item(
                                    "speed".to_owned(),
                                    ValueData::Int((mhz * 1_000_000.0) as i64),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Box::new(ValueData::Vec(Vector::new(cpus)))
    }

    // os.syscall(number, arg1, arg2, arg3) -> int
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

        ok_int(result)
    }

    // os.exec(command) -> string (stdout)
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
