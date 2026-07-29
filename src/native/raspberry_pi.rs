// Raspberry Pi Support for Aly
// High-level Raspberry Pi features: Camera, Display, Network, System, PWM, etc.

#[cfg(feature = "iot-raspberry-pi")]
mod rpi_mod {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::fs;

    use crate::native::{
        std::{arg, split_args},
        types::Validator,
    };
    use crate::validators::str::put_quoted_str;

    // Re-export gpio functions if gpio feature is enabled
    #[cfg(any(feature = "rpi-gpio", feature = "iot-raspberry-pi"))]
    use crate::native::gpio;

    thread_local! {
        static RPI_CAMERAS: RefCell<HashMap<String, thread::JoinHandle<()>>> = RefCell::new(HashMap::new());
        static RPI_CALLBACKS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }

    // Camera module (libcamera / raspicam)
    pub fn rpi_camera_capture(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 4);
        let output = arg(&args, 0);
        let width: u32 = arg(&args, 1).parse().unwrap_or(1920);
        let height: u32 = arg(&args, 2).parse().unwrap_or(1080);
        let format = arg(&args, 3).to_lowercase();

        let mut cmd = Command::new("libcamera-jpeg");
        cmd.arg("-o").arg(&output)
            .arg("--width").arg(&width.to_string())
            .arg("--height").arg(&height.to_string())
            .arg("--timeout").arg("1000")
            .arg("--nopreview");

        if format == "png" {
            cmd.arg("--encoding").arg("png");
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Box::new(put_quoted_str("OK"))
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Box::new(put_quoted_str(format!("Erro: {}", stderr)))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn rpi_camera_record(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 5);
        let output = arg(&args, 0);
        let width: u32 = arg(&args, 1).parse().unwrap_or(1920);
        let height: u32 = arg(&args, 2).parse().unwrap_or(1080);
        let duration: u32 = arg(&args, 3).parse().unwrap_or(10);
        let fps: u32 = arg(&args, 4).parse().unwrap_or(30);

        let mut cmd = Command::new("libcamera-vid");
        cmd.arg("-o").arg(&output)
            .arg("--width").arg(&width.to_string())
            .arg("--height").arg(&height.to_string())
            .arg("--framerate").arg(&fps.to_string())
            .arg("--timeout").arg(&(duration * 1000).to_string())
            .arg("--nopreview");

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Box::new(put_quoted_str("OK"))
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Box::new(put_quoted_str(format!("Erro: {}", stderr)))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn rpi_camera_start_stream(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 4);
        let name = arg(&args, 0);
        let width: u32 = arg(&args, 1).parse().unwrap_or(1280);
        let height: u32 = arg(&args, 2).parse().unwrap_or(720);
        let port: u16 = arg(&args, 3).parse().unwrap_or(8080);

        let handle = thread::spawn(move || {
            let mut cmd = Command::new("libcamera-vid");
            cmd.arg("-o").arg("-")
                .arg("--width").arg(&width.to_string())
                .arg("--height").arg(&height.to_string())
                .arg("--framerate").arg("30")
                .arg("--codec").arg("h264")
                .arg("--inline")
                .arg("--listen")
                .arg("-t").arg("0")
                .arg("--port").arg(&port.to_string())
                .arg("--nopreview");

            let _ = cmd.output();
        });

        RPI_CAMERAS.with(|c| c.borrow_mut().insert(name, handle));
        Box::new(put_quoted_str("OK"))
    }

    pub fn rpi_camera_stop_stream(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        RPI_CAMERAS.with(|c| {
            let mut cams = c.borrow_mut();
            if let Some(handle) = cams.remove(&name) {
                let _ = handle.join();
            }
        });
        Box::new(put_quoted_str("OK"))
    }

    // Display/HDMI
    pub fn rpi_display_info(_params: String) -> Box<dyn Validator> {
        let output = Command::new("tvservice").arg("-s").output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    let info = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    Box::new(put_quoted_str(info))
                } else {
                    Box::new(put_quoted_str("Erro: não foi possível obter info do display".to_string()))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn rpi_display_set_mode(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let width: u32 = arg(&args, 0).parse().unwrap_or(1920);
        let height: u32 = arg(&args, 1).parse().unwrap_or(1080);
        let refresh: u32 = arg(&args, 2).parse().unwrap_or(60);

        let mode = format!("{}x{}@{}", width, height, refresh);
        let output = Command::new("tvservice").arg("-e").arg(&mode).output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    Box::new(put_quoted_str("OK"))
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    Box::new(put_quoted_str(format!("Erro: {}", stderr)))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn rpi_display_off(_params: String) -> Box<dyn Validator> {
        let output = Command::new("tvservice").arg("-o").output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    Box::new(put_quoted_str("OK"))
                } else {
                    Box::new(put_quoted_str("Erro".to_string()))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    // Network configuration
    pub fn rpi_wifi_scan(_params: String) -> Box<dyn Validator> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "SSID,SIGNAL,SECURITY", "dev", "wifi"])
            .output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    Box::new(put_quoted_str(result))
                } else {
                    Box::new(put_quoted_str("Erro ao escanear WiFi".to_string()))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn rpi_wifi_connect(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let ssid = arg(&args, 0);
        let password = arg(&args, 1);

        let output = Command::new("nmcli")
            .args(["dev", "wifi", "connect", &ssid, "password", &password])
            .output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    Box::new(put_quoted_str("OK"))
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    Box::new(put_quoted_str(format!("Erro: {}", stderr)))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn rpi_wifi_ap(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let ssid = arg(&args, 0);
        let password = arg(&args, 1);
        let channel: u8 = arg(&args, 2).parse().unwrap_or(6);

        // Use create_ap or hostapd
        let output = Command::new("create_ap")
            .args(["--no-virt", "--channel", &channel.to_string(), "wlan0", &ssid, &password])
            .output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    Box::new(put_quoted_str("OK"))
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    Box::new(put_quoted_str(format!("Erro: {}", stderr)))
                }
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    // System info
    pub fn rpi_system_info(_params: String) -> Box<dyn Validator> {
        let mut info = String::new();

        // CPU temp
        if let Ok(temp_str) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
            if let Ok(temp) = temp_str.trim().parse::<f32>() {
                info.push_str(&format!("cpu_temp_celsius:{};", temp / 1000.0));
            }
        }

        // Memory
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    let kb: u64 = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                    info.push_str(&format!("mem_total_mb:{};", kb / 1024));
                }
                if line.starts_with("MemAvailable:") {
                    let kb: u64 = line.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
                    info.push_str(&format!("mem_available_mb:{};", kb / 1024));
                }
            }
        }

        // CPU
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let model = cpuinfo.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("").trim())
                .unwrap_or("Unknown");
            info.push_str(&format!("cpu_model:{};", model));
        }

        // OS
        if let Ok(os) = fs::read_to_string("/etc/os-release") {
            let pretty = os.lines()
                .find(|l| l.starts_with("PRETTY_NAME"))
                .map(|l| l.split('=').nth(1).unwrap_or("").trim_matches('"'))
                .unwrap_or("Unknown");
            info.push_str(&format!("os:{};", pretty));
        }

        // Hostname
        if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
            info.push_str(&format!("hostname:{};", hostname.trim()));
        }

        // Uptime
        if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
            let secs: f64 = uptime_str.split_whitespace().next().unwrap_or("0").parse().unwrap_or(0.0);
            let days = (secs / 86400.0) as u64;
            let hours = ((secs % 86400.0) / 3600.0) as u64;
            info.push_str(&format!("uptime:{}d{}h;", days, hours));
        }

        // Disk
        if let Ok(output) = Command::new("df").args(["-h", "/"]).output() {
            if output.status.success() {
                let df = String::from_utf8_lossy(&output.stdout);
                for line in df.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 {
                        info.push_str(&format!("disk_total:{};disk_used:{};disk_free:{};disk_use:{};",
                            parts[1], parts[2], parts[3], parts[4]));
                    }
                }
            }
        }

        Box::new(put_quoted_str(info))
    }

    // GPIO via rppal (already in gpio.rs, but adding higher-level functions)
    pub fn rpi_gpio_setup(params: String) -> Box<dyn Validator> {
        // Delegate to gpio.rs functions
        crate::native::gpio::gpio_setup(params)
    }

    pub fn rpi_gpio_write(params: String) -> Box<dyn Validator> {
        crate::native::gpio::gpio_write(params)
    }

    pub fn rpi_gpio_read(params: String) -> Box<dyn Validator> {
        crate::native::gpio::gpio_read(params)
    }

    pub fn rpi_gpio_pwm(params: String) -> Box<dyn Validator> {
        crate::native::gpio::gpio_pwm(params)
    }

    // PWM via sysfs
    pub fn rpi_pwm_setup(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let chip: u32 = arg(&args, 0).parse().unwrap_or(0);
        let channel: u32 = arg(&args, 1).parse().unwrap_or(0);
        let frequency: u32 = arg(&args, 2).parse().unwrap_or(1000);

        // Export PWM channel
        let export_path = format!("/sys/class/pwm/pwmchip{}/export", chip);
        if fs::write(&export_path, channel.to_string()).is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível exportar PWM".to_string()));
        }

        // Set period
        let period_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/period", chip, channel);
        let period_ns = 1_000_000_000 / frequency as u64;
        if fs::write(&period_path, period_ns.to_string()).is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível configurar período".to_string()));
        }

        Box::new(put_quoted_str("OK"))
    }

    pub fn rpi_pwm_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 4);
        let chip: u32 = arg(&args, 0).parse().unwrap_or(0);
        let channel: u32 = arg(&args, 1).parse().unwrap_or(0);
        let duty_cycle_ns: u64 = arg(&args, 2).parse().unwrap_or(0);
        let enable: u8 = arg(&args, 3).parse().unwrap_or(1);

        let duty_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/duty_cycle", chip, channel);
        if fs::write(&duty_path, duty_cycle_ns.to_string()).is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível definir duty cycle".to_string()));
        }

        let enable_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/enable", chip, channel);
        fs::write(&enable_path, enable.to_string()).ok();

        Box::new(put_quoted_str("OK"))
    }

    pub fn rpi_pwm_disable(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let chip: u32 = arg(&args, 0).parse().unwrap_or(0);
        let channel: u32 = arg(&args, 1).parse().unwrap_or(0);

        let enable_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/enable", chip, channel);
        fs::write(&enable_path, "0").ok();

        Box::new(put_quoted_str("OK"))
    }

    // Watchdog
    pub fn rpi_watchdog_enable(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let timeout: u32 = arg(&args, 0).parse().unwrap_or(10);

        // Write to watchdog
        if fs::write("/dev/watchdog", "V").is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível habilitar watchdog".to_string()));
        }

        // Set timeout via /proc/sys/kernel/watchdog_thresh
        if fs::write("/proc/sys/kernel/watchdog_thresh", timeout.to_string()).is_err() {
            // Non-critical
        }

        Box::new(put_quoted_str("OK"))
    }

    pub fn rpi_watchdog_disable(_params: String) -> Box<dyn Validator> {
        // Write 'V' to disable (magic character)
        fs::write("/dev/watchdog", "V").ok();
        Box::new(put_quoted_str("OK"))
    }

    pub fn rpi_watchdog_keepalive(_params: String) -> Box<dyn Validator> {
        // Write any character to keep watchdog alive
        fs::write("/dev/watchdog", ".").ok();
        Box::new(put_quoted_str("OK"))
    }
}

#[cfg(not(feature = "iot-raspberry-pi"))]
mod rpi_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    macro_rules! stub_fn {
        ($name:ident) => {
            pub fn $name(_: String) -> Box<dyn Validator> {
                Box::new(put_quoted_str("Erro: feature 'iot-raspberry-pi' não habilitada".to_string()))
            }
        }
    }

    stub_fn!(rpi_camera_capture);
    stub_fn!(rpi_camera_record);
    stub_fn!(rpi_camera_start_stream);
    stub_fn!(rpi_camera_stop_stream);
    stub_fn!(rpi_display_info);
    stub_fn!(rpi_display_set_mode);
    stub_fn!(rpi_display_off);
    stub_fn!(rpi_wifi_scan);
    stub_fn!(rpi_wifi_connect);
    stub_fn!(rpi_wifi_ap);
    stub_fn!(rpi_system_info);
    stub_fn!(rpi_gpio_setup);
    stub_fn!(rpi_gpio_write);
    stub_fn!(rpi_gpio_read);
    stub_fn!(rpi_gpio_pwm);
    stub_fn!(rpi_pwm_setup);
    stub_fn!(rpi_pwm_write);
    stub_fn!(rpi_pwm_disable);
    stub_fn!(rpi_watchdog_enable);
    stub_fn!(rpi_watchdog_disable);
    stub_fn!(rpi_watchdog_keepalive);
}

#[cfg(not(feature = "iot-raspberry-pi"))]
pub use rpi_stub::*;