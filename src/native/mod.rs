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
        runtime::{interpreter::exec, memory::{address_of_var, heap_alloc, heap_alloc_typed, heap_exists, heap_free, heap_load, heap_pointer, heap_set_mutable, heap_store}},
        tokens::Tokens,
        validators::{
            is_any_value,
            reference::is_reference,
            str::{is_template_str, put_quoted_str, replace_spined, use_template_str},
        },
    };

    use super::types::{Pointer, Validator, ValueData};

    /// Colapsa os blocos de ponteiro `<&x>` e `<*p>` em tokens sintéticos:
    /// - `[<, &x, >]` → `[&x]` (endereço de x)
    /// - `[<, *, p, >]` → `[*p]` (desreferência de p)
    /// Idempotente: tokens já colapsados não casam mais com os padrões.
    pub fn normalize_pointer_blocks(lexers: &mut Vec<Lexer>) {
        let mut out: Vec<Lexer> = Vec::with_capacity(lexers.len());
        let mut i = 0;
        while i < lexers.len() {
            let is_addr = lexers[i].token == Tokens::LessThan
                && i + 2 < lexers.len()
                && lexers[i + 1].literal.starts_with('&')
                && lexers[i + 2].token == Tokens::GreaterThan;
            if is_addr {
                out.push(Lexer::new(
                    Tokens::Pointer,
                    lexers[i + 1].literal.clone(),
                    lexers[i + 1].line,
                ));
                i += 3;
                continue;
            }

            let is_deref = lexers[i].token == Tokens::LessThan
                && i + 3 < lexers.len()
                && lexers[i + 1].token == Tokens::Multiplication
                && lexers[i + 2].token == Tokens::Reference
                && lexers[i + 3].token == Tokens::GreaterThan;
            if is_deref {
                out.push(Lexer::new(
                    Tokens::Pointer,
                    format!("*{}", lexers[i + 2].literal),
                    lexers[i + 2].line,
                ));
                i += 4;
                continue;
            }

            out.push(lexers[i].clone());
            i += 1;
        }
        *lexers = out;
    }

    pub fn exec_rust(expression: String) -> Result<String, AlyError> {
        eval_math(&expression).map_err(|e| {
            AlyError::runtime(format!(
                "Erro ao avaliar expressão '{}': {}",
                expression, e.message
            ))
        })
    }

        pub fn tomb(args: &[ValueData]) -> ValueData {
        for arg in args {
            match arg {
                ValueData::Pointer(ptr) => {
                    let run = get_runtime();
                    if let Err(e) = heap_set_mutable(ptr.address, false) {
                        eprintln!("RuntimeError: {}", e);
                        continue;
                    }
                    // Congela também a variável nomeada (in_mut), se houver.
                    let name = run
                        .get_vars()
                        .iter()
                        .find(|v| v.get_value() == arg.clone())
                        .map(|v| v.get_name());
                    if let Some(name) = name {
                        if let Ok(v) = run.get_var_per_name(name) {
                            if let Err(err) = v.in_mut() {
                                eprintln!("RuntimeError: {}", err);
                            }
                        }
                    }
                }
                other => {
                    eprintln!(
                        "RuntimeError: tomb não aceita valores, apenas endereços de variáveis (use <&nome>); recebido: {}",
                        other.type_name()
                    );
                }
            }
        }

        ValueData::String("None".to_owned())
    }

    pub fn fun_addr(args: &[ValueData]) -> ValueData {
        match args.first() {
            Some(ValueData::Pointer(p)) => ValueData::Pointer(p.clone()),
            Some(other) => {
                eprintln!(
                    "TypeError: addr espera um ponteiro, recebeu {}.",
                    other.type_name()
                );
                ValueData::String("None".to_owned())
            }
            None => {
                eprintln!("TypeError: addr espera 1 argumento.");
                ValueData::String("None".to_owned())
            }
        }
    }

    pub fn fun_deref(args: &[ValueData]) -> ValueData {
        match args.first() {
            Some(ValueData::Pointer(p)) => match heap_load(p.address) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("RuntimeError: {}", e);
                    ValueData::String("None".to_owned())
                }
            },
            Some(other) => {
                eprintln!(
                    "TypeError: deref espera um ponteiro, recebeu {}.",
                    other.type_name()
                );
                ValueData::String("None".to_owned())
            }
            None => {
                eprintln!("TypeError: deref espera 1 argumento.");
                ValueData::String("None".to_owned())
            }
        }
    }

    pub fn fun_store(args: &[ValueData]) -> ValueData {
        match args {
            [ValueData::Pointer(p), value] => match heap_store(p.address, value.clone()) {
                Ok(()) => ValueData::String("None".to_owned()),
                Err(e) => {
                    eprintln!("RuntimeError: {}", e);
                    ValueData::String("None".to_owned())
                }
            },
            _ => {
                eprintln!("TypeError: store espera (ponteiro, valor).");
                ValueData::String("None".to_owned())
            }
        }
    }

    pub fn fun_alloc(args: &[ValueData]) -> ValueData {
        match args.first() {
            Some(v) => heap_alloc(v.clone()),
            None => {
                eprintln!("TypeError: alloc espera 1 valor.");
                ValueData::String("None".to_owned())
            }
        }
    }

    pub fn fun_alloc_typed(args: &[ValueData]) -> ValueData {
        match args {
            [ValueData::String(ty), v] => heap_alloc_typed(ty, v.clone()),
            _ => {
                eprintln!("TypeError: alloc_typed espera (\"i8\", valor).");
                ValueData::String("None".to_owned())
            }
        }
    }

    pub fn fun_free(args: &[ValueData]) -> ValueData {
        match args.first() {
            Some(ValueData::Pointer(p)) => match heap_free(p.address) {
                Ok(()) => ValueData::String("None".to_owned()),
                Err(e) => {
                    eprintln!("RuntimeError: {}", e);
                    ValueData::String("None".to_owned())
                }
            },
            Some(other) => {
                eprintln!(
                    "TypeError: free espera um ponteiro, recebeu {}.",
                    other.type_name()
                );
                ValueData::String("None".to_owned())
            }
            None => {
                eprintln!("TypeError: free espera 1 argumento.");
                ValueData::String("None".to_owned())
            }
        }
    }

    pub fn fun_is_null(args: &[ValueData]) -> ValueData {
        match args.first() {
            Some(ValueData::Pointer(p)) => {
                ValueData::Bool(p.is_null || !heap_exists(p.address))
            }
            Some(ValueData::String(s)) => {
                ValueData::Bool(s.trim() == "null" || s.trim() == "None")
            }
            _ => ValueData::Bool(false),
        }
    }

    pub fn fun_ptr_type(args: &[ValueData]) -> ValueData {
        match args.first() {
            Some(ValueData::Pointer(p)) => match &p.ty {
                Some(t) => ValueData::String(t.to_string()),
                None => ValueData::String("ptr".to_string()),
            },
            _ => ValueData::String("None".to_string()),
        }
    }

    pub fn process_value(mut lexers: Vec<Lexer>) -> ValueData {
        normalize_pointer_blocks(&mut lexers);

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

            if val.literal.starts_with('&') {
                // <&x> — endereço real de x (ou endereço numérico bruto: &3)
                let name = val.literal.trim_start_matches('&').to_string();
                if let Ok(addr) = name.parse::<usize>() {
                    return heap_pointer(addr);
                }
                match address_of_var(aly, name, val.line) {
                    Ok(p) => return p,
                    Err(e) => {
                        eprintln!("RuntimeError: {}", e);
                        return ValueData::String("None".to_owned());
                    }
                }
            } else if val.literal.starts_with('*') {
                // <*p> — desreferência do ponteiro p (ou endereço numérico: *3)
                let name = val.literal.trim_start_matches('*').to_string();
                if let Ok(addr) = name.parse::<usize>() {
                    match heap_load(addr) {
                        Ok(v) => return v,
                        Err(e) => {
                            eprintln!("RuntimeError: {}", e);
                            return ValueData::String("None".to_owned());
                        }
                    }
                }
                match aly.get_var_per_name(name.clone()) {
                    Ok(var) => match var.get_value() {
                        ValueData::Pointer(ptr) => match heap_load(ptr.address) {
                            Ok(v) => return v,
                            Err(e) => {
                                eprintln!("RuntimeError: {}", e);
                                return ValueData::String("None".to_owned());
                            }
                        },
                        other => {
                            eprintln!(
                                "TypeError: '{}' não é um ponteiro (é {}).",
                                name,
                                other.type_name()
                            );
                            return ValueData::String("None".to_owned());
                        }
                    },
                    Err(err) => {
                        eprintln!("ReferenceError: {}", err);
                        return ValueData::String("None".to_owned());
                    }
                }
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
