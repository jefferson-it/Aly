use crate::vm::chunk::Chunk;
use crate::vm::opcode::*;
use crate::vm::value::Value;
use std::collections::HashSet;

pub struct CodeGenerator {
    code: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            code: String::new(),
        }
    }

    pub fn generate(&mut self, chunk: &Chunk) -> String {
        // Emit C standard headers
        self.code.push_str("#include \"runtime_aly.h\"\n\n");
        self.code.push_str("aly_value_t aly_globals;\n\n");

        // Native functions declarations and definitions
        self.code.push_str("aly_value_t native_print(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    for (int i = 0; i < arg_count; i++) {\n");
        self.code.push_str("        if (i > 0) printf(\" \");\n");
        self.code.push_str("        aly_print_raw(args[i]);\n");
        self.code.push_str("    }\n");
        self.code.push_str("    printf(\"\\n\");\n");
        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_input(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count > 0) { return aly_input(args[0].as.str_val); }\n");
        self.code.push_str("    return aly_input(\"\");\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_tomb(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count > 0) { return aly_tomb(args[0]); }\n");
        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_len(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count > 0) { return aly_int(aly_len(args[0])); }\n");
        self.code.push_str("    return aly_int(0);\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_simd_add(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count >= 2) { return aly_simd_op(args[0], args[1], \"+\"); }\n");
        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_simd_sub(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count >= 2) { return aly_simd_op(args[0], args[1], \"-\"); }\n");
        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_simd_mul(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count >= 2) { return aly_simd_op(args[0], args[1], \"*\"); }\n");
        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");

        self.code.push_str("aly_value_t native_simd_div(int arg_count, aly_value_t* args) {\n");
        self.code.push_str("    if (arg_count >= 2) { return aly_simd_op(args[0], args[1], \"/\"); }\n");
        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");

        // Forward declare all functions in the Chunk hierarchy
        let mut forward_declarations = String::new();
        self.declare_chunk_functions(chunk, &mut forward_declarations, 0);
        self.code.push_str(&forward_declarations);
        self.code.push_str("\n");

        // Define all functions in the Chunk hierarchy
        self.define_chunk_functions(chunk, 0);

        // Define main
        self.code.push_str("int main(int argc, char** argv) {\n");
        self.code.push_str("    aly_init(argc, argv);\n");
        self.code.push_str("    aly_globals.type = ALY_OBJECT;\n");
        self.code.push_str("    aly_globals.as.object_val = aly_object_new();\n");
        self.code.push_str("    aly_globals.type_tag = NULL;\n\n");

        // Register built-in native functions
        self.code.push_str("    aly_object_set(aly_globals, \"print\", aly_function((void*)native_print));\n");
        self.code.push_str("    aly_object_set(aly_globals, \"input\", aly_function((void*)native_input));\n");
        self.code.push_str("    aly_object_set(aly_globals, \"tomb\", aly_function((void*)native_tomb));\n");
        self.code.push_str("    aly_object_set(aly_globals, \"len\", aly_function((void*)native_len));\n\n");

        self.code.push_str("    aly_value_t simd_obj;\n");
        self.code.push_str("    simd_obj.type = ALY_OBJECT;\n");
        self.code.push_str("    simd_obj.as.object_val = aly_object_new();\n");
        self.code.push_str("    simd_obj.type_tag = NULL;\n");
        self.code.push_str("    simd_obj.is_mutable = 1;\n");
        self.code.push_str("    aly_object_set(simd_obj, \"add\", aly_function((void*)native_simd_add));\n");
        self.code.push_str("    aly_object_set(simd_obj, \"sub\", aly_function((void*)native_simd_sub));\n");
        self.code.push_str("    aly_object_set(simd_obj, \"mul\", aly_function((void*)native_simd_mul));\n");
        self.code.push_str("    aly_object_set(simd_obj, \"div\", aly_function((void*)native_simd_div));\n");
        self.code.push_str("    aly_object_set(aly_globals, \"simd\", simd_obj);\n\n");

        // Call top-level chunk (fn_0)
        self.code.push_str("    aly_value_t args[1] = { aly_none() };\n");
        self.code.push_str("    fn_0(0, args);\n");
        self.code.push_str("    aly_cleanup();\n");
        self.code.push_str("    return 0;\n");
        self.code.push_str("}\n");

        self.code.clone()
    }

    fn declare_chunk_functions(&mut self, chunk: &Chunk, decls: &mut String, id: usize) {
        decls.push_str(&format!("aly_value_t fn_{}(int arg_count, aly_value_t* args);\n", id));
        let mut child_id = id + 1;
        for func in &chunk.functions {
            self.declare_chunk_functions(func, decls, child_id);
            child_id += 1;
        }
    }

    fn define_chunk_functions(&mut self, chunk: &Chunk, id: usize) {
        // Recursively define sub-functions first
        let mut child_id = id + 1;
        for func in &chunk.functions {
            self.define_chunk_functions(func, child_id);
            child_id += 1;
        }

        // Now define this function
        self.code.push_str(&format!("aly_value_t fn_{}(int arg_count, aly_value_t* args) {{\n", id));
        self.code.push_str("    aly_value_t stack[1024];\n");
        self.code.push_str("    int stack_top = 0;\n");
        
        let local_count = chunk.local_count;
        self.code.push_str(&format!("    aly_value_t locals[{}];\n", if local_count > 0 { local_count } else { 1 }));
        self.code.push_str(&format!("    for (int i = 0; i < {}; i++) locals[i] = aly_none();\n\n", if local_count > 0 { local_count } else { 1 }));

        // Map arguments to local slots
        let param_count = chunk.param_names.len();
        for i in 0..param_count {
            self.code.push_str(&format!("    if ({} < arg_count) {{\n", i));
            self.code.push_str(&format!("        locals[{}] = aly_clone(args[{}]);\n", i, i));
            self.code.push_str("    }");
            if let Some(Some(ref def_val)) = chunk.param_defaults.get(i) {
                self.code.push_str(" else {\n");
                self.code.push_str(&format!("        locals[{}] = {};\n", i, self.value_to_c(def_val)));
                self.code.push_str("    }\n");
            } else {
                self.code.push_str("\n");
            }
        }
        self.code.push_str("\n");

        // Scan bytecode to find jump targets
        let mut jump_targets = HashSet::new();
        let mut ip = 0;
        while ip < chunk.code.len() {
            let insn = chunk.code[ip];
            let op = decode_op(insn);
            match op {
                OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    jump_targets.insert(target);
                }
                OpCode::Loop => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 - offset as i32) as usize;
                    jump_targets.insert(target);
                }
                _ => {}
            }
            ip += opcode_width(op);
            if op == OpCode::Closure {
                ip += decode_b(insn) as usize;
            }
        }

        // Generate C code for each instruction
        ip = 0;
        while ip < chunk.code.len() {
            if jump_targets.contains(&ip) {
                self.code.push_str(&format!("label_{}_{}:\n", id, ip));
            }

            let insn = chunk.code[ip];
            let op = decode_op(insn);
            let a = decode_a(insn);
            let b = decode_b(insn);

            self.code.push_str(&format!("    /* {:04x}: {:?} */\n", ip, op));

            match op {
                OpCode::Return => {
                    self.code.push_str("    return stack_top > 0 ? stack[--stack_top] : aly_none();\n");
                }
                OpCode::Pop => {
                    self.code.push_str("    if (stack_top > 0) stack_top--;\n");
                }
                OpCode::Dup => {
                    self.code.push_str("    if (stack_top > 0) { stack[stack_top] = stack[stack_top - 1]; stack_top++; }\n");
                }
                OpCode::Const => {
                    let val = &chunk.constants[a as usize];
                    self.code.push_str(&format!("    stack[stack_top++] = {};\n", self.value_to_c(val)));
                }
                OpCode::Nil => {
                    self.code.push_str("    stack[stack_top++] = aly_none();\n");
                }
                OpCode::True => {
                    self.code.push_str("    stack[stack_top++] = aly_bool(1);\n");
                }
                OpCode::False => {
                    self.code.push_str("    stack[stack_top++] = aly_bool(0);\n");
                }
                OpCode::LoadInt => {
                    self.code.push_str(&format!("    stack[stack_top++] = aly_int({}LL);\n", a));
                }
                OpCode::LoadIntLong => {
                    let lo = chunk.code[ip + 1] as u64;
                    let hi = chunk.code[ip + 2] as u64;
                    let val = i64::from_ne_bytes((lo | (hi << 32)).to_ne_bytes());
                    self.code.push_str(&format!("    stack[stack_top++] = aly_int({}LL);\n", val));
                }
                OpCode::DefineGlobal => {
                    let val = &chunk.constants[a as usize];
                    if let Value::Str(ref name) = val {
                        self.code.push_str(&format!("    aly_object_set(aly_globals, \"{}\", stack[--stack_top]);\n", name));
                    }
                }
                OpCode::GetGlobal => {
                    let val = &chunk.constants[a as usize];
                    if let Value::Str(ref name) = val {
                        self.code.push_str(&format!("    stack[stack_top++] = aly_object_get(aly_globals, \"{}\");\n", name));
                    }
                }
                OpCode::SetGlobal => {
                    let val = &chunk.constants[a as usize];
                    if let Value::Str(ref name) = val {
                        self.code.push_str(&format!("    aly_object_set(aly_globals, \"{}\", stack[--stack_top]);\n", name));
                    }
                }
                OpCode::GetLocal => {
                    self.code.push_str(&format!("    stack[stack_top++] = locals[{}];\n", a));
                }
                OpCode::SetLocal => {
                    self.code.push_str(&format!(
                        "    if (!locals[{}].is_mutable) {{ fprintf(stderr, \"TypeError: A variável é constante/imutável, não é possível alterar seu valor.\\n\"); exit(1); }}\n",
                        a
                    ));
                    self.code.push_str(&format!("    locals[{}] = stack[--stack_top];\n", a));
                }
                OpCode::InitLocal => {
                    self.code.push_str(&format!("    locals[{}] = stack[stack_top - 1];\n", a));
                }
                OpCode::GetRefLocal => {
                    self.code.push_str(&format!("    stack[stack_top++] = aly_ref_val(&locals[{}]);\n", a));
                }
                OpCode::GetRefGlobal => {
                    let val = &chunk.constants[a as usize];
                    if let Value::Str(ref name) = val {
                        self.code.push_str(&format!("    stack[stack_top++] = aly_ref_val(aly_object_get_ptr(aly_globals, \"{}\"));\n", name));
                    }
                }
                OpCode::GetUpvalue => {
                    self.code.push_str("    stack[stack_top++] = aly_none();\n");
                }
                OpCode::SetUpvalue => {}
                OpCode::Jump => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    self.code.push_str(&format!("    goto label_{}_{};\n", id, target));
                }
                OpCode::JumpIfFalse => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    self.code.push_str(&format!("    if (!aly_is_truthy(stack[stack_top - 1])) goto label_{}_{};\n", id, target));
                }
                OpCode::JumpIfTrue => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    self.code.push_str(&format!("    if (aly_is_truthy(stack[stack_top - 1])) goto label_{}_{};\n", id, target));
                }
                OpCode::Loop => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 - offset as i32) as usize;
                    self.code.push_str(&format!("    goto label_{}_{};\n", id, target));
                }
                OpCode::Call => {
                    let num_args = a as usize;
                    self.code.push_str("    {\n");
                    self.code.push_str(&format!("        int num_args = {};\n", num_args));
                    self.code.push_str(&format!("        aly_value_t args[{}];\n", if num_args > 1 { num_args - 1 } else { 1 }));
                    self.code.push_str(&format!("        for (int i = num_args - 2; i >= 0; i--) args[i] = stack[--stack_top];\n"));
                    self.code.push_str("        aly_value_t callee = stack[--stack_top];\n");
                    self.code.push_str("        aly_value_t res = aly_none();\n");
                    self.code.push_str("        if (callee.type == ALY_FUNCTION && callee.as.func_val) {\n");
                    self.code.push_str("            res = ((aly_value_t(*)(int, aly_value_t*))callee.as.func_val)(num_args - 1, args);\n");
                    self.code.push_str("        }\n");
                    self.code.push_str("        stack[stack_top++] = res;\n");
                    self.code.push_str("    }\n");
                }
                OpCode::Closure => {
                    // Find child function index
                    let child_func_idx = a as usize;
                    let parent_child_id = id + 1 + child_func_idx;
                    self.code.push_str(&format!("    stack[stack_top++] = aly_function((void*)fn_{});\n", parent_child_id));
                }
                OpCode::Add => {
                    self.code.push_str("    stack[stack_top - 2] = aly_add(stack[stack_top - 2], stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Subtract => {
                    self.code.push_str("    stack[stack_top - 2] = aly_sub(stack[stack_top - 2], stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Multiply => {
                    self.code.push_str("    stack[stack_top - 2] = aly_mul(stack[stack_top - 2], stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Divide => {
                    self.code.push_str("    stack[stack_top - 2] = aly_div(stack[stack_top - 2], stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Modulus => {
                    self.code.push_str("    stack[stack_top - 2] = aly_mod(stack[stack_top - 2], stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Negate => {
                    self.code.push_str("    stack[stack_top - 1] = aly_neg(stack[stack_top - 1]);\n");
                }
                OpCode::Equal => {
                    self.code.push_str("    stack[stack_top - 2] = aly_compare(stack[stack_top - 2], \"==\", stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::NotEqual => {
                    self.code.push_str("    stack[stack_top - 2] = aly_compare(stack[stack_top - 2], \"!=\", stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Less => {
                    self.code.push_str("    stack[stack_top - 2] = aly_compare(stack[stack_top - 2], \"<\", stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Greater => {
                    self.code.push_str("    stack[stack_top - 2] = aly_compare(stack[stack_top - 2], \">\", stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::LessEqual => {
                    self.code.push_str("    stack[stack_top - 2] = aly_compare(stack[stack_top - 2], \"<=\", stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::GreaterEqual => {
                    self.code.push_str("    stack[stack_top - 2] = aly_compare(stack[stack_top - 2], \">=\", stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::Not => {
                    self.code.push_str("    stack[stack_top - 1] = aly_not(stack[stack_top - 1]);\n");
                }
                OpCode::Concat => {
                    self.code.push_str("    stack[stack_top - 2] = aly_add(stack[stack_top - 2], stack[stack_top - 1]); stack_top--;\n");
                }
                OpCode::BuildList => {
                    let count = a as usize;
                    self.code.push_str("    {\n");
                    self.code.push_str(&format!("        int count = {};\n", count));
                    self.code.push_str(&format!("        aly_value_t items[{}];\n", if count > 0 { count } else { 1 }));
                    self.code.push_str("        for (int i = count - 1; i >= 0; i--) items[i] = stack[--stack_top];\n");
                    self.code.push_str("        stack[stack_top++] = aly_array_init(count, items);\n");
                    self.code.push_str("    }\n");
                }
                OpCode::BuildTuple => {
                    let count = a as usize;
                    self.code.push_str("    {\n");
                    self.code.push_str(&format!("        int count = {};\n", count));
                    self.code.push_str(&format!("        aly_value_t items[{}];\n", if count > 0 { count } else { 1 }));
                    self.code.push_str("        for (int i = count - 1; i >= 0; i--) items[i] = stack[--stack_top];\n");
                    self.code.push_str("        stack[stack_top++] = aly_array_init(count, items);\n"); // fallback / or tuple init if available
                    self.code.push_str("    }\n");
                }
                OpCode::BuildMap => {
                    let count = a as usize;
                    self.code.push_str("    {\n");
                    self.code.push_str(&format!("        int count = {};\n", count));
                    self.code.push_str("        aly_value_t obj;\n");
                    self.code.push_str("        obj.type = ALY_OBJECT;\n");
                    self.code.push_str("        obj.as.object_val = aly_object_new();\n");
                    self.code.push_str("        obj.type_tag = NULL;\n");
                    self.code.push_str(&format!("        aly_value_t pairs[{}];\n", if count * 2 > 0 { count * 2 } else { 1 }));
                    self.code.push_str("        for (int i = (count * 2) - 1; i >= 0; i--) pairs[i] = stack[--stack_top];\n");
                    self.code.push_str("        for (int i = 0; i < count; i++) {\n");
                    self.code.push_str("            aly_value_t sk = aly_to_str(pairs[i * 2]);\n");
                    self.code.push_str("            aly_object_set(obj, sk.as.str_val, pairs[i * 2 + 1]);\n");
                    self.code.push_str("            aly_free(sk);\n");
                    self.code.push_str("        }\n");
                    self.code.push_str("        stack[stack_top++] = obj;\n");
                    self.code.push_str("    }\n");
                }
                OpCode::Subscript => {
                    self.code.push_str("    {\n");
                    self.code.push_str("        aly_value_t key = stack[--stack_top];\n");
                    self.code.push_str("        aly_value_t obj = stack[--stack_top];\n");
                    self.code.push_str("        aly_value_t res = aly_none();\n");
                    self.code.push_str("        if (obj.type == ALY_ARRAY) {\n");
                    self.code.push_str("            res = aly_array_get(obj, key);\n");
                    self.code.push_str("        } else if (obj.type == ALY_OBJECT) {\n");
                    self.code.push_str("            aly_value_t sk = aly_to_str(key);\n");
                    self.code.push_str("            res = aly_object_get(obj, sk.as.str_val);\n");
                    self.code.push_str("            aly_free(sk);\n");
                    self.code.push_str("        }\n");
                    self.code.push_str("        stack[stack_top++] = res;\n");
                    self.code.push_str("    }\n");
                }
                OpCode::SubscriptSet => {
                    self.code.push_str("    {\n");
                    self.code.push_str("        aly_value_t value = stack[--stack_top];\n");
                    self.code.push_str("        aly_value_t key = stack[--stack_top];\n");
                    self.code.push_str("        aly_value_t obj = stack[--stack_top];\n");
                    self.code.push_str("        if (obj.type == ALY_ARRAY) {\n");
                    self.code.push_str("            aly_array_set(obj, key, value);\n");
                    self.code.push_str("        } else if (obj.type == ALY_OBJECT) {\n");
                    self.code.push_str("            aly_value_t sk = aly_to_str(key);\n");
                    self.code.push_str("            aly_object_set(obj, sk.as.str_val, value);\n");
                    self.code.push_str("            aly_free(sk);\n");
                    self.code.push_str("        }\n");
                    self.code.push_str("    }\n");
                }
                OpCode::Print => {
                    self.code.push_str("    if (stack_top > 0) aly_print(stack[--stack_top]);\n");
                }
                OpCode::Assert => {
                    self.code.push_str("    if (stack_top > 0 && !aly_is_truthy(stack[--stack_top])) { fprintf(stderr, \"Assertion failed\\n\"); exit(1); }\n");
                }
                OpCode::ForceLazy => {
                    self.code.push_str("    // ForceLazy: evaluate lazy value (not fully implemented in C codegen)\n");
                    self.code.push_str("    if (stack_top > 0 && stack[stack_top-1].type == ALY_FUNCTION) {\n");
                    self.code.push_str("        aly_value_t lazy_fn = stack[stack_top-1];\n");
                    self.code.push_str("        aly_value_t res = ((aly_value_t(*)(int, aly_value_t*))lazy_fn.as.func_val)(0, NULL);\n");
                    self.code.push_str("        stack[stack_top-1] = res;\n");
                    self.code.push_str("    }\n");
                }
                OpCode::Yield => {
                    self.code.push_str("    // Yield: save coroutine state and return\n");
                    self.code.push_str("    return stack_top > 0 ? stack[--stack_top] : aly_none();\n");
                }
                OpCode::MakeCoroutine => {
                    self.code.push_str("    // MakeCoroutine: wrap function as coroutine (passthrough in C codegen)\n");
                }
                OpCode::Resume => {
                    self.code.push_str("    // Resume: resume coroutine (passthrough in C codegen)\n");
                    self.code.push_str("    if (stack_top > 0 && stack[stack_top-1].type == ALY_FUNCTION) {\n");
                    self.code.push_str("        aly_value_t coro = stack[stack_top-1];\n");
                    self.code.push_str("        aly_value_t res = ((aly_value_t(*)(int, aly_value_t*))coro.as.func_val)(0, NULL);\n");
                    self.code.push_str("        stack[stack_top-1] = res;\n");
                    self.code.push_str("    }\n");
                }
                _ => {}
            }

            ip += opcode_width(op);
            if op == OpCode::Closure {
                ip += b as usize;
            }
        }

        self.code.push_str("    return aly_none();\n");
        self.code.push_str("}\n\n");
    }

    fn value_to_c(&self, val: &Value) -> String {
        match val {
            Value::Nil => "aly_none()".to_string(),
            Value::Bool(b) => format!("aly_bool({})", if *b { 1 } else { 0 }),
            Value::Int(i) => format!("aly_int({}LL)", i),
            Value::Float(f) => format!("aly_float({})", f),
            Value::Str(s) => format!("aly_string(\"{}\")", s.escape_default()),
            _ => "aly_none()".to_string(),
        }
    }
}
