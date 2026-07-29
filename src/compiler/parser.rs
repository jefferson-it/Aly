use crate::compiler::ast::*;
use crate::lexer::Lexer;
use crate::tokens::Tokens;

pub fn parse_program(source: &str) -> Program {
    let stmts = group_statements(source);
    let mut ast_stmts = Vec::new();
    for stmt_tokens in stmts {
        if stmt_tokens.is_empty() {
            continue;
        }
        if let Some(stmt) = parse_statement(&mut stmt_tokens.clone()) {
            ast_stmts.push(stmt);
        }
    }
    let stmts = desugar_program(ast_stmts);
    Program { stmts }
}

/// Desugar AST: expand Destructure into individual lets, etc.
fn desugar_program(stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut result = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Destructure { names, init, is_const, is_array } => {
                let tmp_name = format!("_dtmp");
                if is_const {
                    result.push(Stmt::Const { name: tmp_name.clone(), init, attrs: vec![] });
                } else {
                    result.push(Stmt::Let { name: tmp_name.clone(), init: Some(init), attrs: vec![] });
                }
                for (i, name) in names.iter().enumerate() {
                    let access: Expr = if is_array {
                        Expr::Index {
                            object: Box::new(Expr::Var(tmp_name.clone())),
                            index: Box::new(Expr::Int(i as i64)),
                        }
                    } else {
                        Expr::PropAccess {
                            object: Box::new(Expr::Var(tmp_name.clone())),
                            prop: name.clone(),
                        }
                    };
                    if is_const {
                        result.push(Stmt::Const { name: name.clone(), init: access, attrs: vec![] });
                    } else {
                        result.push(Stmt::Let { name: name.clone(), init: Some(access), attrs: vec![] });
                    }
                }
            }
            _ => result.push(stmt),
        }
    }
    result
}

fn group_statements(source: &str) -> Vec<Vec<Lexer>> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mut statements = Vec::new();
    let mut current: Vec<Lexer> = Vec::new();
    let mut depth: i32 = 0;
    let mut in_str: Tokens = Tokens::None;
    let mut comment_multi = false;

    let re_inc = regex::Regex::new(r"\b([a-zA-Z_]\w*)\+\+").unwrap();
    let re_dec = regex::Regex::new(r"\b([a-zA-Z_]\w*)\-\-").unwrap();

    let mut line_num = 1;
    for line in &lines {
        let processed = re_inc.replace_all(line, "$1 = $1 + 1");
        let processed = re_dec.replace_all(&processed, "$1 = $1 - 1");
        let mut exp = String::new();
        let mut previous = "";

        for letter in processed.split("") {
            exp.push_str(&crate::runtime::parser::letter_per_letter_public(
                letter, previous, &mut depth, &mut in_str,
            ));
            previous = letter;
        }

        exp = exp.replace("  ", " ");

        if depth < 0 {
            eprintln!(
                "SyntaxError: chave fechada sem correspondente de abertura no código fonte."
            );
            depth = 0;
        }

        let tokens = tokenize_line_preserve_semicolons(exp.trim(), line_num);
        line_num += 1;

        let mut filtered = Vec::new();
        for tok in &tokens {
            if tok.literal == "#" {
                break;
            }
            if tok.literal == "##" {
                comment_multi = !comment_multi;
                continue;
            }
            if comment_multi {
                continue;
            }
            filtered.push(tok.clone());
        }

        current.extend(filtered);

        if depth == 0 && !current.is_empty() {
            let stmt: Vec<Lexer> = current.drain(..).collect();
            for s in split_on_semicolons(&stmt) {
                if !s.is_empty() {
                    statements.push(s);
                }
            }
        }
    }

    if !current.is_empty() {
        for s in split_on_semicolons(&current) {
            if !s.is_empty() {
                statements.push(s);
            }
        }
    }

    statements
}

fn split_on_semicolons(tokens: &[Lexer]) -> Vec<Vec<Lexer>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut depth = 0;
    let mut in_loop_header = false;

    for tok in tokens {
        if tok.token == Tokens::Loop {
            in_loop_header = true;
        }

        match tok.token {
            Tokens::LeftParenthesis | Tokens::LeftBracket => depth += 1,
            Tokens::LeftBrace => {
                depth += 1;
                in_loop_header = false;
            }
            Tokens::RightParenthesis | Tokens::RightBrace | Tokens::RightBracket => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {}
        }

        if tok.literal == ";" && depth == 0 && !in_loop_header {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }
        } else {
            current.push(tok.clone());
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn tokenize_line_preserve_semicolons(line: &str, line_num: i32) -> Vec<Lexer> {
    let mut exp = String::new();
    let mut previous = "";
    let mut to_end = 0;
    let mut is_str = Tokens::None;

    for letter in line.split("") {
        exp.push_str(&crate::runtime::parser::letter_per_letter_public(
            letter, previous, &mut to_end, &mut is_str,
        ));
        previous = letter;
    }

    exp = exp.replace("  ", " ");

    let re = match regex::Regex::new(r#"("[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*'|\S+)"#) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };

    let expressions: Vec<String> = re
        .find_iter(exp.trim())
        .map(|m| m.as_str().to_string())
        .collect();

    let mut lexers = Vec::new();
    for expression in expressions {
        if expression.trim() == "#" {
            break;
        }
        lexers.push(Lexer::new(
            crate::tokens::get_token(expression.clone()),
            expression,
            line_num,
        ));
    }
    lexers
}

fn parse_statement(tokens: &mut Vec<Lexer>) -> Option<Stmt> {
    if tokens.is_empty() {
        return None;
    }

    match tokens[0].token {
        Tokens::Let => Some(parse_let(tokens)),
        Tokens::Const => Some(parse_const(tokens)),
        Tokens::If => Some(parse_if(tokens)),
        Tokens::Loop => Some(parse_loop(tokens)),
        Tokens::Do => Some(parse_do_loop(tokens)),
        Tokens::Fun => Some(parse_fun(tokens)),
        Tokens::Return => Some(parse_return(tokens)),
        Tokens::Break => Some(parse_break(tokens)),
        Tokens::Continue => Some(parse_continue(tokens)),
        Tokens::Foreach => Some(parse_foreach(tokens)),
        Tokens::Match => Some(parse_match(tokens)),
        Tokens::Try => Some(parse_try(tokens)),
        Tokens::Throw => Some(parse_throw(tokens)),
        Tokens::Lazy => Some(parse_lazy(tokens)),
        Tokens::Macro => Some(parse_macro_def(tokens)),
        Tokens::Coroutine => Some(parse_coroutine(tokens)),
        Tokens::Yield => Some(parse_yield(tokens)),
        Tokens::At => Some(parse_attr_stmt(tokens)),
        _ => {
            if tokens.len() >= 4
                && tokens[0].token == Tokens::Reference
                && is_math_op(&tokens[1].token)
                && tokens[2].token == Tokens::Identifier
            {
                Some(parse_compound_assign(tokens))
            } else {
                Some(Stmt::Expr(parse_expression(tokens)))
            }
        }
    }
}

fn parse_let(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome de variável após 'let'.");
        return Stmt::Expr(Expr::None);
    }

    // Detect destructuring: let [a, b] = expr  or  let {x, y} = expr
    if tokens[0].token == Tokens::LeftBracket || tokens[0].token == Tokens::LeftBrace {
        return parse_destructure(tokens, false);
    }

    // Parse attribute list if present (handle: let @attr name = expr)
    let attrs = parse_attrs_ahead(tokens);

    let name = tokens.remove(0).literal;
    if tokens.is_empty() || (tokens[0].token != Tokens::Identifier && tokens[0].token != Tokens::Reference && tokens[0].literal != "=") {
        return Stmt::Let { name, init: None, attrs };
    }
    tokens.remove(0);
    let init = Some(parse_expression(tokens));
    Stmt::Let { name, init, attrs }
}

fn parse_const(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome de constante após 'const'.");
        return Stmt::Expr(Expr::None);
    }

    // Handle const destructuring
    if tokens[0].token == Tokens::LeftBracket || tokens[0].token == Tokens::LeftBrace {
        return parse_destructure(tokens, true);
    }

    let attrs = parse_attrs_ahead(tokens);

    let name = tokens.remove(0).literal;
    if tokens.is_empty() || (tokens[0].token != Tokens::Identifier && tokens[0].token != Tokens::Reference && tokens[0].literal != "=") {
        eprintln!("SyntaxError: esperado '=' após o nome da constante '{}'.", name);
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);
    let init = parse_expression(tokens);
    Stmt::Const { name, init, attrs }
}

fn parse_if(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    let condition = parse_condition_expr(tokens);
    let then_body = parse_block(tokens);

    let mut elifs = Vec::new();
    while !tokens.is_empty() && tokens[0].token == Tokens::Elif {
        tokens.remove(0);
        let elif_cond = parse_condition_expr(tokens);
        let elif_body = parse_block(tokens);
        elifs.push((elif_cond, elif_body));
    }

    let else_body = if !tokens.is_empty() && tokens[0].token == Tokens::Else {
        tokens.remove(0);
        Some(parse_block(tokens))
    } else {
        None
    };

    Stmt::If {
        condition,
        then_body,
        elifs,
        else_body,
    }
}

fn parse_loop(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);

    let has_semicolons = tokens
        .iter()
        .take_while(|t| t.token != Tokens::LeftBrace)
        .any(|t| t.literal == ";");

    if has_semicolons {
        let init = if tokens.is_empty() {
            None
        } else if tokens[0].token == Tokens::Let {
            Some(Box::new(parse_let(tokens)))
        } else if tokens[0].token != Tokens::Semicolon
            && tokens[0].token == Tokens::Reference
        {
            let name = tokens.remove(0).literal;
            if !tokens.is_empty() && tokens[0].token == Tokens::Identifier {
                tokens.remove(0);
                let val = parse_expression(tokens);
                Some(Box::new(Stmt::Expr(Expr::BinOp {
                    left: Box::new(Expr::Var(name)),
                    op: "=".to_string(),
                    right: Box::new(val),
                })))
            } else {
                None
            }
        } else {
            None
        };

        if !tokens.is_empty() && tokens[0].token == Tokens::Semicolon {
            tokens.remove(0);
        }

        let condition = if tokens.is_empty() || tokens[0].token == Tokens::Semicolon {
            None
        } else {
            let cond_tokens: Vec<Lexer> = tokens
                .iter()
                .take_while(|t| t.literal != ";")
                .cloned()
                .collect();
            for _ in &cond_tokens {
                tokens.remove(0);
            }
            Some(parse_expression(&mut cond_tokens.clone()))
        };

        if !tokens.is_empty() && tokens[0].token == Tokens::Semicolon {
            tokens.remove(0);
        }

        let update = if tokens.is_empty() || tokens[0].token == Tokens::LeftBrace {
            None
        } else {
            Some(parse_expression(tokens))
        };

        let body = parse_block(tokens);
        Stmt::For {
            init,
            condition,
            update,
            body,
        }
    } else {
        let condition = parse_condition_expr(tokens);
        let condition = if matches!(condition, Expr::None) {
            Expr::Bool(true)
        } else {
            condition
        };
        let body = parse_block(tokens);
        Stmt::While { condition, body }
    }
}

fn parse_do_loop(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    let body = parse_block(tokens);

    if tokens.is_empty() || tokens[0].token != Tokens::Loop {
        eprintln!("SyntaxError: esperado 'loop' após bloco 'do'.");
        return Stmt::DoWhile {
            body,
            condition: Expr::Bool(false),
        };
    }
    tokens.remove(0);

    let condition = parse_condition_expr(tokens);
    Stmt::DoWhile { body, condition }
}

fn parse_fun(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);

    let attrs = if !tokens.is_empty() && tokens[0].token == Tokens::At {
        parse_attrs(tokens)
    } else {
        Vec::new()
    };

    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome de função após 'fun'.");
        return Stmt::Expr(Expr::None);
    }
    let name = tokens.remove(0).literal;

    if tokens.is_empty() || tokens[0].token != Tokens::LeftParenthesis {
        eprintln!(
            "SyntaxError: esperado '(' após o nome da função '{}'.",
            name
        );
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);

    let mut params = Vec::new();
    let mut defaults = Vec::new();
    let mut variadic = None;

    while !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
        if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
            continue;
        }
        if tokens[0].literal == "..." {
            tokens.remove(0);
            let v_name = if !tokens.is_empty() {
                tokens.remove(0).literal
            } else {
                "args".to_string()
            };
            variadic = Some(v_name.clone());
            params.push(v_name);
            defaults.push(None);
            break;
        }

        let p_name = tokens.remove(0).literal;
        let mut def_val = None;
        if !tokens.is_empty() && tokens[0].token == Tokens::Identifier && tokens[0].literal == "=" {
            tokens.remove(0); // consume '='
            let mut val_tokens = Vec::new();
            let mut depth = 0;
            while !tokens.is_empty() && (depth > 0 || (tokens[0].token != Tokens::Comma && tokens[0].token != Tokens::RightParenthesis)) {
                if tokens[0].token == Tokens::LeftParenthesis || tokens[0].token == Tokens::LeftBracket || tokens[0].token == Tokens::LeftBrace {
                    depth += 1;
                } else if tokens[0].token == Tokens::RightParenthesis || tokens[0].token == Tokens::RightBracket || tokens[0].token == Tokens::RightBrace {
                    if depth > 0 { depth -= 1; }
                }
                val_tokens.push(tokens.remove(0));
            }
            if !val_tokens.is_empty() {
                def_val = Some(parse_expression(&mut val_tokens));
            }
        }
        params.push(p_name);
        defaults.push(def_val);
    }

    if !tokens.is_empty() {
        tokens.remove(0); // consume ')'
    }

    let body = parse_block(tokens);
    Stmt::Fun { name, params, defaults, variadic, body, attrs }
}

fn parse_return(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() || tokens[0].token == Tokens::RightBrace {
        Stmt::Return(None)
    } else {
        let expr = parse_expression(tokens);
        Stmt::Return(Some(expr))
    }
}

fn parse_break(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    Stmt::Break
}

fn parse_continue(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    Stmt::Continue
}

fn parse_foreach(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0); // consume 'foreach'
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado identificador após 'foreach'.");
        return Stmt::Expr(Expr::None);
    }
    let item = if tokens[0].token == Tokens::Reference || tokens[0].token == Tokens::None {
        tokens.remove(0).literal
    } else {
        eprintln!("SyntaxError: esperado identificador para o item do foreach.");
        tokens.remove(0).literal
    };
    if tokens.is_empty() || tokens[0].token != Tokens::In {
        eprintln!("SyntaxError: esperado 'in' após identificador no foreach.");
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0); // consume 'in'
    let brace_idx = tokens.iter().position(|l| l.token == Tokens::LeftBrace);
    if brace_idx.is_none() {
        eprintln!("SyntaxError: esperado '{{' para iniciar o bloco do foreach.");
        return Stmt::Expr(Expr::None);
    }
    let brace_idx = brace_idx.unwrap();
    let mut collection_tokens = tokens.drain(..brace_idx).collect::<Vec<_>>();
    let collection = parse_expression(&mut collection_tokens);
    let body = parse_block(tokens);
    Stmt::Foreach { item, collection, body }
}

/// Parse `throw <expr>`
fn parse_throw(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0); // consume 'throw'
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado expressão após 'throw'.");
        return Stmt::Throw(Expr::None);
    }
    let expr = parse_expression(tokens);
    Stmt::Throw(expr)
}

/// Parse `try { … } catch(err) { … } finally { … }`
///
/// Both `catch` and `finally` are optional but at least one must be present
/// in valid Aly code. The parser is lenient and accepts either or both.
fn parse_try(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0); // consume 'try'
    let body = parse_block(tokens);

    // Optional catch
    let (catch_var, catch_body) = if !tokens.is_empty() && tokens[0].token == Tokens::Catch {
        tokens.remove(0); // consume 'catch'

        // Optional `(var)` binding
        let var = if !tokens.is_empty() && tokens[0].token == Tokens::LeftParenthesis {
            tokens.remove(0);
            let name = if !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
                Some(tokens.remove(0).literal)
            } else {
                None
            };
            if !tokens.is_empty() && tokens[0].token == Tokens::RightParenthesis {
                tokens.remove(0);
            }
            name
        } else {
            None
        };

        let body = parse_block(tokens);
        (var, body)
    } else {
        (None, Vec::new())
    };

    // Optional finally
    let finally_body = if !tokens.is_empty() && tokens[0].token == Tokens::Finally {
        tokens.remove(0);
        parse_block(tokens)
    } else {
        Vec::new()
    };

    Stmt::Try {
        body,
        catch_var,
        catch_body,
        finally_body,
    }
}

fn parse_match(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    let scrutinee = parse_expression(tokens);
    if tokens.is_empty() || tokens[0].token != Tokens::LeftBrace {
        eprintln!("SyntaxError: esperado '{{' após expressão 'match'.");
        return Stmt::Match {
            scrutinee,
            arms: Vec::new(),
        };
    }
    tokens.remove(0);

    let mut arms = Vec::new();
    while !tokens.is_empty() && tokens[0].token != Tokens::RightBrace {
        let mut patterns = Vec::new();
        let mut pattern_tokens = Vec::new();

        while !tokens.is_empty()
            && tokens[0].token != Tokens::Colon
            && tokens[0].token != Tokens::LeftBrace
        {
            if tokens[0].token == Tokens::Or {
                if !pattern_tokens.is_empty() {
                    patterns.push(parse_pattern(&mut pattern_tokens));
                    pattern_tokens.clear();
                }
                tokens.remove(0);
            } else if tokens[0].token == Tokens::Subtraction
                && tokens.len() > 1
                && tokens[1].token == Tokens::GreaterThan
            {
                if !pattern_tokens.is_empty() {
                    let lo = parse_expression(&mut pattern_tokens);
                    tokens.remove(0);
                    tokens.remove(0);
                    let hi = parse_expression(tokens);
                    patterns.push(Pattern::Range(lo, hi));
                    break;
                }
            } else {
                pattern_tokens.push(tokens.remove(0));
            }
        }

        if !pattern_tokens.is_empty() {
            let lit = pattern_tokens.remove(0);
            if lit.literal == "_" {
                patterns.push(Pattern::Wildcard);
            } else {
                patterns.push(Pattern::Literal(Expr::Var(lit.literal)));
            }
        }

        if patterns.is_empty() {
            patterns.push(Pattern::Wildcard);
        }

        let body = if !tokens.is_empty() && tokens[0].token == Tokens::Colon {
            tokens.remove(0);
            if !tokens.is_empty() && tokens[0].token == Tokens::LeftBrace {
                parse_block(tokens)
            } else {
                let mut single = Vec::new();
                while !tokens.is_empty()
                    && tokens[0].token != Tokens::Comma
                    && tokens[0].token != Tokens::RightBrace
                {
                    single.push(tokens.remove(0));
                }
                if !tokens.is_empty() && tokens[0].token == Tokens::Comma {
                    tokens.remove(0);
                }
                match parse_statement(&mut single) {
                    Some(s) => vec![s],
                    None => vec![],
                }
            }
        } else {
            if !tokens.is_empty() && tokens[0].token == Tokens::Comma {
                tokens.remove(0);
            }
            vec![]
        };

        if !tokens.is_empty() && tokens[0].token == Tokens::Comma {
            tokens.remove(0);
        }

        arms.push(MatchArm { patterns, body });
    }

    if !tokens.is_empty() {
        tokens.remove(0);
    }

    Stmt::Match { scrutinee, arms }
}

fn parse_pattern(tokens: &mut Vec<Lexer>) -> Pattern {
    if tokens.len() == 1 && tokens[0].literal == "_" {
        Pattern::Wildcard
    } else if tokens.len() == 3 && tokens[1].literal == ".." {
        Pattern::Range(
            Expr::Var(tokens[0].literal.clone()),
            Expr::Var(tokens[2].literal.clone()),
        )
    } else {
        Pattern::Literal(parse_expression(tokens))
    }
}

fn parse_block(tokens: &mut Vec<Lexer>) -> Vec<Stmt> {
    if tokens.is_empty() || tokens[0].token != Tokens::LeftBrace {
        eprintln!("SyntaxError: esperado '{{' para abrir bloco.");
        return Vec::new();
    }
    tokens.remove(0);

    let mut stmts = Vec::new();
    let mut depth = 1;

    while !tokens.is_empty() && depth > 0 {
        if tokens[0].token == Tokens::LeftBrace {
            depth += 1;
            let mut block_tokens = vec![tokens.remove(0)];
            while depth > 0 && !tokens.is_empty() {
                if tokens[0].token == Tokens::LeftBrace {
                    depth += 1;
                } else if tokens[0].token == Tokens::RightBrace {
                    depth -= 1;
                }
                block_tokens.push(tokens.remove(0));
            }
            if let Some(stmt) = parse_statement(&mut block_tokens) {
                stmts.push(stmt);
            }
        } else if tokens[0].token == Tokens::RightBrace {
            depth -= 1;
            if depth == 0 {
                tokens.remove(0);
            }
        } else {
            let mut stmt_tokens = Vec::new();
            let is_block_stmt = !tokens.is_empty() && matches!(
                tokens[0].token,
                Tokens::If | Tokens::Loop | Tokens::Do | Tokens::Fun | Tokens::Foreach
                | Tokens::Macro | Tokens::Coroutine
            );

            if is_block_stmt {
                let mut brace_depth = 0;
                let mut seen_brace = false;
                while !tokens.is_empty() {
                    let tok = &tokens[0];
                    if tok.token == Tokens::LeftBrace {
                        brace_depth += 1;
                        seen_brace = true;
                    } else if tok.token == Tokens::RightBrace {
                        brace_depth -= 1;
                    }
                    stmt_tokens.push(tokens.remove(0));
                    if seen_brace && brace_depth == 0 {
                        if !tokens.is_empty() && (tokens[0].token == Tokens::Elif || tokens[0].token == Tokens::Else) {
                            seen_brace = false;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                let mut p_depth = 0;
                let mut b_depth = 0;
                while !tokens.is_empty()
                    && tokens[0].token != Tokens::RightBrace
                    && !(tokens[0].token == Tokens::Semicolon && depth == 1)
                {
                    if tokens[0].token == Tokens::LeftBrace {
                        break;
                    }
                    if !stmt_tokens.is_empty()
                        && p_depth == 0
                        && b_depth == 0
                        && tokens[0].line != stmt_tokens.last().unwrap().line
                    {
                        break;
                    }
                    let t = &tokens[0].token;
                    if *t == Tokens::LeftParenthesis { p_depth += 1; }
                    else if *t == Tokens::RightParenthesis { p_depth -= 1; }
                    else if *t == Tokens::LeftBracket { b_depth += 1; }
                    else if *t == Tokens::RightBracket { b_depth -= 1; }

                    stmt_tokens.push(tokens.remove(0));
                }
            }

            if !tokens.is_empty() && tokens[0].token == Tokens::Semicolon {
                tokens.remove(0);
            }

            if !stmt_tokens.is_empty() {
                for sub_stmt in split_on_semicolons(&stmt_tokens) {
                    if !sub_stmt.is_empty() {
                        if let Some(stmt) = parse_statement(&mut sub_stmt.clone()) {
                            stmts.push(stmt);
                        }
                    }
                }
            }
        }
    }

    stmts
}

/// Parse `lazy name = expr`
fn parse_lazy(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome após 'lazy'.");
        return Stmt::Expr(Expr::None);
    }
    let name = tokens.remove(0).literal;
    if tokens.is_empty() || (tokens[0].token != Tokens::Identifier && tokens[0].literal != "=") {
        eprintln!("SyntaxError: esperado '=' após nome em lazy.");
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);
    let init = parse_expression(tokens);
    Stmt::Lazy { name, init }
}

/// Parse `macro name { pattern => body, ... }`
fn parse_macro_def(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome após 'macro'.");
        return Stmt::Expr(Expr::None);
    }
    let name = tokens.remove(0).literal;
    if tokens.is_empty() || tokens[0].token != Tokens::LeftBrace {
        eprintln!("SyntaxError: esperado '{{' após nome de macro.");
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);
    let mut patterns = Vec::new();
    let mut bodies = Vec::new();
    while !tokens.is_empty() && tokens[0].token != Tokens::RightBrace {
        let mut pat_tokens = Vec::new();
        while !tokens.is_empty() && tokens[0].token != Tokens::LeftBrace && tokens[0].token != Tokens::RightBrace {
            pat_tokens.push(tokens.remove(0));
        }
        let pat_expr = if pat_tokens.is_empty() {
            vec![]
        } else {
            vec![parse_expression(&mut pat_tokens)]
        };
        let body = if !tokens.is_empty() && tokens[0].token == Tokens::LeftBrace {
            parse_block(tokens)
        } else {
            let mut single = Vec::new();
            while !tokens.is_empty() && tokens[0].token != Tokens::Comma && tokens[0].token != Tokens::RightBrace {
                single.push(tokens.remove(0));
            }
            match parse_statement(&mut single) {
                Some(s) => vec![s],
                None => vec![],
            }
        };
        patterns.push(pat_expr);
        bodies.push(body);
        if !tokens.is_empty() && tokens[0].token == Tokens::Comma {
            tokens.remove(0);
        }
    }
    if !tokens.is_empty() && tokens[0].token == Tokens::RightBrace {
        tokens.remove(0);
    }
    Stmt::MacroDef { name, patterns, bodies }
}

/// Parse `coroutine name(params) { body }`
fn parse_coroutine(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome após 'coroutine'.");
        return Stmt::Expr(Expr::None);
    }
    let name = tokens.remove(0).literal;
    if tokens.is_empty() || tokens[0].token != Tokens::LeftParenthesis {
        eprintln!("SyntaxError: esperado '(' após nome de coroutine.");
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);
    let mut params = Vec::new();
    while !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
        if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
            continue;
        }
        params.push(tokens.remove(0).literal);
    }
    if !tokens.is_empty() {
        tokens.remove(0);
    }
    let body = parse_block(tokens);
    Stmt::Coroutine { name, params, body }
}

/// Parse `yield expr` or `yield`
fn parse_yield(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() || tokens[0].token == Tokens::RightBrace || tokens[0].token == Tokens::Semicolon {
        Stmt::Yield(None)
    } else {
        let expr = parse_expression(tokens);
        Stmt::Yield(Some(expr))
    }
}

/// Parse `let [a, b] = expr` or `let {x, y} = expr` 
/// Creates a Destructure AST node that will be desugared after parsing.
fn parse_destructure(tokens: &mut Vec<Lexer>, is_const: bool) -> Stmt {
    let is_array = tokens[0].token == Tokens::LeftBracket;
    tokens.remove(0);
    let mut names = Vec::new();
    while !tokens.is_empty() && tokens[0].token != if is_array { Tokens::RightBracket } else { Tokens::RightBrace } {
        if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
            continue;
        }
        if tokens[0].token == Tokens::At {
            tokens.remove(0);
            if !tokens.is_empty() {
                names.push(tokens.remove(0).literal);
            }
        } else {
            names.push(tokens.remove(0).literal);
        }
    }
    if !tokens.is_empty() {
        tokens.remove(0);
    }
    if tokens.is_empty() || (tokens[0].token != Tokens::Identifier && tokens[0].literal != "=") {
        eprintln!("SyntaxError: esperado '=' após padrão de destruct.");
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);
    let init = parse_expression(tokens);
    Stmt::Destructure { names, init, is_const, is_array }
}

/// Parse attribute list from current position: `@name(args) @name2 ...`
fn parse_attrs(tokens: &mut Vec<Lexer>) -> Vec<Attribute> {
    let mut attrs = Vec::new();
    while !tokens.is_empty() && tokens[0].token == Tokens::At {
        tokens.remove(0);
        let name = if !tokens.is_empty() {
            tokens.remove(0).literal
        } else {
            break;
        };
        let args = if !tokens.is_empty() && tokens[0].token == Tokens::LeftParenthesis {
            parse_call_args(tokens)
        } else {
            Vec::new()
        };
        attrs.push(Attribute { name, args });
    }
    attrs
}

/// Peek ahead: if next token is `@`, parse attributes, else return empty vec
fn parse_attrs_ahead(tokens: &mut Vec<Lexer>) -> Vec<Attribute> {
    if !tokens.is_empty() && tokens[0].token == Tokens::At {
        parse_attrs(tokens)
    } else {
        Vec::new()
    }
}

/// Handle a statement that starts with `@` (attribute annotation)
fn parse_attr_stmt(tokens: &mut Vec<Lexer>) -> Stmt {
    let attrs = parse_attrs(tokens);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado statement após atributos.");
        return Stmt::Expr(Expr::None);
    }
    // Apply attributes to the following statement
    let mut stmt = match parse_statement(tokens) {
        Some(s) => s,
        None => return Stmt::Expr(Expr::None),
    };
    // Attach attrs to the statement
    match &mut stmt {
        Stmt::Fun { attrs: fa, .. } => *fa = attrs,
        Stmt::Let { attrs: la, .. } => *la = attrs,
        Stmt::Const { attrs: ca, .. } => *ca = attrs,
        _ => {} // attributes on other statements are ignored
    }
    stmt
}

fn parse_compound_assign(tokens: &mut Vec<Lexer>) -> Stmt {
    let name = tokens.remove(0).literal;
    let op_token = tokens.remove(0);
    tokens.remove(0);
    let rhs = parse_expression(tokens);

    let op = match op_token.token {
        Tokens::Addition => "+",
        Tokens::Subtraction => "-",
        Tokens::Multiplication => "*",
        Tokens::Division => "/",
        _ => {
            eprintln!(
                "SyntaxError: operador composto desconhecido '{}'.",
                op_token.literal
            );
            "+"
        }
    };

    Stmt::Expr(Expr::BinOp {
        left: Box::new(Expr::Var(name.clone())),
        op: "=".to_string(),
        right: Box::new(Expr::BinOp {
            left: Box::new(Expr::Var(name)),
            op: op.to_string(),
            right: Box::new(rhs),
        }),
    })
}

fn parse_condition_expr(tokens: &mut Vec<Lexer>) -> Expr {
    let mut expr_tokens = Vec::new();
    while !tokens.is_empty()
        && tokens[0].token != Tokens::LeftBrace
        && tokens[0].token != Tokens::Colon
    {
        expr_tokens.push(tokens.remove(0));
    }
    parse_expression(&mut expr_tokens)
}

fn parse_expression(tokens: &mut Vec<Lexer>) -> Expr {
    parse_binary_expr(tokens, 0)
}

#[allow(non_snake_case)]
fn parse_binary_expr(tokens: &mut Vec<Lexer>, min_prec: u8) -> Expr {
    let mut left = parse_unary(tokens);

    while !tokens.is_empty() {
        let next = &tokens[0];
        let prec = get_precedence(&next.literal);
        if prec == 0 || prec < min_prec {
            break;
        }
        let op = tokens.remove(0).literal.clone();
        let right = parse_binary_expr(tokens, prec + 1);
        left = Expr::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    left
}

fn parse_unary(tokens: &mut Vec<Lexer>) -> Expr {
    if tokens.is_empty() {
        return Expr::None;
    }

    if tokens[0].token == Tokens::Subtraction {
        tokens.remove(0);
        let expr = parse_unary(tokens);
        return Expr::UnaryOp {
            op: "-".to_string(),
            expr: Box::new(expr),
        };
    }

    if tokens[0].token == Tokens::Not {
        tokens.remove(0);
        let expr = parse_unary(tokens);
        return Expr::UnaryOp {
            op: "!".to_string(),
            expr: Box::new(expr),
        };
    }

    if tokens[0].token == Tokens::BitwiseNot {
        tokens.remove(0);
        let expr = parse_unary(tokens);
        return Expr::UnaryOp {
            op: "~".to_string(),
            expr: Box::new(expr),
        };
    }

    if tokens[0].token == Tokens::Yield {
        tokens.remove(0);
        let expr = if tokens.is_empty() || tokens[0].token == Tokens::RightBrace || tokens[0].token == Tokens::Semicolon {
            Expr::None
        } else {
            parse_unary(tokens)
        };
        return Expr::YieldExpr(Box::new(expr));
    }

    let expr = parse_primary(tokens);
    parse_postfix(expr, tokens)
}

fn parse_primary(tokens: &mut Vec<Lexer>) -> Expr {
    if tokens.is_empty() {
        return Expr::None;
    }

    let tok = tokens[0].clone();

    match tok.token {
        Tokens::Value => {
            tokens.remove(0);
            let lit = &tok.literal;
            let clean = lit.replace('_', "");
            if clean.contains('.')
                && clean.chars().any(|c| c.is_ascii_digit())
                && !clean.starts_with('"')
                && !clean.starts_with('\'')
            {
                if let Ok(f) = clean.parse::<f64>() {
                    Expr::Float(f)
                } else {
                    Expr::Str(lit.clone())
                }
            } else if let Ok(i) = clean.parse::<i64>() {
                Expr::Int(i)
            } else if clean == "true" {
                Expr::Bool(true)
            } else if lit == "false" {
                Expr::Bool(false)
            } else if lit.starts_with('"') || lit.starts_with('\'') {
                parse_value_token(lit)
            } else {
                Expr::Var(lit.clone())
            }
        }
        Tokens::Reference => {
            tokens.remove(0);
            Expr::Var(tok.literal.clone())
        }
        Tokens::None => {
            tokens.remove(0);
            if tok.literal == "None" {
                Expr::None
            } else if tok.literal == "void" {
                Expr::Void
            } else {
                Expr::Var(tok.literal.clone())
            }
        }
        Tokens::LeftParenthesis => {
            tokens.remove(0); // consume '('
            if !tokens.is_empty() && tokens[0].token == Tokens::RightParenthesis {
                tokens.remove(0); // consume ')'
                return Expr::Tuple(vec![]);
            }
            let first_expr = parse_expression(tokens);
            if !tokens.is_empty() && tokens[0].token == Tokens::Comma {
                let mut elements = vec![first_expr];
                while !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
                    if tokens[0].token == Tokens::Comma {
                        tokens.remove(0);
                    }
                    if !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
                        elements.push(parse_expression(tokens));
                    }
                }
                if !tokens.is_empty() && tokens[0].token == Tokens::RightParenthesis {
                    tokens.remove(0);
                }
                Expr::Tuple(elements)
            } else {
                if !tokens.is_empty() && tokens[0].token == Tokens::RightParenthesis {
                    tokens.remove(0);
                }
                first_expr
            }
        }
        Tokens::LeftBracket => {
            tokens.remove(0);
            parse_array_literal(tokens)
        }
        Tokens::LeftBrace => {
            tokens.remove(0);
            parse_object_literal(tokens)
        }
        Tokens::Fun => {
            tokens.remove(0); // consume 'fun'
            if tokens.is_empty() || tokens[0].token != Tokens::LeftParenthesis {
                eprintln!("SyntaxError: esperado '(' após 'fun'.");
                return Expr::None;
            }
            tokens.remove(0); // consume '('
            let mut params = Vec::new();
            let mut defaults = Vec::new();
            let mut variadic = None;
            while !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
                if tokens[0].token == Tokens::Comma {
                    tokens.remove(0);
                    continue;
                }
                if tokens[0].literal == "..." {
                    tokens.remove(0);
                    let v_name = if !tokens.is_empty() {
                        tokens.remove(0).literal
                    } else {
                        "args".to_string()
                    };
                    variadic = Some(v_name.clone());
                    params.push(v_name);
                    defaults.push(None);
                    break;
                }
                let p_name = tokens.remove(0).literal;
                let mut def_val = None;
                if !tokens.is_empty() && tokens[0].token == Tokens::Identifier && tokens[0].literal == "=" {
                    tokens.remove(0); // consume '='
                    let mut val_tokens = Vec::new();
                    let mut depth = 0;
                    while !tokens.is_empty() && (depth > 0 || (tokens[0].token != Tokens::Comma && tokens[0].token != Tokens::RightParenthesis)) {
                        if tokens[0].token == Tokens::LeftParenthesis || tokens[0].token == Tokens::LeftBracket || tokens[0].token == Tokens::LeftBrace {
                            depth += 1;
                        } else if tokens[0].token == Tokens::RightParenthesis || tokens[0].token == Tokens::RightBracket || tokens[0].token == Tokens::RightBrace {
                            if depth > 0 { depth -= 1; }
                        }
                        val_tokens.push(tokens.remove(0));
                    }
                    if !val_tokens.is_empty() {
                        def_val = Some(parse_expression(&mut val_tokens));
                    }
                }
                params.push(p_name);
                defaults.push(def_val);
            }
            if !tokens.is_empty() {
                tokens.remove(0); // consume ')'
            }
            let body = parse_block(tokens);
            Expr::AnonymousFun { params, defaults, variadic, body }
        }
        _ => {
            tokens.remove(0);
            Expr::Var(tok.literal.clone())
        }
    }
}

fn parse_value_token(lit: &str) -> Expr {
    let is_single_quote = lit.starts_with('\'') && lit.ends_with('\'');
    let content = if lit.starts_with('"') && lit.ends_with('"') {
        &lit[1..lit.len() - 1]
    } else if is_single_quote {
        &lit[1..lit.len() - 1]
    } else {
        return Expr::Str(lit.to_string());
    };

    if is_single_quote {
        let mut chars = content.chars();
        if let Some(first) = chars.next() {
            if first == '\\' {
                if let Some(second) = chars.next() {
                    if chars.next().is_none() {
                        let c = match second {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '\\' => '\\',
                            '\'' => '\'',
                            '"' => '"',
                            _ => second,
                        };
                        return Expr::Char(c);
                    }
                }
            } else if chars.next().is_none() {
                return Expr::Char(first);
            }
        }
    }

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            if !current.is_empty() {
                parts.push(TemplatePart::Text(std::mem::take(&mut current)));
            }
            let mut var_name = String::new();
            while let Some(&nc) = chars.peek() {
                if nc.is_alphanumeric() || nc == '_' {
                    var_name.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            if var_name.is_empty() {
                current.push('$');
            } else {
                parts.push(TemplatePart::Var(var_name));
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        parts.push(TemplatePart::Text(current));
    }

    let has_vars = parts.iter().any(|p| matches!(p, TemplatePart::Var(_)));

    if has_vars {
        Expr::TemplateStr(parts)
    } else {
        Expr::Str(content.to_string())
    }
}

fn parse_call_args(tokens: &mut Vec<Lexer>) -> Vec<Expr> {
    if tokens.is_empty() || tokens[0].token != Tokens::LeftParenthesis {
        return Vec::new();
    }
    tokens.remove(0);

    let mut args = Vec::new();

    while !tokens.is_empty() {
        if tokens[0].token == Tokens::RightParenthesis {
            tokens.remove(0);
            break;
        } else if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
        } else {
            let mut arg_tokens = Vec::new();
            let mut inner_depth = 0;
            while !tokens.is_empty()
                && (inner_depth > 0
                    || (tokens[0].token != Tokens::Comma
                        && tokens[0].token != Tokens::RightParenthesis))
            {
                if tokens[0].token == Tokens::LeftParenthesis
                    || tokens[0].token == Tokens::LeftBracket
                    || tokens[0].token == Tokens::LeftBrace
                {
                    inner_depth += 1;
                } else if tokens[0].token == Tokens::RightParenthesis
                    || tokens[0].token == Tokens::RightBracket
                    || tokens[0].token == Tokens::RightBrace
                {
                    if inner_depth > 0 {
                        inner_depth -= 1;
                    }
                }
                arg_tokens.push(tokens.remove(0));
            }
            if !arg_tokens.is_empty() {
                args.push(parse_expression(&mut arg_tokens));
            }
        }
    }

    args
}

fn parse_array_literal(tokens: &mut Vec<Lexer>) -> Expr {
    let mut elements = Vec::new();
    let mut depth = 1;

    while !tokens.is_empty() && depth > 0 {
        if tokens[0].token == Tokens::LeftBracket {
            depth += 1;
            elements.push(parse_primary(tokens));
        } else if tokens[0].token == Tokens::RightBracket {
            depth -= 1;
            if depth == 0 {
                tokens.remove(0);
                break;
            }
            elements.push(parse_primary(tokens));
        } else if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
        } else {
            let mut elem_tokens = Vec::new();
            while !tokens.is_empty()
                && tokens[0].token != Tokens::Comma
                && tokens[0].token != Tokens::RightBracket
            {
                elem_tokens.push(tokens.remove(0));
            }
            if !elem_tokens.is_empty() {
                elements.push(parse_expression(&mut elem_tokens));
            }
        }
    }

    Expr::Array(elements)
}

fn parse_object_literal(tokens: &mut Vec<Lexer>) -> Expr {
    let mut entries = Vec::new();
    let mut depth = 1;

    while !tokens.is_empty() && depth > 0 {
        if tokens[0].token == Tokens::LeftBrace {
            depth += 1;
            entries.push((tokens.remove(0).literal.clone(), parse_primary(tokens)));
        } else if tokens[0].token == Tokens::RightBrace {
            depth -= 1;
            if depth == 0 {
                tokens.remove(0);
                break;
            }
        } else if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
        } else {
            let key = tokens.remove(0).literal.clone();
            if !tokens.is_empty() && tokens[0].token == Tokens::Colon {
                tokens.remove(0);
            }
            let mut val_tokens = Vec::new();
            while !tokens.is_empty()
                && tokens[0].token != Tokens::Comma
                && tokens[0].token != Tokens::RightBrace
            {
                val_tokens.push(tokens.remove(0));
            }
            let val = if val_tokens.is_empty() {
                Expr::None
            } else {
                parse_expression(&mut val_tokens)
            };
            entries.push((key, val));
        }
    }

    Expr::Object(entries)
}

fn parse_postfix(expr: Expr, tokens: &mut Vec<Lexer>) -> Expr {
    let mut result = expr;

    while !tokens.is_empty() {
        // Macro expansion: ident!(args)
        if tokens[0].literal == "!" && tokens.len() > 1 && tokens[1].token == Tokens::LeftParenthesis {
            if let Expr::Var(name) = &result {
                tokens.remove(0); // consume '!'
                let args = parse_call_args(tokens);
                result = Expr::MacroExpand { name: name.clone(), args };
                continue;
            }
        }

        if tokens[0].token == Tokens::Dot {
            tokens.remove(0);
            if tokens.is_empty() {
                break;
            }
            if tokens[0].token == Tokens::LeftParenthesis {
                let args = parse_call_args(tokens);
                result = Expr::Call {
                    function: Box::new(result),
                    args,
                };
            } else {
                let prop = tokens.remove(0).literal.clone();
                if !tokens.is_empty() && tokens[0].token == Tokens::LeftParenthesis {
                    let obj = Expr::PropAccess {
                        object: Box::new(result),
                        prop,
                    };
                    let args = parse_call_args(tokens);
                    result = Expr::Call {
                        function: Box::new(obj),
                        args,
                    };
                } else {
                    result = Expr::PropAccess {
                        object: Box::new(result),
                        prop,
                    };
                }
            }
        } else if tokens[0].token == Tokens::LeftBracket {
            tokens.remove(0);
            let idx = parse_expression(tokens);
            if !tokens.is_empty() && tokens[0].token == Tokens::RightBracket {
                tokens.remove(0);
            }
            result = Expr::Index {
                object: Box::new(result),
                index: Box::new(idx),
            };
        } else if tokens[0].token == Tokens::LeftParenthesis {
            let args = parse_call_args(tokens);
            result = Expr::Call {
                function: Box::new(result),
                args,
            };
        } else if tokens[0].token == Tokens::Percent {
            tokens.remove(0);
            result = Expr::Percent(Box::new(result));
        } else if tokens[0].token == Tokens::Addition
            && tokens.len() > 1
            && tokens[1].token == Tokens::Addition
        {
            tokens.remove(0);
            tokens.remove(0);
            result = Expr::BinOp {
                left: Box::new(result.clone()),
                op: "=".to_string(),
                right: Box::new(Expr::BinOp {
                    left: Box::new(result),
                    op: "+".to_string(),
                    right: Box::new(Expr::Int(1)),
                }),
            };
        } else if tokens[0].token == Tokens::Subtraction
            && tokens.len() > 1
            && tokens[1].token == Tokens::Subtraction
        {
            tokens.remove(0);
            tokens.remove(0);
            result = Expr::BinOp {
                left: Box::new(result.clone()),
                op: "=".to_string(),
                right: Box::new(Expr::BinOp {
                    left: Box::new(result),
                    op: "-".to_string(),
                    right: Box::new(Expr::Int(1)),
                }),
            };
        } else {
            break;
        }
    }

    result
}

fn is_math_op(tok: &Tokens) -> bool {
    matches!(
        tok,
        Tokens::Addition
            | Tokens::Subtraction
            | Tokens::Multiplication
            | Tokens::Division
            | Tokens::Modulus
            | Tokens::Percent
    )
}

pub fn get_precedence(op: &str) -> u8 {
    match op {
        "=" | "+=" | "-=" | "*=" | "/=" => 1,
        "or" | "OR" => 2,
        "and" | "AND" => 3,
        "xor" | "XOR" => 4,
        "eq" | "EQ" | "neq" | "NEQ" => 5,
        "lt" | "LT" | "gt" | "GT" | "lte" | "LTE" | "gte" | "GTE" => 6,
        "bor" => 7,
        "bxor" | "^" => 8,
        "band" | "&" => 9,
        "shl" | "shr" | "<<" | ">>" => 10,
        "+" | "-" => 11,
        "*" | "/" | "|" | "%" => 12,
        _ => 0,
    }
}

pub fn op_to_c(op: &str) -> &str {
    match op {
        "eq" | "EQ" => "==",
        "neq" | "NEQ" => "!=",
        "lt" | "LT" => "<",
        "lte" | "LTE" => "<=",
        "gt" | "GT" => ">",
        "gte" | "GTE" => ">=",
        "and" | "AND" => "&&",
        "or" | "OR" => "||",
        "xor" | "XOR" => "^",
        "not" | "NOT" => "!",
        "|" => "%",
        "band" | "&" => "&",
        "bor" => "|",
        "bxor" | "^" => "^",
        "bnot" | "~" => "~",
        "shl" | "<<" => "<<",
        "shr" | ">>" => ">>",
        _ => op,
    }
}
