// Arduino Integration for Aly
// Provides communication with Arduino boards via serial/USB

#[cfg(feature = "iot-arduino")]
mod arduino_mod {
    use serialport::{SerialPort, SerialPortInfo, SerialPortType};
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::thread;
    use std::time::Duration;

    use crate::native::std::{arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    thread_local! {
        static ARDUINO_PORTS: RefCell<HashMap<String, Box<dyn std::io::Read + std::io::Write + Send>>> = RefCell::new(HashMap::new());
        static ARDUINO_CALLBACKS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }

    pub fn arduino_list(_params: String) -> Box<dyn Validator> {
        let ports = serialport::available_ports().unwrap_or_default();
        let mut result = String::new();
        for (i, port) in ports.iter().enumerate() {
            if i > 0 {
                result.push(',');
            }
            let port_name = match &port.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    format!("{} ({}:{})", port.port_name, info.vid, info.pid)
                }
                serialport::SerialPortType::BluetoothPort => {
                    format!("{} (Bluetooth)", port.port_name)
                }
                serialport::SerialPortType::PciPort => {
                    format!("{} (PCI)", port.port_name)
                }
                serialport::SerialPortType::Unknown => port.port_name.clone(),
            };
            result.push_str(&port_name);
        }
        Box::new(put_quoted_str(result))
    }

    pub fn arduino_open(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 5);
        let name = arg(&args, 0);
        let port_name = arg(&args, 1);
        let baud_rate: u32 = arg(&args, 2).parse().unwrap_or(115200);
        let data_bits: u8 = arg(&args, 3).parse().unwrap_or(8);
        let parity = arg(&args, 4).to_uppercase();

        let parity_enum = match parity.as_str() {
            "EVEN" => serialport::Parity::Even,
            "ODD" => serialport::Parity::Odd,
            _ => serialport::Parity::None,
        };

        let port = serialport::new(&port_name, baud_rate)
            .data_bits(data_bits)
            .parity(parity_enum)
            .stop_bits(serialport::StopBits::One)
            .timeout(Duration::from_millis(1000))
            .open();

        if port.is_err() {
            return Box::new(put_quoted_str(format!("Erro: {}", port.err().unwrap())));
        }

        ARDUINO_PORTS.with(|ports| {
            ports.borrow_mut().insert(name.clone(), port.unwrap());
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn arduino_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let data = arg(&args, 1);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                match port.write_all(data.as_bytes()) {
                    Ok(_) => Box::new(put_quoted_str("OK".to_string())),
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let max_bytes: usize = arg(&args, 1).parse().unwrap_or(1024);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let mut buffer = vec![0u8; max_bytes];
                match port.read(&mut buffer) {
                    Ok(bytes_read) => {
                        buffer.truncate(bytes_read);
                        let result = String::from_utf8_lossy(&buffer).to_string();
                        Box::new(put_quoted_str(result))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_read_line(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let mut buffer = vec![0u8; 1024];
                let mut result = String::new();
                loop {
                    match port.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => {
                            let chunk = String::from_utf8_lossy(&buffer[..n]);
                            if chunk.contains('\n') {
                                let lines: Vec<&str> = chunk.split('\n').collect();
                                for (i, line) in lines.iter().enumerate() {
                                    if i == lines.len() - 1 && !chunk.ends_with('\n') {
                                        result.push_str(line);
                                    } else {
                                        result.push_str(line);
                                        break;
                                    }
                                }
                                break;
                            } else {
                                result.push_str(&chunk);
                            }
                        }
                        Err(e) => return Box::new(put_quoted_str(format!("Erro: {}", e))),
                    }
                }
                Box::new(put_quoted_str(result))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_on_data(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let callback = arg(&args, 1);

        ARDUINO_CALLBACKS.with(|cb| {
            cb.borrow_mut().insert(name.clone(), callback);
        });

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(mut port) = ports.get_mut(&name).cloned() {
                let name_clone = name.clone();
                let handle = thread::spawn(move || {
                    let mut buffer = [0u8; 1024];
                    loop {
                        match port.read(&mut buffer) {
                            Ok(0) => break,
                            Ok(n) => {
                                let data = String::from_utf8_lossy(&buffer[..n]);
                                let cb = ARDUINO_CALLBACKS.with(|cb| cb.borrow().get(&name_clone).cloned());
                                if let Some(cb) = cb {
                                    let run = crate::aly::get_runtime();
                                    let fake_lexer = vec![
                                        crate::lexer::Lexer::new(crate::tokens::Tokens::Identifier, cb, 0),
                                    ];
                                    let _ = run.function_run(fake_lexer);
                                }
                            }
                            Err(_) => break,
                        }
                    }
                });
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_close(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        ARDUINO_PORTS.with(|ports| {
            ports.borrow_mut().remove(&name);
        });
        ARDUINO_CALLBACKS.with(|cb| {
            cb.borrow_mut().remove(&name);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn arduino_flush(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let _ = port.flush();
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_set_baud(params: String) -> Box<dyn Validator> {
        // Arduino baud rate change requires reopening the port
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let baud_rate: u32 = arg(&args, 1).parse().unwrap_or(115200);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if ports.contains_key(&name) {
                // We need to close and reopen with new baud rate
                ports.remove(&name);
                // Note: User must call arduino_open again with new baud rate
                Box::new(put_quoted_str("Reabra a porta com novo baud rate".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_pin_mode(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let name = arg(&args, 0);
        let pin: u8 = arg(&args, 1).parse().unwrap_or(0);
        let mode = arg(&args, 2).to_uppercase();

        // Send pin mode command to Arduino
        let cmd = match mode.as_str() {
            "INPUT" => format!("PM:{}:I", pin),
            "OUTPUT" => format!("PM:{}:O", pin),
            "INPUT_PULLUP" => format!("PM:{}:P", pin),
            _ => return Box::new(put_quoted_str("Erro: modo inválido".to_string())),
        };

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let _ = port.write_all(cmd.as_bytes());
                let _ = port.write_all(b"\n");
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_digital_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let name = arg(&args, 0);
        let pin: u8 = arg(&args, 1).parse().unwrap_or(0);
        let value: u8 = arg(&args, 2).parse().unwrap_or(0);

        let cmd = format!("DW:{}:{}", pin, value);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let _ = port.write_all(cmd.as_bytes());
                let _ = port.write_all(b"\n");
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_digital_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let pin: u8 = arg(&args, 1).parse().unwrap_or(0);

        let cmd = format!("DR:{}", pin);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let _ = port.write_all(cmd.as_bytes());
                let _ = port.write_all(b"\n");
                let mut buffer = [0u8; 16];
                match port.read(&mut buffer) {
                    Ok(n) => {
                        let result = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                        Box::new(put_quoted_str(result))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_analog_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let name = arg(&args, 0);
        let pin: u8 = arg(&args, 1).parse().unwrap_or(0);
        let value: u8 = arg(&args, 2).parse().unwrap_or(0);

        let cmd = format!("AW:{}:{}", pin, value);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let _ = port.write_all(cmd.as_bytes());
                let _ = port.write_all(b"\n");
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }

    pub fn arduino_analog_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let pin: u8 = arg(&args, 1).parse().unwrap_or(0);

        let cmd = format!("AR:{}", pin);

        ARDUINO_PORTS.with(|ports| {
            let mut ports = ports.borrow_mut();
            if let Some(port) = ports.get_mut(&name) {
                let _ = port.write_all(cmd.as_bytes());
                let _ = port.write_all(b"\n");
                let mut buffer = [0u8; 16];
                match port.read(&mut buffer) {
                    Ok(n) => {
                        let result = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                        Box::new(put_quoted_str(result))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: Arduino não encontrado".to_string()))
            }
        })
    }
}

#[cfg(not(feature = "iot-arduino"))]
mod arduino_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn arduino_list(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_open(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_read_line(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_on_data(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_close(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_flush(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_set_baud(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_pin_mode(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_digital_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_digital_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_analog_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
    pub fn arduino_analog_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'iot-arduino' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "iot-arduino"))]
pub use arduino_stub::*;