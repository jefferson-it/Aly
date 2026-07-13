use crate::compiler::ast::*;
use crate::lexer::Lexer;
use crate::runtime::parser::tokenize_line;
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
    Program { stmts: ast_stmts }
}

fn group_statements(source: &str) -> Vec<Vec<Lexer>> {
    let lines: Vec<&str> = source.split('\n').collect();
    let mut statements = Vec::new();
    let mut current: Vec<Lexer> = Vec::new();
    let mut depth: i32 = 0;
    let mut in_str: Tokens = Tokens::None;
    let mut comment_multi = false;

    for line in &lines {
        let mut exp = String::new();
        let mut previous = "";

        for letter in line.split("") {
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

        let tokens = tokenize_line_preserve_semicolons(exp.trim());

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

    for tok in tokens {
        match tok.token {
            Tokens::LeftParenthesis | Tokens::LeftBrace | Tokens::LeftBracket => depth += 1,
            Tokens::RightParenthesis | Tokens::RightBrace | Tokens::RightBracket => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {}
        }

        if tok.literal == ";" && depth == 0 {
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

fn tokenize_line_preserve_semicolons(line: &str) -> Vec<Lexer> {
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
            0,
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
        Tokens::Match => Some(parse_match(tokens)),
        Tokens::Try => Some(parse_try(tokens)),
        Tokens::Throw => Some(parse_throw(tokens)),
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
    let name = tokens.remove(0).literal;
    if tokens.is_empty() || tokens[0].token != Tokens::Identifier {
        return Stmt::Let { name, init: None };
    }
    tokens.remove(0);
    let init = Some(parse_expression(tokens));
    Stmt::Let { name, init }
}

fn parse_const(tokens: &mut Vec<Lexer>) -> Stmt {
    tokens.remove(0);
    if tokens.is_empty() {
        eprintln!("SyntaxError: esperado nome de constante após 'const'.");
        return Stmt::Expr(Expr::None);
    }
    let name = tokens.remove(0).literal;
    if tokens.is_empty() || tokens[0].token != Tokens::Identifier {
        eprintln!("SyntaxError: esperado '=' após o nome da constante '{}'.", name);
        return Stmt::Expr(Expr::None);
    }
    tokens.remove(0);
    let init = parse_expression(tokens);
    Stmt::Const { name, init }
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

    let has_semicolons = tokens.iter().any(|t| t.literal == ";");

    if has_semicolons {
        let init = if tokens.is_empty() {
            None
        } else if tokens[0].token == Tokens::Let {
            Some(Box::new(parse_let(tokens)))
        } else if tokens[0].token != Tokens::Semicolon
            && tokens[0].token != Tokens::Reference
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
    while !tokens.is_empty() && tokens[0].token != Tokens::RightParenthesis {
        if tokens[0].token == Tokens::Comma {
            tokens.remove(0);
            continue;
        }
        params.push(tokens.remove(0).literal);
    }

    if !tokens.is_empty() {
        tokens.remove(0); // consume ')'
    }

    let body = parse_block(tokens);
    Stmt::Fun { name, params, body }
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
            while !tokens.is_empty()
                && tokens[0].token != Tokens::RightBrace
                && !(tokens[0].token == Tokens::Semicolon && depth == 1)
            {
                if tokens[0].token == Tokens::LeftBrace {
                    break;
                }
                stmt_tokens.push(tokens.remove(0));
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

    parse_primary(tokens)
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
            if lit.contains('.')
                && lit.chars().any(|c| c.is_ascii_digit())
                && !lit.starts_with('"')
                && !lit.starts_with('\'')
            {
                if let Ok(f) = lit.parse::<f64>() {
                    Expr::Float(f)
                } else {
                    Expr::Str(lit.clone())
                }
            } else if let Ok(i) = lit.parse::<i64>() {
                Expr::Int(i)
            } else if lit == "true" {
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
            } else {
                Expr::Var(tok.literal.clone())
            }
        }
        Tokens::LeftParenthesis => {
            tokens.remove(0);
            let expr = parse_expression(tokens);
            if !tokens.is_empty() && tokens[0].token == Tokens::RightParenthesis {
                tokens.remove(0);
            }
            expr
        }
        Tokens::LeftBracket => {
            tokens.remove(0);
            parse_array_literal(tokens)
        }
        Tokens::LeftBrace => {
            tokens.remove(0);
            parse_object_literal(tokens)
        }
        _ => {
            tokens.remove(0);
            Expr::Var(tok.literal.clone())
        }
    }
}

fn parse_value_token(lit: &str) -> Expr {
    let content = if lit.starts_with('"') && lit.ends_with('"') {
        &lit[1..lit.len() - 1]
    } else if lit.starts_with('\'') && lit.ends_with('\'') {
        &lit[1..lit.len() - 1]
    } else {
        return Expr::Str(lit.to_string());
    };

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
        Expr::Str(lit.to_string())
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
        "+" | "-" => 7,
        "*" | "/" | "|" | "%" => 8,
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
        _ => op,
    }
}
