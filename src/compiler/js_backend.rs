use crate::compiler::CompileOptions;

pub fn compile_to_js(source: &str, _options: &CompileOptions) -> Result<String, String> {
    let program = crate::compiler::parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    
    let mut js = String::new();
    js.push_str("// Generated from Aly source\n");
    js.push_str("const aly_runtime = require('./runtime_aly.js');\n\n");
    
    // Generate JS from chunk bytecode
    js.push_str(&generate_js_from_chunk(&chunk));
    
    js.push_str("\n\n// Entry point\n");
    js.push_str("aly_main();\n");
    
    Ok(js)
}

fn generate_js_from_chunk(chunk: &crate::vm::chunk::Chunk) -> String {
    let mut js = String::new();
    
    // Stack simulation
    js.push_str("let stack = [];\n");
    js.push_str("let locals = [];\n");
    js.push_str("let globals = {};\n\n");
    
    // Function wrapper
    js.push_str("function aly_main() {\n");
    js.push_str("    try {\n");
    
    // Generate code for each instruction
    let mut ip = 0;
    while ip < chunk.code.len() {
        let insn = chunk.code[ip];
        let op = crate::vm::opcode::decode_op(insn);
        let a = crate::vm::opcode::decode_a(insn);
        let b = crate::vm::opcode::decode_b(insn);
        let width = crate::vm::opcode::opcode_width(op);
        
        let code = match op {
            crate::vm::opcode::OpCode::Nil => "    stack.push(null);\n".to_string(),
            crate::vm::opcode::OpCode::Void => "    stack.push(undefined);\n".to_string(),
            crate::vm::opcode::OpCode::True => "    stack.push(true);\n".to_string(),
            crate::vm::opcode::OpCode::False => "    stack.push(false);\n".to_string(),
            crate::vm::opcode::OpCode::LoadInt => format!("    stack.push({});\n", a as i64),
            crate::vm::opcode::OpCode::Const => {
                if (a as usize) < chunk.constants.len() {
                    format!("    stack.push({});\n", constant_to_js(&chunk.constants[a as usize]))
                } else {
                    "    stack.push(null);\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::Add => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a + b); }\n".to_string(),
            crate::vm::opcode::OpCode::Subtract => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a - b); }\n".to_string(),
            crate::vm::opcode::OpCode::Multiply => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a * b); }\n".to_string(),
            crate::vm::opcode::OpCode::Divide => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a / b); }\n".to_string(),
            crate::vm::opcode::OpCode::Modulus => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a % b); }\n".to_string(),
            crate::vm::opcode::OpCode::Negate => "    { let a = stack.pop(); stack.push(-a); }\n".to_string(),
            crate::vm::opcode::OpCode::Equal => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a === b); }\n".to_string(),
            crate::vm::opcode::OpCode::NotEqual => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a !== b); }\n".to_string(),
            crate::vm::opcode::OpCode::Less => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a < b); }\n".to_string(),
            crate::vm::opcode::OpCode::Greater => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a > b); }\n".to_string(),
            crate::vm::opcode::OpCode::LessEqual => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a <= b); }\n".to_string(),
            crate::vm::opcode::OpCode::GreaterEqual => "    { let b = stack.pop(); let a = stack.pop(); stack.push(a >= b); }\n".to_string(),
            crate::vm::opcode::OpCode::Not => "    { let a = stack.pop(); stack.push(!a); }\n".to_string(),
            crate::vm::opcode::OpCode::Concat => "    { let b = stack.pop(); let a = stack.pop(); stack.push(String(a) + String(b)); }\n".to_string(),
            crate::vm::opcode::OpCode::Pop => "    stack.pop();\n".to_string(),
            crate::vm::opcode::OpCode::Print => "    { let a = stack.pop(); console.log(a); }\n".to_string(),
            crate::vm::opcode::OpCode::Assert => "    { let a = stack.pop(); if (!a) { throw new Error('Assertion failed'); } }\n".to_string(),
            crate::vm::opcode::OpCode::GetLocal => format!("    stack.push(locals[{}]);\n", a),
            crate::vm::opcode::OpCode::SetLocal => format!("    locals[{}] = stack.pop();\n", a),
            crate::vm::opcode::OpCode::GetGlobal => {
                if (a as usize) < chunk.constants.len() {
                    if let crate::vm::value::Value::Str(name) = &chunk.constants[a as usize] {
                        format!("    stack.push(globals['{}'] ?? null);\n", name)
                    } else {
                        "    stack.push(null);\n".to_string()
                    }
                } else {
                    "    stack.push(null);\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::SetGlobal => {
                if (a as usize) < chunk.constants.len() {
                    if let crate::vm::value::Value::Str(name) = &chunk.constants[a as usize] {
                        format!("    globals['{}'] = stack.pop();\n", name)
                    } else {
                        "    stack.pop();\n".to_string()
                    }
                } else {
                    "    stack.pop();\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::DefineGlobal => {
                if (a as usize) < chunk.constants.len() {
                    if let crate::vm::value::Value::Str(name) = &chunk.constants[a as usize] {
                        format!("    globals['{}'] = stack.pop();\n", name)
                    } else {
                        "    stack.pop();\n".to_string()
                    }
                } else {
                    "    stack.pop();\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::BuildList => format!("    {{ let count = {}; let arr = []; for (let i = 0; i < count; i++) arr.unshift(stack.pop()); stack.push(arr); }}\n", a),
            crate::vm::opcode::OpCode::Return => "    return;\n".to_string(),
            _ => format!("    // {:?} not implemented\n", op),
        };
        
        js.push_str(&code);
        ip += width;
        if op == crate::vm::opcode::OpCode::Closure {
            ip += b as usize;
        }
    }
    
    js.push_str("    } catch (e) {\n");
    js.push_str("        console.error('Runtime error:', e);\n");
    js.push_str("        process.exit(1);\n");
    js.push_str("    }\n");
    js.push_str("}\n");
    
    js
}

fn constant_to_js(val: &crate::vm::value::Value) -> String {
    match val {
        crate::vm::value::Value::Nil => "null".to_string(),
        crate::vm::value::Value::Void => "undefined".to_string(),
        crate::vm::value::Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        crate::vm::value::Value::Int(i) => i.to_string(),
        crate::vm::value::Value::Float(f) => f.to_string(),
        crate::vm::value::Value::Char(c) => format!("\"{}\"", c),
        crate::vm::value::Value::Str(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        _ => "null".to_string(),
    }
}