#[cfg(feature = "rpi-uart")]
mod uart_mod {
    use rppal::uart::{Uart, Parity, StopBits};
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::thread;
    use std::time::Duration;

    use crate::native::std::{arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    thread_local! {
        static UART_DEVICES: RefCell<HashMap<String, Uart>> = RefCell::new(HashMap::new());
        static UART_CALLBACKS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
        static UART_THREADS: RefCell<HashMap<String, thread::JoinHandle<()>>> = RefCell::new(HashMap::new());
    }

    pub fn uart_open(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 6);
        let name = arg(&args, 0);
        let path = arg(&args, 1);
        let baud_rate: u32 = arg(&args, 2).parse().unwrap_or(9600);
        let data_bits: u8 = arg(&args, 3).parse().unwrap_or(8);
        let parity = arg(&args, 4).to_uppercase();
        let stop_bits: u8 = arg(&args, 5).parse().unwrap_or(1);

        let mut uart = Uart::new(path, baud_rate, Parity::None, StopBits::One, data_bits);
        if uart.is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível abrir UART".to_string()));
        }
        let mut uart = uart.unwrap();
        
        uart.set_parity(match parity.as_str() {
            "EVEN" => Parity::Even,
            "ODD" => Parity::Odd,
            _ => Parity::None,
        });
        uart.set_stop_bits(match stop_bits {
            2 => StopBits::Two,
            _ => StopBits::One,
        });

        UART_DEVICES.with(|devices| {
            devices.borrow_mut().insert(name.clone(), uart);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn uart_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let data = arg(&args, 1);

        UART_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(uart) = devices.get_mut(&name) {
                match uart.write(data.as_bytes()) {
                    Ok(bytes) => Box::new(put_quoted_str(format!("OK:{}", bytes))),
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: UART não encontrado".to_string()))
            }
        })
    }

    pub fn uart_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let max_bytes: usize = arg(&args, 1).parse().unwrap_or(1024);

        UART_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(uart) = devices.get_mut(&name) {
                let mut buffer = vec![0u8; max_bytes];
                match uart.read(&mut buffer) {
                    Ok(bytes_read) => {
                        buffer.truncate(bytes_read);
                        let result = String::from_utf8_lossy(&buffer).to_string();
                        Box::new(put_quoted_str(result))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: UART não encontrado".to_string()))
            }
        })
    }

    pub fn uart_read_line(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        UART_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(uart) = devices.get_mut(&name) {
                let mut buffer = vec![0u8; 1024];
                let mut result = String::new();
                loop {
                    match uart.read(&mut buffer) {
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
                Box::new(put_quoted_str("Erro: UART não encontrado".to_string()))
            }
        })
    }

    pub fn uart_on_data(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let callback = arg(&args, 1);

        UART_CALLBACKS.with(|cb| {
            cb.borrow_mut().insert(name.clone(), callback);
        });

        UART_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(mut uart) = devices.get_mut(&name).cloned() {
                let name_clone = name.clone();
                let handle = thread::spawn(move || {
                    let mut buffer = [0u8; 1024];
                    loop {
                        match uart.read(&mut buffer) {
                            Ok(0) => break,
                            Ok(n) => {
                                let data = String::from_utf8_lossy(&buffer[..n]);
                                let cb = UART_CALLBACKS.with(|cb| cb.borrow().get(&name_clone).cloned());
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
                UART_THREADS.with(|threads| {
                    threads.borrow_mut().insert(name, handle);
                });
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: UART não encontrado".to_string()))
            }
        })
    }

    pub fn uart_close(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        UART_DEVICES.with(|devices| {
            devices.borrow_mut().remove(&name);
        });
        UART_CALLBACKS.with(|cb| {
            cb.borrow_mut().remove(&name);
        });
        UART_THREADS.with(|threads| {
            threads.borrow_mut().remove(&name);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn uart_flush(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        UART_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(uart) = devices.get_mut(&name) {
                let _ = uart.flush();
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: UART não encontrado".to_string()))
            }
        })
    }

    pub fn uart_set_baud(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let baud_rate: u32 = arg(&args, 1).parse().unwrap_or(9600);

        UART_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(uart) = devices.get_mut(&name) {
                uart.set_baud_rate(baud_rate);
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: UART não encontrado".to_string()))
            }
        })
    }
}

#[cfg(feature = "rpi-uart")]
pub use uart_mod::*;

#[cfg(not(feature = "rpi-uart"))]
mod uart_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn uart_open(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_read_line(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_on_data(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_close(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_flush(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
    pub fn uart_set_baud(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-uart' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "rpi-uart"))]
pub use uart_stub::*;