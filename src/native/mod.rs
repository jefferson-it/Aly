pub mod crypto;
pub mod fs;

pub mod std;
pub mod vars;
pub mod types;
pub mod conditions;
pub mod create_object;
pub mod vector;
pub mod tuple;
pub mod set;
pub mod stack;
pub mod queue;
pub mod linked_list;
pub mod enumeration;
pub mod devices;
pub mod os;
pub mod console;
pub mod http_api;
pub mod curl;
pub mod http;
pub mod shell;
pub mod system;
pub mod platform;
pub mod jot;
pub mod pdf;
pub mod tar;
pub mod graphql;
pub mod grpc;
pub mod proxy;
pub mod rpc;
pub mod microservices;
pub mod serverless;
pub mod game;
pub mod data_science;
pub mod ml;

#[cfg(feature = "mqtt")]
pub mod mqtt;
#[cfg(feature = "serial")]
pub mod serial;
#[cfg(any(feature = "rpi-gpio", feature = "iot-raspberry-pi"))]
pub mod gpio;
#[cfg(feature = "rpi-i2c")]
pub mod i2c;
#[cfg(feature = "rpi-spi")]
pub mod spi;
#[cfg(feature = "rpi-uart")]
pub mod uart;
#[cfg(feature = "iot-esp32")]
pub mod esp32;
#[cfg(feature = "iot-arduino")]
pub mod arduino;
#[cfg(feature = "iot-raspberry-pi")]
pub mod raspberry_pi;

#[cfg(feature = "gui")]
pub mod gtk;
#[cfg(feature = "gui")]
pub mod gui_fltk;
#[cfg(feature = "gui")]
pub mod gui_abstraction;
#[cfg(feature = "gui")]
pub mod gtk_backend;
#[cfg(feature = "gui")]
pub mod dom_backend;
#[cfg(feature = "gui")]
pub mod qt_backend;
#[cfg(feature = "wayland-backend")]
pub mod wayland_backend;
#[cfg(target_os = "ios")]
pub mod ios;
#[cfg(target_os = "ios")]
pub mod ios_backend;
#[cfg(target_os = "ios")]
pub mod ios_hardware;
#[cfg(target_os = "windows")]
pub mod win32_backend;

#[cfg(feature = "android")]
pub mod jni;

#[cfg(feature = "android")]
pub mod android;

pub mod wasm_backend;
pub mod web_components;

mod native {
    use std::io::{stdin, stdout, Write};

    use crate::{
        aly::get_runtime,
        error::AlyError,
        lexer::Lexer,
        math_eval::eval_math,
        runtime::interpreter::exec,
        tokens::Tokens,
        validators::{
            is_any_value,
            reference::is_reference,
            str::{is_template_str, put_quoted_str, replace_spined, use_template_str},
        },
    };

    use super::types::{Validator, ValueData};

    pub fn exec_rust(expression: String) -> Result<String, AlyError> {
        eval_math(&expression).map_err(|e| {
            AlyError::runtime(format!(
                "Erro ao avaliar expressão '{}': {}",
                expression, e.message
            ))
        })
    }

        pub fn tomb(x: String) -> Box<dyn Validator> {
        let run = get_runtime();
        let variables: Vec<&str> = x.split(' ').collect();

        for var in variables {
            if var.starts_with("address_") {
                let name = var[8..].to_string();

                match run.get_var_per_name(name.clone()) {
                    Ok(v) => {
                        if let Err(err) = v.in_mut() {
                            eprintln!("RuntimeError: {}", err);
                        }
                    }
                    Err(err) => {
                        eprintln!("RuntimeError: {}", err);
                    }
                }
            } else {
                eprintln!(
                    "RuntimeError: tomb não aceita valores, apenas endereços de variáveis (use &nome)"
                );
            }
        }

        Box::new("None".to_owned())
    }

    pub fn process_value(mut lexers: Vec<Lexer>) -> ValueData {
        let aly = get_runtime();

        let mut resolved_lexers = Vec::new();
        let mut j = 0;
        while j < lexers.len() {
            if lexers[j].token == Tokens::This && j + 1 < lexers.len() {
                let is_assignment = j + 2 < lexers.len() && lexers[j+2].token == Tokens::Identifier && lexers[j+2].literal == "=";
                if !is_assignment {
                    let mut resolved_val = ValueData::String("None".to_owned());
                    if let Ok(this_var) = aly.get_var_per_name("this".to_string()) {
                        let prop_res = this_var.get_prop(vec![
                            Lexer::new(Tokens::Dot, ".".to_string(), lexers[j].line),
                            lexers[j+1].clone(),
                        ]);
                        resolved_val = prop_res.valid().1;
                    }
                    resolved_lexers.push(Lexer::new(Tokens::Value, resolved_val.to_string(true), lexers[j].line));
                    j += 2;
                    continue;
                }
            }
            resolved_lexers.push(lexers[j].clone());
            j += 1;
        }
        lexers = resolved_lexers;

        if lexers.is_empty() {
            return ValueData::String("None".to_owned());
        } else if lexers.len() > 1 {
            let mut val: Box<dyn Validator> = Box::new(String::new());

            exec(&mut lexers, &mut val);

            let (_, res) = val.valid();

            return res;
        } else {
            let val = lexers[0].clone();
            let mut res = String::new();

            if val.literal.starts_with(&Tokens::Pointer.literal()) {
                res = format!("address_{}", val.literal.replace('&', ""));
            } else if is_any_value(&val.literal) {
                if is_template_str(&val.literal) {
                    res = use_template_str(val.literal);
                } else {
                    res = val.literal;
                }
            } else if is_reference(&val.literal) {
                let var = aly.get_var_per_name(val.literal);

                match var {
                    Ok(i) => return i.get_value().clone(),
                    Err(_) => return ValueData::String("None".to_string()),
                }
            }

            ValueData::String(res)
        }
    }

    pub fn fun_print(x: String) -> Box<dyn Validator> {
        println!("{}", replace_spined(x));
        Box::new("None".to_owned())
    }

    pub fn fun_drop(x: String) -> Box<dyn Validator> {
        let run = get_runtime();
        run.drop_var(x.trim());
        Box::new("None".to_owned())
    }

    pub fn fun_input(x: String) -> Box<dyn Validator> {
        let mut output = String::new();

        print!("{}\n> ", replace_spined(x));

        if let Err(e) = stdout().flush() {
            eprintln!("RuntimeError: falha ao liberar stdout: {}", e);
        }

        if let Err(e) = stdin().read_line(&mut output) {
            eprintln!("RuntimeError: falha ao ler entrada: {}", e);
            return Box::new("None".to_owned());
        }

        let trimmed = output.trim().to_owned();

        if is_any_value(&trimmed) {
            Box::new(trimmed)
        } else {
            Box::new(put_quoted_str(trimmed))
        }
    }
}

pub use native::*;

#[cfg(feature = "android")]
pub use jni::*;
