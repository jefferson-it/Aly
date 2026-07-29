#[cfg(feature = "rpi-spi")]
mod spi_mod {
    use rppal::spi::{Spi, Mode, BitOrder};
    use std::collections::HashMap;
    use std::cell::RefCell;

    use crate::native::std::{arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    thread_local! {
        static SPI_DEVICES: RefCell<HashMap<String, Spi>> = RefCell::new(HashMap::new());
    }

    pub fn spi_open(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 6);
        let name = arg(&args, 0);
        let bus: u8 = arg(&args, 1).parse().unwrap_or(0);
        let chip_select: u8 = arg(&args, 2).parse().unwrap_or(0);
        let speed: u32 = arg(&args, 3).parse().unwrap_or(500_000);
        let mode = arg(&args, 4).to_uppercase();
        let bit_order = arg(&args, 5).to_uppercase();

        let mut spi = Spi::new(bus, chip_select, speed, match mode.as_str() {
            "MODE1" => Mode::Mode1,
            "MODE2" => Mode::Mode2,
            "MODE3" => Mode::Mode3,
            _ => Mode::Mode0,
        });

        if spi.is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível abrir barramento SPI".to_string()));
        }
        
        let mut spi = spi.unwrap();
        spi.set_bit_order(match bit_order.as_str() {
            "LSB" => BitOrder::LsbFirst,
            _ => BitOrder::MsbFirst,
        });

        SPI_DEVICES.with(|devices| {
            devices.borrow_mut().insert(name.clone(), spi);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn spi_transfer(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let data_str = arg(&args, 1);

        let write_data: Vec<u8> = data_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        let mut read_buffer = vec![0u8; write_data.len()];

        SPI_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(spi) = devices.get_mut(&name) {
                match spi.transfer(&mut read_buffer, &write_data) {
                    Ok(_) => {
                        let hex = read_buffer.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(",");
                        Box::new(put_quoted_str(hex))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: dispositivo SPI não encontrado".to_string()))
            }
        })
    }

    pub fn spi_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let data_str = arg(&args, 1);

        let write_data: Vec<u8> = data_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        SPI_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(spi) = devices.get_mut(&name) {
                match spi.write(&write_data) {
                    Ok(_) => Box::new(put_quoted_str("OK".to_string())),
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: dispositivo SPI não encontrado".to_string()))
            }
        })
    }

    pub fn spi_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let len: usize = arg(&args, 1).parse().unwrap_or(1);

        let mut read_buffer = vec![0u8; len];

        SPI_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(spi) = devices.get_mut(&name) {
                match spi.read(&mut read_buffer) {
                    Ok(_) => {
                        let hex = read_buffer.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(",");
                        Box::new(put_quoted_str(hex))
                    }
                    Err(e) => Box::new(put_quoted_str(format!("Erro: {}", e))),
                }
            } else {
                Box::new(put_quoted_str("Erro: dispositivo SPI não encontrado".to_string()))
            }
        })
    }

    pub fn spi_close(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let name = arg(&args, 0);

        SPI_DEVICES.with(|devices| {
            devices.borrow_mut().remove(&name);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn spi_set_mode(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let name = arg(&args, 0);
        let mode = arg(&args, 1).to_uppercase();
        let bit_order = arg(&args, 2).to_uppercase();

        SPI_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(spi) = devices.get_mut(&name) {
                spi.set_mode(match mode.as_str() {
                    "MODE1" => Mode::Mode1,
                    "MODE2" => Mode::Mode2,
                    "MODE3" => Mode::Mode3,
                    _ => Mode::Mode0,
                });
                spi.set_bit_order(match bit_order.as_str() {
                    "LSB" => BitOrder::LsbFirst,
                    _ => BitOrder::MsbFirst,
                });
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: dispositivo SPI não encontrado".to_string()))
            }
        })
    }

    pub fn spi_set_speed(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let name = arg(&args, 0);
        let speed: u32 = arg(&args, 1).parse().unwrap_or(500_000);

        SPI_DEVICES.with(|devices| {
            let mut devices = devices.borrow_mut();
            if let Some(spi) = devices.get_mut(&name) {
                spi.set_speed(speed);
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: dispositivo SPI não encontrado".to_string()))
            }
        })
    }
}

#[cfg(feature = "rpi-spi")]
pub use spi_mod::*;

#[cfg(not(feature = "rpi-spi"))]
mod spi_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn spi_open(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
    pub fn spi_transfer(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
    pub fn spi_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
    pub fn spi_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
    pub fn spi_close(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
    pub fn spi_set_mode(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
    pub fn spi_set_speed(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-spi' não habilitada".to_string()))
    }
}

#[cfg(not(feature = "rpi-spi"))]
pub use spi_stub::*;