use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use serialport::{SerialPort, SerialPortType, UsbPortInfo, SerialPortInfo};
use lazy_static::lazy_static;

use crate::native::{
    std::{arg, split_args},
    types::Validator,
};
use crate::validators::str::put_quoted_str;
    static ref SERIAL_PORTS: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort + Send>>>>> = Mutex::new(HashMap::new());
    static ref READ_THREADS: Mutex<HashMap<String, thread::JoinHandle<()>>> = Mutex::new(HashMap::new());
    static ref SERIAL_CALLBACKS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[cfg(feature = "serial")]
mod serial_mod {
    use super::*;

    pub fn serial_list(_params: String) -> Box<dyn Validator> {
        let ports = serialport::available_ports().unwrap_or_default();
        let mut result = String::new();
        for (i, port) in ports.iter().enumerate() {
            if i > 0 {
                result.push(',');
            }
            let port_name = match &port.port_type {
                SerialPortType::UsbPort(info) => format!("{} ({}:{})", port.port_name, info.vid, info.pid),
                SerialPortType::BluetoothPort => format!("{} (Bluetooth)", port.port_name),
                SerialPortType::PciPort => format!("{} (PCI)", port.port_name),
                SerialPortType::Unknown => port.port_name.clone(),
            };
            result.push_str(&port_name);
        }
        Box::new(put_quoted_str(result))
    }

    pub fn serial_open(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 5);
        let port_name = arg(&args, 0);
        let baud_rate: u32 = arg(&args, 1).parse().unwrap_or(9600);
        let data_bits: u8 = arg(&args, 2).parse().unwrap_or(8);
        let parity = arg(&args, 3).to_uppercase();
        let stop_bits: u8 = arg(&args, 4).parse().unwrap_or(1);

        let parity_enum = match parity.as_str() {
            "EVEN" => serialport::Parity::Even,
            "ODD" => serialport::Parity::Odd,
            _ => serialport::Parity::None,
        };

        let builder = serialport::new(&port_name, baud_rate)
            .data_bits(data_bits)
            .parity(parity_enum)
            .stop_bits(match stop_bits {
                2 => serialport::StopBits::Two,
                _ => serialport::StopBits::One,
            })
            .timeout(Duration::from_millis(100));

        match builder.open() {
            Ok(port) => {
                let id = port_name.clone();
                SERIAL_PORTS.lock().unwrap().insert(id.clone(), Arc::new(Mutex::new(port)));
                Box::new(put_quoted_str(format!("OK:{}", id)))
            }
            Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
        }
    }

    pub fn serial_close(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let port_id = arg(&args, 0);

        let mut ports = SERIAL_PORTS.lock().unwrap();
        if ports.remove(&port_id).is_some() {
            READ_THREADS.lock().unwrap().remove(&port_id);
            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: porta não encontrada".to_string()))
        }
    }

    pub fn serial_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let port_id = arg(&args, 0);
        let data = arg(&args, 1);

        let ports = SERIAL_PORTS.lock().unwrap();
        if let Some(port) = ports.get(&port_id) {
            let mut p = port.lock().unwrap();
            match p.write(data.as_bytes()) {
                Ok(bytes) => Box::new(put_quoted_str(format!("OK:{}", bytes))),
                Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
            }
        } else {
            Box::new(put_quoted_str("Erro: porta não encontrada".to_string()))
        }
    }

    pub fn serial_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let port_id = arg(&args, 0);
        let max_bytes: usize = arg(&args, 1).parse().unwrap_or(1024);

        let ports = SERIAL_PORTS.lock().unwrap();
        if let Some(port) = ports.get(&port_id) {
            let mut p = port.lock().unwrap();
            let mut buffer = vec![0u8; max_bytes];
            match p.read(&mut buffer) {
                Ok(bytes_read) => {
                    buffer.truncate(bytes_read);
                    let result = String::from_utf8_lossy(&buffer).to_string();
                    Box::new(put_quoted_str(result))
                }
                Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
            }
        } else {
            Box::new(put_quoted_str("Erro: porta não encontrada".to_string()))
        }
    }

    pub fn serial_read_line(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let port_id = arg(&args, 0);

        let ports = SERIAL_PORTS.lock().unwrap();
        if let Some(port) = ports.get(&port_id) {
            let mut p = port.lock().unwrap();
            let mut buffer = vec![0u8; 1024];
            let mut result = String::new();
            loop {
                match p.read(&mut buffer) {
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
            Box::new(put_quoted_str("Erro: porta não encontrada".to_string()))
        }
    }

    pub fn serial_on_data(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let port_id = arg(&args, 0);
        let callback = arg(&args, 1);

        SERIAL_CALLBACKS.lock().unwrap().insert(port_id.clone(), callback);
        
        let port_id_clone = port_id.clone();
        let ports = SERIAL_PORTS.lock().unwrap();
        if let Some(port) = ports.get(&port_id) {
            let port_clone = port.clone();
            let handle = thread::spawn(move || {
                let mut buffer = [0u8; 1024];
                loop {
                    let mut p = port_clone.lock().unwrap();
                    match p.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => {
                            let data = String::from_utf8_lossy(&buffer[..n]);
                            let cb = SERIAL_CALLBACKS.lock().unwrap().get(&port_id_clone).cloned();
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
            READ_THREADS.lock().unwrap().insert(port_id, handle);
            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: porta não encontrada".to_string()))
        }
    }
}

#[cfg(feature = "serial")]
pub use serial_mod::*;

#[cfg(not(feature = "serial"))]
mod serial_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn serial_list(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
    pub fn serial_open(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
    pub fn serial_close(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
    pub fn serial_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
    pub fn serial_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
    pub fn serial_read_line(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
    pub fn serial_on_data(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'serial' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "serial"))]
pub use serial_stub::*;