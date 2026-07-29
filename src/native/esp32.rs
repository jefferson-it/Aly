// ESP32 Support for Aly
// Provides ESP32 flashing, monitoring, OTA updates, and configuration

#[cfg(feature = "iot-esp32")]
mod esp32_mod {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::cell::RefCell;

    use crate::native::{
        std::{arg, split_args},
        types::Validator,
    };
    use crate::validators::str::put_quoted_str;

    thread_local! {
        static ESP32_MONITORS: RefCell<HashMap<String, thread::JoinHandle<()>>> = RefCell::new(HashMap::new());
        static ESP32_CALLBACKS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }

    fn find_esptool() -> Result<String, String> {
        // Try to find esptool.py in PATH
        if let Ok(output) = Command::new("which").arg("esptool.py").output() {
            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        if let Ok(output) = Command::new("which").arg("esptool").output() {
            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        Err("esptool not found in PATH. Install with: pip install esptool".to_string())
    }

    pub fn esp32_flash(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 5);
        let port = arg(&args, 0);
        let baud_rate: u32 = arg(&args, 1).parse().unwrap_or(460800);
        let bootloader = arg(&args, 2);
        let partitions = arg(&args, 3);
        let app = arg(&args, 4);

        let esptool = match find_esptool() {
            Ok(e) => e,
            Err(e) => return Box::new(put_quoted_str(e)),
        };

        let mut cmd = Command::new(&esptool);
        cmd.arg("--port").arg(&port)
            .arg("--baud").arg(&baud_rate.to_string())
            .arg("write_flash")
            .arg("0x1000").arg(&bootloader)
            .arg("0x8000").arg(&partitions)
            .arg("0x10000").arg(&app);

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

    pub fn esp32_erase(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let port = arg(&args, 0);

        let esptool = match find_esptool() {
            Ok(e) => e,
            Err(e) => return Box::new(put_quoted_str(e)),
        };

        let mut cmd = Command::new(&esptool);
        cmd.arg("--port").arg(&port).arg("erase_flash");

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

    pub fn esp32_monitor(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let port = arg(&args, 0);
        let baud_rate: u32 = arg(&args, 1).parse().unwrap_or(115200);
        let callback = arg(&args, 2);

        // Store callback for received data
        ESP32_CALLBACKS.with(|cb| {
            cb.borrow_mut().insert(port.clone(), callback);
        });

        let port_clone = port.clone();
        let handle = thread::spawn(move || {
            // Use screen or minicom or picocom for monitoring
            let mut cmd = Command::new("screen");
            cmd.arg(&port_clone).arg(&baud_rate.to_string());

            // Alternative: use miniterm.py (comes with esptool)
            let _ = cmd.status();
        });

        ESP32_MONITORS.with(|m| {
            m.borrow_mut().insert(port, handle);
        });

        Box::new(put_quoted_str("OK"))
    }

    pub fn esp32_monitor_stop(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let port = arg(&args, 0);

        ESP32_MONITORS.with(|m| {
            if let Some(handle) = m.borrow_mut().remove(&port) {
                let _ = handle.join();
            }
        });
        ESP32_CALLBACKS.with(|cb| {
            cb.borrow_mut().remove(&port);
        });

        Box::new(put_quoted_str("OK"))
    }

    pub fn esp32_ota(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let ip = arg(&args, 0);
        let port = arg(&args, 1);
        let firmware = arg(&args, 2);

        // Use espota.py for OTA updates
        let mut cmd = Command::new("espota.py");
        cmd.arg("-i").arg(&ip)
            .arg("-p").arg(&port)
            .arg("-f").arg(&firmware);

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

    pub fn esp32_wifi_config(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 4);
        let port = arg(&args, 0);
        let ssid = arg(&args, 1);
        let password = arg(&args, 2);
        let static_ip = arg(&args, 3);

        // Send WiFi config via serial
        let mut cmd = Command::new("python3");
        cmd.arg("-c")
            .arg(format!(
                "import serial; s=serial.Serial('{}', 115200, timeout=2); s.write(b'WIFI:{}:{}:{}'); print(s.read(100).decode())",
                port, ssid, password, static_ip
            ));

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

    pub fn esp32_ble_scan(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let port = arg(&args, 0);
        let duration: u64 = arg(&args, 1).parse().unwrap_or(5);

        // Send BLE scan command via serial
        let mut cmd = Command::new("python3");
        cmd.arg("-c")
            .arg(format!(
                "import serial, time; s=serial.Serial('{}', 115200, timeout=2); s.write(b'BLE_SCAN:{{}}'); time.sleep(1); print(s.read(500).decode())",
                port, duration
            ));

        match cmd.output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                Box::new(put_quoted_str(result.to_string()))
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn esp32_info(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let port = arg(&args, 0);

        let mut cmd = Command::new("python3");
        cmd.arg("-c")
            .arg(format!(
                "import serial; s=serial.Serial('{}', 115200, timeout=2); s.write(b'INFO'); time.sleep(0.5); print(s.read(200).decode())",
                port
            ));

        match cmd.output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout);
                Box::new(put_quoted_str(result.to_string()))
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }
}

#[cfg(not(feature = "iot-esp32"))]
mod esp32_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn esp32_flash(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_erase(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_monitor(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_monitor_stop(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_ota(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_wifi_config(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_ble_scan(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
    pub fn esp32_info(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-esp32' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "iot-esp32"))]
pub use esp32_stub::*;
