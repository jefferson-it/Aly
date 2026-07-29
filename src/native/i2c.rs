#[cfg(feature = "rpi-i2c")]
mod i2c_mod {
    use rppal::i2c::I2c;
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::cell::RefCell;

    use crate::native::std::{arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    thread_local! {
        static I2C_DEVICES: RefCell<HashMap<String, I2c>> = RefCell::new(HashMap::new());
    }

    pub fn i2c_open(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let name = arg(&args, 0);
        let bus: u8 = arg(&args, 1).parse().unwrap_or(1);
        let address: u16 = arg(&args, 2).parse().unwrap_or(0);

        let i2c = I2c::with_bus(bus);
        if i2c.is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível abrir barramento I2C".to_string()));
        }
        let mut i2c = i2c.unwrap();
        
        if i2c.set_slave_address(address).is_err() {
            return Box::new(put_quoted_str("Erro: endereço I2C inválido".to_string()));
        }

        I2C_DEVICES.with(|devices| {
            devices.borrow_mut().insert(name.clone(), i2c);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn i2c_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let data_str = arg(&args, 1);

        let data: Vec<u8> = data_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        I2C_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(i2c) = devices.get_mut(&name) {
                match i2c.write(&data) {
                    Ok(_) => Box::new(put_quoted_str("OK".to_string())),
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: dispositivo I2C não encontrado".to_string()))
            }
        })
    }

    pub fn i2c_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let len: usize = arg(&args, 1).parse().unwrap_or(1);

        I2C_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(i2c) = devices.get_mut(&name) {
                let mut buffer = vec![0u8; len];
                match i2c.read(&mut buffer) {
                    Ok(_) => {
                        let hex = buffer.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(",");
                        Box::new(put_quoted_str(hex))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: dispositivo I2C não encontrado".to_string()))
            }
        })
    }

    pub fn i2c_write_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let name = arg(&args, 0);
        let write_data = arg(&args, 1);
        let read_len: usize = arg(&args, 2).parse().unwrap_or(1);

        let write_bytes: Vec<u8> = write_data
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        I2C_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(i2c) = devices.get_mut(&name) {
                let mut buffer = vec![0u8; read_len];
                match i2c.write_read(&write_bytes, &mut buffer) {
                    Ok(_) => {
                        let hex = buffer.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(",");
                        Box::new(put_quoted_str(hex))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: dispositivo I2C não encontrado".to_string()))
            }
        })
    }

    pub fn i2c_close(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        I2C_DEVICES.with(|devices| {
            devices.borrow_mut().remove(&name);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn i2c_scan(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let bus: u8 = arg(&args, 0).parse().unwrap_or(1);

        let i2c = I2c::with_bus(bus);
        if i2c.is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível abrir barramento I2C".to_string()));
        }
        let mut i2c = i2c.unwrap();

        let mut found = Vec::new();
        for addr in 0x03..=0x77 {
            if i2c.set_slave_address(addr).is_ok() {
                let mut buf = [0u8; 1];
                if i2c.read(&mut buf).is_ok() {
                    found.push(format!("0x{:02X}", addr));
                }
            }
        }

        Box::new(put_quoted_str(found.join(",")))
    }
}

#[cfg(feature = "rpi-i2c")]
pub use i2c_mod::*;

#[cfg(not(feature = "rpi-i2c"))]
mod i2c_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn i2c_open(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-i2c' não habilitada".to_string()))
    }
    pub fn i2c_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-i2c' não habilitada".to_string()))
    }
    pub fn i2c_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-i2c' não habilitada".to_string()))
    }
    pub fn i2c_write_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-i2c' não habilitada".to_string()))
    }
    pub fn i2c_close(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-i2c' não habilitada".to_string()))
    }
    pub fn i2c_scan(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-i2c' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "rpi-i2c"))]
pub use i2c_stub::*;