#[cfg(any(feature = "rpi-gpio", feature = "iot-raspberry-pi"))]
mod gpio_mod {
    use rppal::gpio::{Gpio, InputPin, OutputPin, Level, Trigger};
    use std::sync::Arc;
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::thread;

    use crate::native::std::{arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    thread_local! {
        static GPIO_PINS: RefCell<HashMap<u8, PinState>> = RefCell::new(HashMap::new());
        static GPIO_CALLBACKS: RefCell<HashMap<u8, String>> = RefCell::new(HashMap::new());
    }

    enum PinState {
        Input(InputPin),
        Output(OutputPin),
    }

    pub fn gpio_setup(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);
        let direction = arg(&args, 1).to_lowercase();

        let gpio = Gpio::new();
        if gpio.is_err() {
            return Box::new(put_quoted_str("Erro: não foi possível inicializar GPIO".to_string()));
        }
        let gpio = gpio.unwrap();

        GPIO_PINS.with(|pins| {
            let mut pins = pins.borrow_mut();
            if direction == "out" || direction == "output" {
                if let Ok(p) = gpio.get(pin).and_then(|p| p.into_output()) {
                    pins.insert(pin, PinState::Output(p));
                } else {
                    return;
                }
            } else {
                if let Ok(p) = gpio.get(pin).and_then(|p| p.into_input()) {
                    pins.insert(pin, PinState::Input(p));
                } else {
                    return;
                }
            }
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn gpio_write(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);
        let value: u8 = arg(&args, 1).parse().unwrap_or(0);

        GPIO_PINS.with(|pins| {
            let mut pins = pins.borrow_mut();
            if let Some(PinState::Output(p)) = pins.get_mut(&pin) {
                let level = if value != 0 { Level::High } else { Level::Low };
                let _ = p.write(level);
                Box::new(put_quoted_str("OK".to_string()))
            } else {
                Box::new(put_quoted_str("Erro: pino não configurado como saída".to_string()))
            }
        })
    }

    pub fn gpio_read(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);

        GPIO_PINS.with(|pins| {
            let pins = pins.borrow();
            if let Some(PinState::Input(p)) = pins.get(&pin) {
                let level = p.read();
                return Box::new(put_quoted_str(if level == Level::High { "1" } else { "0" }.to_string()));
            }
            // OutputPin doesn't have read() in rppal
            Box::new(put_quoted_str("Erro: pino é saída, use gpio_read em pino de entrada".to_string()))
        })
    }

    pub fn gpio_on_rising(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);
        let callback = arg(&args, 1);

        let gpio = Gpio::new().unwrap();
        if let Ok(p) = gpio.get(pin).and_then(|p| p.into_input()) {
            let callback_clone = callback.clone();
            p.set_async_interrupt(Trigger::RisingEdge, move |_| {
                let run = crate::aly::get_runtime();
                let fake_lexer = vec![crate::lexer::Lexer::new(crate::tokens::Tokens::Identifier, callback_clone.clone(), 0)];
                let _ = run.function_run(fake_lexer);
            });
            
            GPIO_PINS.with(|pins| {
                pins.borrow_mut().insert(pin, PinState::Input(p));
            });
            
            GPIO_CALLBACKS.with(|cb| {
                cb.borrow_mut().insert(pin, callback);
            });

            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: pino inválido".to_string()))
        }
    }

    pub fn gpio_on_falling(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);
        let callback = arg(&args, 1);

        let gpio = Gpio::new().unwrap();
        if let Ok(p) = gpio.get(pin).and_then(|p| p.into_input()) {
            let callback_clone = callback.clone();
            p.set_async_interrupt(Trigger::FallingEdge, move |_| {
                let run = crate::aly::get_runtime();
                let fake_lexer = vec![crate::lexer::Lexer::new(crate::tokens::Tokens::Identifier, callback_clone.clone(), 0)];
                let _ = run.function_run(fake_lexer);
            });
            
            GPIO_PINS.with(|pins| {
                pins.borrow_mut().insert(pin, PinState::Input(p));
            });
            
            GPIO_CALLBACKS.with(|cb| {
                cb.borrow_mut().insert(pin, callback);
            });

            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: pino inválido".to_string()))
        }
    }

    pub fn gpio_on_change(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 2);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);
        let callback = arg(&args, 1);

        let gpio = Gpio::new().unwrap();
        if let Ok(p) = gpio.get(pin).and_then(|p| p.into_input()) {
            let callback_clone = callback.clone();
            p.set_async_interrupt(Trigger::Both, move |_| {
                let run = crate::aly::get_runtime();
                let fake_lexer = vec![crate::lexer::Lexer::new(crate::tokens::Tokens::Identifier, callback_clone.clone(), 0)];
                let _ = run.function_run(fake_lexer);
            });
            
            GPIO_PINS.with(|pins| {
                pins.borrow_mut().insert(pin, PinState::Input(p));
            });
            
            GPIO_CALLBACKS.with(|cb| {
                cb.borrow_mut().insert(pin, callback);
            });

            Box::new(put_quoted_str("OK".to_string()))
        } else {
            Box::new(put_quoted_str("Erro: pino inválido".to_string()))
        }
    }

    pub fn gpio_remove_callback(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 1);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);

        GPIO_CALLBACKS.with(|cb| {
            cb.borrow_mut().remove(&pin);
        });

        GPIO_PINS.with(|pins| {
            pins.borrow_mut().remove(&pin);
        });

        Box::new(put_quoted_str("OK".to_string()))
    }

    pub fn gpio_pwm(params: String) -> Box<dyn Validator> {
        let args = split_args(&params, 3);
        let pin: u8 = arg(&args, 0).parse().unwrap_or(0);
        let frequency: f64 = arg(&args, 1).parse().unwrap_or(1000.0);
        let duty_cycle: f64 = arg(&args, 2).parse().unwrap_or(0.5);

        let gpio = Gpio::new().unwrap();
        match gpio.get(pin).and_then(|p| p.into_output()) {
            Some(mut p) => {
                let _ = p.set_pwm_frequency(frequency, duty_cycle);
                GPIO_PINS.with(|pins| {
                    pins.borrow_mut().insert(pin, PinState::Output(p));
                });
                Box::new(put_quoted_str("OK".to_string()))
            }
            None => Box::new(put_quoted_str("Erro: pino inválido".to_string()))
        }
    }

    pub fn gpio_cleanup(_params: String) -> Box<dyn Validator> {
        GPIO_PINS.with(|pins| {
            pins.borrow_mut().clear();
        });
        GPIO_CALLBACKS.with(|cb| {
            cb.borrow_mut().clear();
        });
        Box::new(put_quoted_str("OK".to_string()))
    }
}

#[cfg(any(feature = "rpi-gpio", feature = "iot-raspberry-pi"))]
pub use gpio_mod::*;

#[cfg(not(any(feature = "rpi-gpio", feature = "iot-raspberry-pi")))]
mod gpio_stub {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    pub fn gpio_setup(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_write(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_read(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_on_rising(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_on_falling(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_on_change(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_remove_callback(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_pwm(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
    pub fn gpio_cleanup(_: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str("Erro: feature 'rpi-gpio' não habilitada".to_string()))
    }
}

#[cfg(not(any(feature = "rpi-gpio", feature = "iot-raspberry-pi")))]
pub use gpio_stub::*;