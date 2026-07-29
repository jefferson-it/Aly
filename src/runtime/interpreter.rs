mod interpreter {
    use regex::Regex;
    use std::sync::OnceLock;
    use std::time::Instant;

    use crate::{
        aly::get_runtime,
        lexer::Lexer,
        native::{
            conditions::exec_cond,
            create_object::create_object,
            tuple::create_tuple,
            exec_rust,
            process_value,
            types::{Validator, ValueData, Type},
            vars::{is_const_declaration, is_var_declaration},
            vector::create_array,
        },
        plugin::load_plugin,
        tokens::Tokens,
        validators::{
            is_compound_assign,
            is_conditional_exp,
            numeric::{def_value_float, is_math_operator},
            str::{put_quoted_str, remove_quoted_str},
            structures::{is_close, is_opened},
        },
    };

    fn line_is_dec(previous: Vec<Lexer>) -> bool {
        previous
            .iter()
            .rev()
            .any(|item| item.token.id() == "identifier")
    }

        pub fn exec(lexers: &mut Vec<Lexer>, val: &mut Box<dyn Validator>) {
        let mut to_made = "none";
        let mut ind = 0;
        let mut previous: Vec<Lexer> = vec![];

        let run = get_runtime();

        for lex in &mut *lexers {
            if ind == 0 {
                if lex.token.id() == "block_if" {
                    to_made = "block_if";
                } else if lex.token.id() == "block_loop" {
                    to_made = "block_loop";
                } else if lex.token.id() == "block_do" {
                    to_made = "block_do";
                } else if lex.token.id() == "block_foreach" {
                    to_made = "block_foreach";
                } else if lex.token.id() == "break" {
                    to_made = "break";
                } else if lex.token.id() == "continue" {
                    to_made = "continue";
                } else if lex.token.id() == "def_plugin" {
                    to_made = "plugin_import";
                } else if lex.token.id() == "def_schema" {
                    to_made = "schema_dec";
                } else if lex.token.id() == "def_type" {
                    to_made = "type_dec";
                } else if lex.token.id() == "new_instance" {
                    to_made = "new_instance";
                }
            }

            if to_made == "block_if" || to_made == "block_loop" || to_made == "block_do" || to_made == "block_foreach" || to_made == "plugin_import" || to_made == "schema_dec" || to_made == "type_dec" {
                previous.push(lex.clone());
                ind += 1;
                continue;
            }
            if to_made == "var_dec" || to_made == "const_dec" {
                previous.push(lex.clone());
                ind += 1;
                continue;
            }

            if is_opened(lex.token.clone()) || is_close(lex.token.clone()) {
                to_made = match lex.token.clone() {
                    Tokens::RightParenthesis | Tokens::LeftParenthesis => {
                        if to_made.contains("_dec") {
                            if line_is_dec(previous.clone()) {
                                to_made
                            } else {
                                "use_fun"
                            }
                        } else {
                            "use_fun"
                        }
                    }
                    Tokens::LeftBracket | Tokens::RightBracket => {
                        if line_is_dec(previous.clone()) {
                            to_made
                        } else if lex.token == Tokens::LeftBracket
                            && !previous.is_empty()
                            && (previous.last().unwrap().token == Tokens::Value
                                || previous.last().unwrap().token == Tokens::None
                                || previous.last().unwrap().token == Tokens::Reference)
                        {
                            "get_index"
                        } else if lex.token == Tokens::RightBracket && to_made == "get_index" {
                            "get_index"
                        } else if lex.token == Tokens::LeftParenthesis && !previous.is_empty() {
                            "create_tuple"
                        } else if lex.token == Tokens::RightParenthesis && to_made == "create_tuple" {
                            "create_tuple"
                        } else {
                            "create_array"
                        }
                    }
                    Tokens::LeftBrace | Tokens::RightBrace => {
                        if line_is_dec(previous.clone()) {
                            to_made
                        } else {
        match to_made {
                                "use_fun" => "dec_fun",
                                _ => "create_object",
                            }
                        }
                    }
                    _ => "",
                }
            } else if lex.token == Tokens::Identifier && to_made == "get_index" {
                to_made = "set_index";
            } else if is_math_operator(lex.token.clone()) {
                to_made = if to_made.contains("_dec") {
                    if line_is_dec(previous.clone()) {
                        to_made
                    } else {
                        "math"
                    }
                } else {
                    "math"
                }
            } else if is_compound_assign(lex.token.clone()) {
                // Compound assignment: +=, -=, *=, /=, %=
                to_made = "compound_assign"
            } else if is_conditional_exp(lex.token.clone()) {
                to_made = if to_made.contains("_dec") {
                    if line_is_dec(previous.clone()) {
                        to_made
                    } else {
                        "conditional"
                    }
                } else {
                    "conditional"
                }
            } else if ind == 0 && lex.token.id() == "block_if" {
                to_made = "block_if";
            } else if ind == 0 && lex.token.id() == "block_loop" {
                to_made = "block_loop";
            } else if ind == 0 && lex.token.id() == "block_do" {
                to_made = "block_do";
            } else if ind == 0 && lex.token.id() == "block_foreach" {
                to_made = "block_foreach";
            } else if ind == 0 && lex.token.id() == Tokens::Value.id() {
                to_made = "use_val";
            } else if is_var_declaration(lex.token.clone()) {
                if (ind > 0
                    && to_made == "var_dec"
                    && is_var_declaration(previous[ind - 1].clone().token))
                    || ind == 0
                {
                    to_made = "var_dec";
                }
            } else if is_const_declaration(lex.token.clone()) {
                if (ind > 0
                    && to_made == "const_dec"
                    && is_const_declaration(previous[ind - 1].clone().token))
                    || ind == 0
                {
                    to_made = "const_dec";
                }
            } else if lex.token.id() == Tokens::Dot.id() {
                if to_made == "use_fun" {
                    previous.push(lex.clone());
                    ind += 1;
                    continue;
                }

                to_made = if to_made.contains("_dec") {
                    if previous
                        .iter()
                        .rev()
                        .skip(1)
                        .any(|item| item.token.id() == "identifier")
                    {
                        to_made
                    } else if previous
                        .iter()
                        .all(|item| item.literal.chars().all(|c| c.is_numeric()))
                    {
                        "value_float"
                    } else {
                        "use_prop"
                    }
                } else if previous
                    .iter()
                    .all(|item| item.token.id().chars().all(|c| c.is_numeric()))
                {
                    to_made
                } else {
                    "use_prop"
                };
            }

            previous.push(lex.clone());
            ind += 1;
        }

        match to_made {
            "var_dec" => run.create_variable(previous),
            "const_dec" => run.create_constant(previous),
            "use_fun" => {
                *val = run.function_run(previous);
            }
            "math" => {
                let mut exp = String::new();

                static RE_PERCENT: OnceLock<Regex> = OnceLock::new();
                static RE_PERCENT_EXP: OnceLock<Regex> = OnceLock::new();
                let re_percent = RE_PERCENT.get_or_init(|| {
                    Regex::new(r"(\d+\s?%)").expect("RE_PERCENT: invalid regex")
                });
                let re_percent_exp = RE_PERCENT_EXP.get_or_init(|| {
                    Regex::new(r"(\d+\s?%\s?\d+)").expect("RE_PERCENT_EXP: invalid regex")
                });

                for prev in previous {
                    match prev.token {
                        Tokens::Value | Tokens::Reference => {
                            let data = process_value([prev].to_vec());
                            exp.push_str(&data.to_string(false));
                        }
                        _ => exp.push_str(&prev.literal),
                    }

                    exp.push(' ');
                }

                // Handle `x % y` style percentage expressions first
                let caps: Vec<_> = re_percent_exp
                    .captures_iter(&exp)
                    .map(|cap| cap[0].to_string())
                    .collect();

                for item in caps {
                    let target: Vec<_> = item.split('%').collect();
                    let item1 = target[0].trim();
                    let item2 = target[1].trim();
                    let final_exp = format!("({item1} * ({item2} / 100))");
                    exp = exp.replace(&item, &final_exp);
                }

                let caps_percent: Vec<_> = re_percent
                    .captures_iter(&exp)
                    .map(|cap| cap[0].to_string())
                    .collect();

                for item in caps_percent {
                    let target: Vec<_> = item.split('%').collect();
                    let item1 = target[0].trim();
                    let final_exp = format!("({item1} / 100)");
                    exp = exp.replace(&item, &final_exp);
                }

                // Replace modulus pipe `|` with `%`
                exp = exp.replace('|', "%");

                match exec_rust(exp.clone()) {
                    Err(err) => {
                        eprintln!("RuntimeError: {}", err.message);
                    }
                    Ok(res) => {
                        *val = Box::new(res);
                    }
                }
            }
            "conditional" => {
                *val = exec_cond(previous);
            }
            "try_catch" => {
                // Implementação básica de try/catch
                // Estrutura: try { body } catch (var) { body } finally { body }
                // Esta parte exige uma reestruturação profunda do `exec_block` para lidar com estados de erro
            }
            "compound_assign" => {
                // Handles: name += value, name -= value, etc.
                // The form is: [name, operator, value...]
                if previous.len() >= 2 {
                    let target = &previous[0];
                    let op = &previous[1].token;

                    // Get the value after the operator
                    let value_lexers = if previous.len() > 2 {
                        (&previous[2..]).to_vec()
                    } else {
                        eprintln!("SyntaxError: esperado valor após o operador de atribuição composto.");
                        return;
                    };

                    // Get current value of variable
                    let current_val = match run.get_var_per_name(target.literal.clone()) {
                        Ok(v) => v.get_value().to_string(false),
                        Err(_) => {
                            eprintln!("ReferenceError: variável '{}' não definida.", target.literal);
                            return;
                        }
                    };

                    // Get new value
                    let new_val = process_value(value_lexers).to_string(false);

                    // Build compound expression
                    let op_str = match op {
                        Tokens::PlusEqual => "+",
                        Tokens::MinusEqual => "-",
                        Tokens::TimesEqual => "*",
                        Tokens::DivideEqual => "/",
                        Tokens::ModulusEqual => "|",  // Using | for modulus
                        _ => return,
                    };

                    let expr = format!("{} {} {}", current_val, op_str, new_val);

                    match exec_rust(expr) {
                        Err(err) => {
                            eprintln!("RuntimeError: {}", err.message);
                        }
                        Ok(res) => {
                            // Update the variable
                            match run.get_var(target.clone()) {
                                Ok(v) => {
                                    if v.is_borrowed() {
                                        eprintln!("BorrowError: variável '{}' está emprestada.", target.literal);
                                        return;
                                    }
                                    if let Err(err) = v.change_value(res) {
                                        eprintln!("TypeError: {}", err);
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
            "use_prop" => *val = run.get_var_prop(previous),
            "create_object" => *val = create_object(previous),
            "create_array" => *val = create_array(previous),
            "create_tuple" => *val = create_tuple(previous),
            "get_index" => {
                if previous.len() < 3 {
                    eprintln!("SyntaxError: esperado array[índice]");
                    return;
                }
                let arr_name = &previous[0];
                let index_lexers = previous[2..previous.len().saturating_sub(1)].to_vec();
                let index_val = if index_lexers.len() == 1 {
                    process_value(index_lexers)
                } else {
                    let mut v: Box<dyn Validator> = Box::new(String::new());
                    exec(&mut index_lexers.clone(), &mut v);
                    v.valid().1
                };
                let idx: usize = match index_val {
                    ValueData::Int(i) => i as usize,
                    ValueData::Float(f) => f as usize,
                    _ => match index_val.to_string(false).parse() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("TypeError: índice deve ser um número");
                            return;
                        }
                    },
                };

                match run.get_var(arr_name.clone()) {
                    Ok(var) => match var.get_value() {
                        ValueData::Vec(vec) => {
                            if idx < vec.len() {
                                *val = Box::new(vec.get_index(idx));
                            } else {
                                eprintln!("IndexError: índice {} fora dos limites", idx);
                            }
                        }
                        ValueData::Tuple(tup) => {
                            if idx < tup.len() {
                                *val = Box::new(tup.get_index(idx));
                            } else {
                                eprintln!("IndexError: índice {} fora dos limites", idx);
                            }
                        }
                        _ => {
                            eprintln!("TypeError: '{}' não é um array ou tupla", arr_name.literal);
                        }
                    },
                    Err(_) => {
                        eprintln!("ReferenceError: variável '{}' não definida", arr_name.literal);
                    }
                }
            }
            "set_index" => {
                if previous.len() < 4 {
                    eprintln!("SyntaxError: esperado array[índice] = valor");
                    return;
                }
                let arr_name = &previous[0];
                let close_bracket_pos = previous.iter().position(|l| l.token == Tokens::RightBracket).unwrap_or(previous.len() - 1);
                let index_lexers = previous[2..close_bracket_pos].to_vec();
                let index_val = if index_lexers.len() == 1 {
                    process_value(index_lexers)
                } else {
                    let mut v: Box<dyn Validator> = Box::new(String::new());
                    exec(&mut index_lexers.clone(), &mut v);
                    v.valid().1
                };
                let idx: usize = match index_val {
                    ValueData::Int(i) => i as usize,
                    ValueData::Float(f) => f as usize,
                    _ => match index_val.to_string(false).parse() {
                        Ok(n) => n,
                        Err(_) => {
                            eprintln!("TypeError: índice deve ser um número");
                            return;
                        }
                    },
                };
                let value_lexers = previous[close_bracket_pos + 2..].to_vec();
                let new_val = process_value(value_lexers);

                match run.get_var(arr_name.clone()) {
                    Ok(var) => match var.get_value() {
                        ValueData::Vec(mut vec) => {
                            if idx < vec.len() {
                                vec.set_index(idx, new_val);
                                if let Err(e) = var.change_value(ValueData::Vec(vec)) {
                                    eprintln!("TypeError: {}", e);
                                }
                            } else {
                                eprintln!("IndexError: índice {} fora dos limites", idx);
                            }
                        }
                        _ => {
                            eprintln!("TypeError: '{}' não é um array", arr_name.literal);
                        }
                    },
                    Err(_) => {
                        eprintln!("ReferenceError: variável '{}' não definida", arr_name.literal);
                    }
                }
            }
            "value_float" => *val = def_value_float(previous),
            "use_val" => {
                let mut value = String::new();

                for lex in &previous {
                    let sub =
                        remove_quoted_str(process_value(vec![lex.clone()]).to_string(false));
                    value.push_str(&sub);
                }

                value = put_quoted_str(value);
                *val = Box::new(ValueData::String(value));
            }
            "block_if" => {
                if let Some(open_idx) = previous.iter().position(|l| l.token == Tokens::LeftBrace) {
                    let cond_tokens = previous[1..open_idx].to_vec();
                    let body_tokens = previous[open_idx + 1..previous.len().saturating_sub(1)].to_vec();

                    let cond_res = exec_cond(cond_tokens);
                    let (_, val) = cond_res.valid();
                    if val.literal() == "true" {
                        exec_block(body_tokens);
                    }
                }
            }
            "break" => {
                let run = get_runtime();
                run.control_flow = crate::aly::ControlFlow::Break;
            }
            "continue" => {
                let run = get_runtime();
                run.control_flow = crate::aly::ControlFlow::Continue;
            }
            "block_loop" => {
                if let Some(open_idx) = previous.iter().position(|l| l.token == Tokens::LeftBrace) {
                    let header_tokens = previous[1..open_idx].to_vec();
                    let body_tokens = previous[open_idx + 1..previous.len().saturating_sub(1)].to_vec();

                    let mut init_tokens: Vec<Lexer> = vec![];
                    let mut cond_tokens: Vec<Lexer> = vec![];
                    let mut update_tokens: Vec<Lexer> = vec![];

                    let semi_positions: Vec<usize> = header_tokens
                        .iter()
                        .enumerate()
                        .filter(|(_, l)| l.literal == ";")
                        .map(|(i, _)| i)
                        .collect();

                    if semi_positions.len() >= 2 {
                        init_tokens = header_tokens[..semi_positions[0]].to_vec();
                        cond_tokens = header_tokens[semi_positions[0] + 1..semi_positions[1]].to_vec();
                        update_tokens = header_tokens[semi_positions[1] + 1..].to_vec();
                    } else {
                        cond_tokens = header_tokens.clone();
                    }

                    if !init_tokens.is_empty() {
                        let mut _val: Box<dyn Validator> = Box::new(String::new());
                        exec(&mut init_tokens, &mut _val);
                    }

                    let start_time = Instant::now();
                    let timeout = std::time::Duration::from_secs(30);
                    while start_time.elapsed() < timeout {
                        if !cond_tokens.is_empty() {
                            let cond_res = exec_cond(cond_tokens.clone());
                            let (_, val) = cond_res.valid();
                            if val.literal() != "true" {
                                break;
                            }
                        }
                        exec_block(body_tokens.clone());
                        
                        let run = get_runtime();
                        if run.control_flow == crate::aly::ControlFlow::Break {
                            run.control_flow = crate::aly::ControlFlow::None;
                            break;
                        } else if run.control_flow == crate::aly::ControlFlow::Continue {
                            run.control_flow = crate::aly::ControlFlow::None;
                        } else if run.control_flow != crate::aly::ControlFlow::None {
                            break;
                        }

                        if !update_tokens.is_empty() {
                            let mut _val: Box<dyn Validator> = Box::new(String::new());
                            exec(&mut update_tokens.clone(), &mut _val);
                        }
                    }
                }
            }
            "block_do" => {
                if let Some(open_idx) = previous.iter().position(|l| l.token == Tokens::LeftBrace) {
                    let mut brace_depth = 0;
                    let mut close_idx = None;
                    for (i, l) in previous.iter().enumerate().skip(open_idx) {
                        if l.token == Tokens::LeftBrace {
                            brace_depth += 1;
                        } else if l.token == Tokens::RightBrace {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                close_idx = Some(i);
                                break;
                            }
                        }
                    }
                    if let Some(close_idx) = close_idx {
                        let body_tokens = previous[open_idx + 1..close_idx].to_vec();
                        if close_idx + 1 < previous.len() && previous[close_idx + 1].token == Tokens::Loop {
                            let cond_tokens = previous[close_idx + 2..].to_vec();
                            
                            let start_time = Instant::now();
                            let timeout = std::time::Duration::from_secs(30);
                            while start_time.elapsed() < timeout {
                                exec_block(body_tokens.clone());
                                
                                let run = get_runtime();
                                if run.control_flow == crate::aly::ControlFlow::Break {
                                    run.control_flow = crate::aly::ControlFlow::None;
                                    break;
                                } else if run.control_flow == crate::aly::ControlFlow::Continue {
                                    run.control_flow = crate::aly::ControlFlow::None;
                                } else if run.control_flow != crate::aly::ControlFlow::None {
                                    break;
                                }

                                let cond_res = exec_cond(cond_tokens.clone());
                                let (_, val) = cond_res.valid();
                                if val.literal() != "true" {
                                    break;
                                }
                            }
                        } else {
                            eprintln!("SyntaxError: esperado 'loop' após bloco 'do'.");
                        }
                    }
                }
            }
            "block_foreach" => {
                if let Some(open_idx) = previous.iter().position(|l| l.token == Tokens::LeftBrace) {
                    let header_tokens = previous[1..open_idx].to_vec();
                    let body_tokens = previous[open_idx + 1..previous.len().saturating_sub(1)].to_vec();

                    if header_tokens.len() >= 3 && header_tokens[1].token == Tokens::In {
                        let item_name = header_tokens[0].literal.clone();
                        let col_expr_tokens = header_tokens[2..].to_vec();
                        
                        let col_res = process_value(col_expr_tokens);
                        
                        match col_res {
                            ValueData::Vec(vec) => {
                                let run = get_runtime();
                                
                                for val_data in vec.get_elements() {
                                    run.create_variable(vec![
                                        Lexer::new(Tokens::Let, "let".to_owned(), header_tokens[0].line),
                                        Lexer::new(Tokens::Identifier, item_name.clone(), header_tokens[0].line),
                                        Lexer::new(Tokens::Identifier, "=".to_owned(), header_tokens[0].line),
                                        Lexer::new(Tokens::Value, "None".to_owned(), header_tokens[0].line),
                                    ]);
                                    if let Ok(var) = run.get_var_per_name(item_name.clone()) {
                                        let _ = var.change_value(val_data.clone());
                                    }
                                    
                                    exec_block(body_tokens.clone());
                                    
                                    let run = get_runtime();
                                    if run.control_flow == crate::aly::ControlFlow::Break {
                                        run.control_flow = crate::aly::ControlFlow::None;
                                        break;
                                    } else if run.control_flow == crate::aly::ControlFlow::Continue {
                                        run.control_flow = crate::aly::ControlFlow::None;
                                    } else if run.control_flow != crate::aly::ControlFlow::None {
                                        break;
                                    }
                                }
                            }
                            _ => {
                                eprintln!("TypeError: '{}' não é iterável.", header_tokens[2..].iter().map(|l| l.literal.clone()).collect::<Vec<_>>().join(" "));
                            }
                        }
                    } else {
                        eprintln!("SyntaxError: formato inválido do foreach. Esperado: 'foreach item in colecao'.");
                    }
                }
            }
            "plugin_import" => {
                // plugin import "./path.so" as pluginname
                if previous.len() < 3 {
                    eprintln!("SyntaxError: esperado 'plugin import \"caminho\"'");
                    return;
                }

                let is_import = previous.get(1).map(|l| l.literal.as_str() == "import").unwrap_or(false);
                if !is_import {
                    eprintln!("SyntaxError: esperado 'import' após 'plugin'");
                    return;
                }

                let quoted_path = &previous[2].literal;
                let path = remove_quoted_str(quoted_path.clone());

                // Determine namespace: after optional "as" keyword
                let namespace = if previous.len() >= 5
                    && previous[3].literal == "as"
                {
                    previous[4].literal.clone()
                } else {
                    std::path::Path::new(&path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("plugin")
                        .to_string()
                };

                match load_plugin(&path, &namespace) {
                    Ok(funcs) => {
                        let run = get_runtime();
                        // Register each plugin function as namespace.funcname
                        for func_name in &funcs {
                            let var_name = format!("{}.{}", namespace, func_name);
                            run.register_plugin_function(
                                var_name,
                                namespace.clone(),
                                func_name.clone(),
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("ImportError: {}", e);
                    }
                }
            }
            "dec_fun" => {
                let is_anon = previous.len() >= 2 && previous[1].token == Tokens::LeftParenthesis;
                let func_val = ValueData::Function(previous.clone());
                if !is_anon {
                    let name = previous[1].literal.clone();
                    let run = get_runtime();
                    run.register_function(name, func_val.clone());
                }
                *val = Box::new(func_val);
            }
            "schema_dec" => {
                let run = get_runtime();
                if let Err(e) = run.register_schema(previous) {
                    eprintln!("TypeError: {}", e);
                }
            }
            "type_dec" => {
                let run = get_runtime();
                // type Name = Target
                if previous.len() >= 4 && previous[2].literal == "=" {
                    let alias_name = previous[1].literal.clone();
                    let target_type_str = previous[3].literal.clone();
                    
                    let target_type = match target_type_str.as_str() {
                        "int" => Type::Int,
                        "float" => Type::Float,
                        "string" => Type::String,
                        "boolean" => Type::Bool,
                        "vector" => Type::Vec,
                        _ => Type::None,
                    };
                    
                    run.register_type_alias(alias_name, target_type);
                }
            }
            "new_instance" => {
                let run = get_runtime();
                *val = run.create_instance(previous);
            }
            _ => {
                // No recognised action — silently skip.
            }
        };

        lexers.clear();
    }

    fn exec_block(body_tokens: Vec<Lexer>) {
        let mut stmt: Vec<Lexer> = vec![];
        let mut depth = 0i32;

        for lex in body_tokens {
            if get_runtime().control_flow != crate::aly::ControlFlow::None {
                break;
            }

            let starts_with_block = !stmt.is_empty()
                && (stmt[0].token == Tokens::Loop
                    || stmt[0].token == Tokens::If
                    || stmt[0].token == Tokens::Do
                    || stmt[0].token == Tokens::Foreach
                    || stmt[0].token == Tokens::Schema);

            let is_new_stmt = depth == 0 && !stmt.is_empty() && (
                (!starts_with_block && (
                    lex.token == Tokens::Let
                    || lex.token == Tokens::Const
                    || lex.token == Tokens::If
                    || lex.token == Tokens::Loop
                    || lex.token == Tokens::Do
                    || lex.token == Tokens::Foreach
                    || lex.token == Tokens::Break
                    || lex.token == Tokens::Continue
                ))
                || lex.line != stmt.last().unwrap().line
                || stmt.last().unwrap().token == Tokens::RightBrace
                || (stmt.last().unwrap().token == Tokens::RightParenthesis
                    && lex.token != Tokens::LeftBrace
                    && lex.token != Tokens::RightParenthesis
                    && lex.token != Tokens::Comma
                    && lex.token != Tokens::Dot
                    && !matches!(lex.token, Tokens::Identifier | Tokens::Reference | Tokens::Value)
                    && !is_var_declaration(lex.token.clone())
                    && !is_const_declaration(lex.token.clone())
                    && lex.token != Tokens::If
                    && lex.token != Tokens::Loop
                    && lex.token != Tokens::Do
                    && lex.token != Tokens::Foreach
                    && lex.token != Tokens::Try)
            );

            if is_new_stmt {
                let mut val: Box<dyn Validator> = Box::new(String::new());
                exec(&mut stmt, &mut val);
                stmt.clear();
                
                if get_runtime().control_flow != crate::aly::ControlFlow::None {
                    break;
                }
            }

            if is_opened(lex.token.clone()) {
                depth += 1;
            } else if is_close(lex.token.clone()) {
                depth -= 1;
            }

            stmt.push(lex);
        }

        if !stmt.is_empty() && get_runtime().control_flow == crate::aly::ControlFlow::None {
            let mut val: Box<dyn Validator> = Box::new(String::new());
            exec(&mut stmt, &mut val);
        }
    }

    pub fn run_block(body_tokens: Vec<Lexer>) {
        exec_block(body_tokens);
    }

}

pub use interpreter::*;