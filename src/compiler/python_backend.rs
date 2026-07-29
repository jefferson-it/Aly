use crate::compiler::CompileOptions;

pub fn compile_to_python(source: &str, _options: &CompileOptions) -> Result<String, String> {
    let program = crate::compiler::parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    
    let mut py = String::new();
    py.push_str("# Generated from Aly source\n");
    py.push_str("import sys\n");
    py.push_str("sys.path.insert(0, '.')\n");
    py.push_str("import runtime_aly\n\n");
    
    // Generate Python from chunk bytecode
    py.push_str(&generate_python_from_chunk(&chunk));
    
    py.push_str("\n\n# Entry point\n");
    py.push_str("if __name__ == '__main__':\n");
    py.push_str("    try:\n");
    py.push_str("        aly_main()\n");
    py.push_str("    except Exception as e:\n");
    py.push_str("        print(f'Runtime error: {e}')\n");
    py.push_str("        sys.exit(1)\n");
    
    Ok(py)
}

fn generate_python_from_chunk(chunk: &crate::vm::chunk::Chunk) -> String {
    let mut py = String::new();
    
    py.push_str("stack = []\n");
    py.push_str("locals = {}\n");
    py.push_str("globals = {}\n\n");
    
    py.push_str("def aly_main():\n");
    
    let mut ip = 0;
    while ip < chunk.code.len() {
        let insn = chunk.code[ip];
        let op = crate::vm::opcode::decode_op(insn);
        let a = crate::vm::opcode::decode_a(insn);
        let b = crate::vm::opcode::decode_b(insn);
        let width = crate::vm::opcode::opcode_width(op);
        
        let code = match op {
            crate::vm::opcode::OpCode::Nil => "    stack.append(None)\n".to_string(),
            crate::vm::opcode::OpCode::Void => "    stack.append(None)\n".to_string(),
            crate::vm::opcode::OpCode::True => "    stack.append(True)\n".to_string(),
            crate::vm::opcode::OpCode::False => "    stack.append(False)\n".to_string(),
            crate::vm::opcode::OpCode::LoadInt => format!("    stack.append({})\n", a as i64),
            crate::vm::opcode::OpCode::Const => {
                if (a as usize) < chunk.constants.len() {
                    format!("    stack.append({})\n", constant_to_python(&chunk.constants[a as usize]))
                } else {
                    "    stack.append(None)\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::Add => "    b = stack.pop(); a = stack.pop(); stack.append(a + b)\n".to_string(),
            crate::vm::opcode::OpCode::Subtract => "    b = stack.pop(); a = stack.pop(); stack.append(a - b)\n".to_string(),
            crate::vm::opcode::OpCode::Multiply => "    b = stack.pop(); a = stack.pop(); stack.append(a * b)\n".to_string(),
            crate::vm::opcode::OpCode::Divide => "    b = stack.pop(); a = stack.pop(); stack.append(a / b)\n".to_string(),
            crate::vm::opcode::OpCode::Modulus => "    b = stack.pop(); a = stack.pop(); stack.append(a % b)\n".to_string(),
            crate::vm::opcode::OpCode::Negate => "    a = stack.pop(); stack.append(-a)\n".to_string(),
            crate::vm::opcode::OpCode::Equal => "    b = stack.pop(); a = stack.pop(); stack.append(a == b)\n".to_string(),
            crate::vm::opcode::OpCode::NotEqual => "    b = stack.pop(); a = stack.pop(); stack.append(a != b)\n".to_string(),
            crate::vm::opcode::OpCode::Less => "    b = stack.pop(); a = stack.pop(); stack.append(a < b)\n".to_string(),
            crate::vm::opcode::OpCode::Greater => "    b = stack.pop(); a = stack.pop(); stack.append(a > b)\n".to_string(),
            crate::vm::opcode::OpCode::LessEqual => "    b = stack.pop(); a = stack.pop(); stack.append(a <= b)\n".to_string(),
            crate::vm::opcode::OpCode::GreaterEqual => "    b = stack.pop(); a = stack.pop(); stack.append(a >= b)\n".to_string(),
            crate::vm::opcode::OpCode::Not => "    a = stack.pop(); stack.append(not a)\n".to_string(),
            crate::vm::opcode::OpCode::Concat => "    b = stack.pop(); a = stack.pop(); stack.append(str(a) + str(b))\n".to_string(),
            crate::vm::opcode::OpCode::Pop => "    stack.pop()\n".to_string(),
            crate::vm::opcode::OpCode::Print => "    a = stack.pop(); print(a)\n".to_string(),
            crate::vm::opcode::OpCode::Assert => "    a = stack.pop(); assert a, 'Assertion failed'\n".to_string(),
            crate::vm::opcode::OpCode::GetLocal => format!("    stack.append(locals.get({}))\n", a),
            crate::vm::opcode::OpCode::SetLocal => format!("    locals[{}] = stack.pop()\n", a),
            crate::vm::opcode::OpCode::GetGlobal => {
                if (a as usize) < chunk.constants.len() {
                    if let crate::vm::value::Value::Str(name) = &chunk.constants[a as usize] {
                        format!("    stack.append(globals.get('{}', None))\n", name)
                    } else {
                        "    stack.append(None)\n".to_string()
                    }
                } else {
                    "    stack.append(None)\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::SetGlobal => {
                if (a as usize) < chunk.constants.len() {
                    if let crate::vm::value::Value::Str(name) = &chunk.constants[a as usize] {
                        format!("    globals['{}'] = stack.pop()\n", name)
                    } else {
                        "    stack.pop()\n".to_string()
                    }
                } else {
                    "    stack.pop()\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::DefineGlobal => {
                if (a as usize) < chunk.constants.len() {
                    if let crate::vm::value::Value::Str(name) = &chunk.constants[a as usize] {
                        format!("    globals['{}'] = stack.pop()\n", name)
                    } else {
                        "    stack.pop()\n".to_string()
                    }
                } else {
                    "    stack.pop()\n".to_string()
                }
            }
            crate::vm::opcode::OpCode::BuildList => format!("    count = {}; arr = []; for _ in range(count): arr.insert(0, stack.pop()); stack.append(arr)\n", a),
            crate::vm::opcode::OpCode::Return => "    return\n".to_string(),
            _ => format!("    # {:?} not implemented\n", op),
        };
        
        py.push_str(&code);
        ip += width;
        if op == crate::vm::opcode::OpCode::Closure {
            ip += b as usize;
        }
    }
    
    py
}

fn constant_to_python(val: &crate::vm::value::Value) -> String {
    match val {
        crate::vm::value::Value::Nil => "None".to_string(),
        crate::vm::value::Value::Void => "None".to_string(),
        crate::vm::value::Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        crate::vm::value::Value::Int(i) => i.to_string(),
        crate::vm::value::Value::Float(f) => f.to_string(),
        crate::vm::value::Value::Char(c) => format!("\"{}\"", c),
        crate::vm::value::Value::Str(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        _ => "None".to_string(),
    }
}