use crate::vm::chunk::Chunk;
use crate::vm::opcode::*;
use crate::vm::value::Value;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::types::{BasicType, BasicTypeEnum, IntType, StructType};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue, StructValue};
use inkwell::IntPredicate;
use inkwell::OptimizationLevel;

use std::collections::HashMap;

pub struct LlvmBackend<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    target_machine: Option<TargetMachine>,
    opt_level: OptLevel,
    target_triple: String,
    emit_type: EmitType,
    /// Maps function names to LLVM function values.
    functions: HashMap<String, FunctionValue<'ctx>>,
    /// The Aly value struct type.
    value_struct: StructType<'ctx>,
    /// Tag constants
    tag_nil: IntValue<'ctx>,
    tag_void: IntValue<'ctx>,
    tag_bool: IntValue<'ctx>,
    tag_int: IntValue<'ctx>,
    tag_float: IntValue<'ctx>,
    tag_char: IntValue<'ctx>,
    tag_string: IntValue<'ctx>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptLevel {
    Debug,      // O0
    Fast,       // O1
    Moderate,   // O2
    Aggressive, // O3
    Size,       // Os
    SizeZ,      // Oz
}

impl OptLevel {
    pub fn from_str(s: &str) -> Self {
        match s {
            "debug" | "o0" | "O0" => OptLevel::Debug,
            "fast" | "o1" | "O1" => OptLevel::Fast,
            "moderate" | "o2" | "O2" => OptLevel::Moderate,
            "aggressive" | "o3" | "O3" => OptLevel::Aggressive,
            "size" | "os" | "Os" => OptLevel::Size,
            "sizez" | "oz" | "Oz" => OptLevel::SizeZ,
            _ => OptLevel::Moderate,
        }
    }

    fn to_llvm(&self) -> OptimizationLevel {
        match self {
            OptLevel::Debug => OptimizationLevel::None,
            OptLevel::Fast => OptimizationLevel::Less,
            OptLevel::Moderate => OptimizationLevel::Default,
            OptLevel::Aggressive => OptimizationLevel::Aggressive,
            OptLevel::Size => OptimizationLevel::Default,
            OptLevel::SizeZ => OptimizationLevel::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmitType {
    Object,
    Assembly,
    LlvmIr,
}

impl EmitType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "obj" | "o" => EmitType::Object,
            "asm" | "s" => EmitType::Assembly,
            "llvm-ir" | "ir" | "ll" => EmitType::LlvmIr,
            _ => EmitType::Object,
        }
    }
}

impl<'ctx> LlvmBackend<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        let i8_type = context.i8_type();
        let i64_type = context.i64_type();

        let value_struct = context
            .opaque_struct_type("aly_value_t")
            .as_direct_type();
        let tag_type = context.i8_type();
        let data_types: &[BasicTypeEnum] = &[
            tag_type.as_basic_type_enum(),
            // data: union of int64, double, bool, char, ptr
            i64_type.as_basic_type_enum(),
        ];
        value_struct.set_body(data_types, false);

        let llvoid = context.i8_type();
        let tag_nil = llvoid.const_int(0, false);
        let tag_void = llvoid.const_int(1, false);
        let tag_bool = llvoid.const_int(2, false);
        let tag_int = llvoid.const_int(3, false);
        let tag_float = llvoid.const_int(4, false);
        let tag_char = llvoid.const_int(5, false);
        let tag_string = llvoid.const_int(6, false);

        LlvmBackend {
            context,
            module,
            builder,
            target_machine: None,
            opt_level: OptLevel::Moderate,
            target_triple: TargetMachine::get_default_triple().to_string(),
            emit_type: EmitType::Object,
            functions: HashMap::new(),
            value_struct,
            tag_nil,
            tag_void,
            tag_bool,
            tag_int,
            tag_float,
            tag_char,
            tag_string,
        }
    }

    pub fn set_opt_level(&mut self, level: OptLevel) {
        self.opt_level = level;
    }

    pub fn set_target_triple(&mut self, triple: &str) {
        self.target_triple = triple.to_string();
    }

    pub fn set_emit_type(&mut self, emit: EmitType) {
        self.emit_type = emit;
    }

    fn init_target(&mut self) -> Result<(), String> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialize native target: {}", e))?;
        Ok(())
    }

    fn create_target_machine(&self) -> Result<TargetMachine, String> {
        let triple = TargetTriple::create(&self.target_triple);
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to get target from triple: {}", e))?;

        let cpu = TargetMachine::get_host_cpu_name();
        let features = TargetMachine::get_host_cpu_features();

        let machine = target
            .create_target_machine(
                &triple,
                &cpu,
                &features,
                self.opt_level.to_llvm(),
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| "Failed to create target machine".to_string())?;

        Ok(machine)
    }

    /// Compile a Chunk (VM bytecode) to LLVM IR module.
    pub fn compile_chunk(&mut self, chunk: &Chunk, fn_name: &str) -> Result<(), String> {
        let i64_type = self.context.i64_type();
        let i8_type = self.context.i8_type();
        let void_type = self.context.void_type();

        let func_type = void_type.fn_type(&[
            i64_type.as_basic_type_enum(), // stack_ptr
            i64_type.as_basic_type_enum(), // locals_ptr
            i64_type.as_basic_type_enum(), // globals_ptr
        ], false);

        let function = self.module.add_function(fn_name, func_type, None);
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        let stack_ptr_param = function.get_nth_param(0).unwrap().into_int_value();
        let locals_ptr_param = function.get_nth_param(1).unwrap().into_int_value();
        let globals_ptr_param = function.get_nth_param(2).unwrap().into_int_value();

        let mut stack_top = self.builder.build_alloca(i64_type, "stack_top");
        self.builder.build_store(stack_top, i64_type.const_int(0, false));

        let mut ip = 0;
        while ip < chunk.code.len() {
            let insn = chunk.code[ip];
            let op = decode_op(insn);
            let a = decode_a(insn);
            let b = decode_b(insn);
            let width = opcode_width(op);

            match op {
                OpCode::Return => {
                    self.builder.build_return(None);
                }
                OpCode::Pop => {
                    let top = self.builder.build_load(i64_type, stack_top, "top");
                    let one = i64_type.const_int(1, false);
                    let new_top = self.builder.build_int_sub(top.into_int_value(), one, "new_top");
                    self.builder.build_store(stack_top, new_top);
                }
                OpCode::Nil => {
                    let val = self.make_value(self.tag_nil, i64_type.const_zero());
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::True => {
                    let val = self.make_value(self.tag_bool, i64_type.const_int(1, false));
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::False => {
                    let val = self.make_value(self.tag_bool, i64_type.const_int(0, false));
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::LoadInt => {
                    let val = self.make_value(self.tag_int, i64_type.const_int(a as u64, false));
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Const => {
                    let constant = &chunk.constants[a as usize];
                    let val = self.value_to_llvm(constant, i64_type);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Add => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let result = self.builder.build_int_add(left, right, "add");
                    let val = self.make_value(self.tag_int, result);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Subtract => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let result = self.builder.build_int_sub(left, right, "sub");
                    let val = self.make_value(self.tag_int, result);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Multiply => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let result = self.builder.build_int_mul(left, right, "mul");
                    let val = self.make_value(self.tag_int, result);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Divide => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let result = self.builder.build_int_signed_div(left, right, "div");
                    let val = self.make_value(self.tag_int, result);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Equal => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::EQ, left, right, "eq");
                    let ext = self.builder.build_int_z_extend(cmp, i64_type, "eq_ext");
                    let val = self.make_value(self.tag_bool, ext);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::NotEqual => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::NE, left, right, "ne");
                    let ext = self.builder.build_int_z_extend(cmp, i64_type, "ne_ext");
                    let val = self.make_value(self.tag_bool, ext);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Less => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::SLT, left, right, "lt");
                    let ext = self.builder.build_int_z_extend(cmp, i64_type, "lt_ext");
                    let val = self.make_value(self.tag_bool, ext);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Greater => {
                    let right = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let left = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::SGT, left, right, "gt");
                    let ext = self.builder.build_int_z_extend(cmp, i64_type, "gt_ext");
                    let val = self.make_value(self.tag_bool, ext);
                    self.push_value(val, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Print => {
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let printf_fn = self.module.get_function("printf");
                    if printf_fn.is_none() {
                        let printf_type = i64_type.fn_type(&[
                            i8_type.ptr_type(inkwell::AddressSpace::Generic).as_basic_type_enum(),
                        ], true);
                        self.module.add_function("printf", printf_type, None);
                    }
                }
                OpCode::DefineGlobal => {
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let name = if let Value::Str(s) = &chunk.constants[a as usize] {
                        s.clone()
                    } else {
                        format!("global_{}", a)
                    };
                    let gv = self.module.add_global(self.value_struct, None, &name);
                    gv.set_initializer(&self.value_struct.const_zero());
                }
                OpCode::GetGlobal => {
                    let name = if let Value::Str(s) = &chunk.constants[a as usize] {
                        s.clone()
                    } else {
                        format!("global_{}", a)
                    };
                    if let Some(gv) = self.module.get_global(&name) {
                        let ptr = gv.as_pointer_value();
                        let loaded = self.builder.build_load(self.value_struct, ptr, &name);
                        self.push_value(loaded.into_struct_value(), stack_ptr_param, stack_top, i64_type);
                    } else {
                        let val = self.make_value(self.tag_nil, i64_type.const_zero());
                        self.push_value(val, stack_ptr_param, stack_top, i64_type);
                    }
                }
                OpCode::SetGlobal => {
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let name = if let Value::Str(s) = &chunk.constants[a as usize] {
                        s.clone()
                    } else {
                        format!("global_{}", a)
                    };
                    if let Some(gv) = self.module.get_global(&name) {
                        let ptr = gv.as_pointer_value();
                        self.builder.build_store(ptr, val);
                    }
                }
                OpCode::Not => {
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let zero = i64_type.const_int(0, false);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::EQ, val, zero, "not_cmp");
                    let ext = self.builder.build_int_z_extend(cmp, i64_type, "not_ext");
                    let result = self.make_value(self.tag_bool, ext);
                    self.push_value(result, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Negate => {
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let neg = self.builder.build_int_neg(val, "neg");
                    let result = self.make_value(self.tag_int, neg);
                    self.push_value(result, stack_ptr_param, stack_top, i64_type);
                }
                OpCode::Jump => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    let target_label = format!("block_{}", target);
                    let bb = self.context.append_basic_block(function, &target_label);
                    self.builder.build_unconditional_branch(bb);
                    self.builder.position_at_end(bb);
                }
                OpCode::JumpIfFalse => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let zero = i64_type.const_int(0, false);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::EQ, val, zero, "jmpf_cond");
                    let else_bb = self.context.append_basic_block(function, "jmpf_else");
                    let merge_bb = self.context.append_basic_block(function, "jmpf_merge");
                    self.builder.build_conditional_branch(cmp, else_bb, merge_bb);
                    self.builder.position_at_end(else_bb);
                    let target_label = format!("block_{}", target);
                    let target_bb = self.context.append_basic_block(function, &target_label);
                    self.builder.build_unconditional_branch(target_bb);
                    self.builder.position_at_end(merge_bb);
                }
                OpCode::JumpIfTrue => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    let val = self.pop_value(stack_ptr_param, stack_top, i64_type);
                    let zero = i64_type.const_int(0, false);
                    let cmp = self.builder.build_int_compare(
                        IntPredicate::NE, val, zero, "jmpf_cond");
                    let then_bb = self.context.append_basic_block(function, "jmpt_then");
                    let merge_bb = self.context.append_basic_block(function, "jmpt_merge");
                    self.builder.build_conditional_branch(cmp, then_bb, merge_bb);
                    self.builder.position_at_end(then_bb);
                    let target_label = format!("block_{}", target);
                    let target_bb = self.context.append_basic_block(function, &target_label);
                    self.builder.build_unconditional_branch(target_bb);
                    self.builder.position_at_end(merge_bb);
                }
                OpCode::Loop => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    let target_label = format!("block_{}", target);
                    if let Some(bb) = function.get_basic_block(&target_label) {
                        self.builder.build_unconditional_branch(bb);
                    }
                }
                OpCode::Call => {
                }
                OpCode::Closure => {
                }
                _ => {}
            }

            ip += width;
        }

        self.functions.insert(fn_name.to_string(), function);

        Ok(())
    }

    fn value_to_llvm(&self, val: &Value, i64_type: IntType<'ctx>) -> StructValue<'ctx> {
        match val {
            Value::Nil => self.make_value(self.tag_nil, i64_type.const_zero()),
            Value::Void => self.make_value(self.tag_void, i64_type.const_zero()),
            Value::Bool(b) => self.make_value(
                self.tag_bool,
                i64_type.const_int(if *b { 1 } else { 0 }, false),
            ),
            Value::Int(i) => self.make_value(
                self.tag_int,
                i64_type.const_int(*i as u64, false),
            ),
            Value::Float(f) => self.make_value(
                self.tag_float,
                i64_type.const_int(f.to_bits(), false),
            ),
            Value::Char(c) => self.make_value(
                self.tag_char,
                i64_type.const_int(*c as u64, false),
            ),
            Value::Str(s) => self.make_value(
                self.tag_string,
                i64_type.const_zero(),
            ),
            _ => self.make_value(self.tag_nil, i64_type.const_zero()),
        }
    }

    fn make_value(&self, tag: IntValue<'ctx>, data: IntValue<'ctx>) -> StructValue<'ctx> {
        self.value_struct.const_packed_struct(&[
            tag.as_basic_value_enum(),
            data.as_basic_value_enum(),
        ])
    }

    fn push_value(
        &self,
        val: StructValue<'ctx>,
        stack_ptr: PointerValue<'ctx>,
        stack_top: PointerValue<'ctx>,
        i64_type: IntType<'ctx>,
    ) {
        let top = self.builder.build_load(i64_type, stack_top, "push_top");
        let top_val = top.into_int_value();
        let gep = self.builder.build_in_bounds_gep(
            self.value_struct,
            stack_ptr,
            &[top_val],
            "push_gep",
        );
        self.builder.build_store(gep, val);
        let one = i64_type.const_int(1, false);
        let new_top = self.builder.build_int_add(top_val, one, "push_new_top");
        self.builder.build_store(stack_top, new_top);
    }

    fn pop_value(
        &self,
        stack_ptr: PointerValue<'ctx>,
        stack_top: PointerValue<'ctx>,
        i64_type: IntType<'ctx>,
    ) -> IntValue<'ctx> {
        let top = self.builder.build_load(i64_type, stack_top, "pop_top");
        let top_val = top.into_int_value();
        let one = i64_type.const_int(1, false);
        let new_top = self.builder.build_int_sub(top_val, one, "pop_new_top");
        self.builder.build_store(stack_top, new_top);
        let gep = self.builder.build_in_bounds_gep(
            self.value_struct,
            stack_ptr,
            &[new_top],
            "pop_gep",
        );
        let loaded = self.builder.build_load(self.value_struct, gep, "pop_val");
        let struct_val = loaded.into_struct_value();
        let data = self.builder.build_extract_value(struct_val, 1, "pop_data")
            .unwrap()
            .into_int_value();
        data
    }

    /// Emit the compiled module to an object file.
    pub fn emit_to_file(&mut self, output_path: &str) -> Result<(), String> {
        self.init_target()?;
        let machine = self.create_target_machine()?;

        let file_type = match self.emit_type {
            EmitType::Object => FileType::Object,
            EmitType::Assembly => FileType::Assembly,
            EmitType::LlvmIr => {
                let ir = self.module.to_string();
                std::fs::write(output_path, ir)
                    .map_err(|e| format!("Failed to write LLVM IR: {}", e))?;
                return Ok(());
            }
        };

        machine
            .write_to_file(&self.module, file_type, std::path::Path::new(output_path))
            .map_err(|(_, e)| format!("Failed to emit file: {}", e))
    }

    /// Optimize the module using LLVM's pass manager.
    pub fn optimize(&mut self) -> Result<(), String> {
        self.module
            .run_verification_and_assert_ir();

        let pass_manager = self.module.create_pass_manager();
        match self.opt_level {
            OptLevel::Debug => {}
            OptLevel::Fast => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
                pass_manager.add_gvn_pass();
                pass_manager.add_cfg_simplification_pass();
                pass_manager.add_basic_alias_analysis_pass();
                pass_manager.add_promote_memory_to_register_pass();
                pass_manager.add_early_cse_pass();
            }
            OptLevel::Moderate => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
                pass_manager.add_gvn_pass();
                pass_manager.add_cfg_simplification_pass();
                pass_manager.add_basic_alias_analysis_pass();
                pass_manager.add_promote_memory_to_register_pass();
                pass_manager.add_early_cse_pass();
                pass_manager.add_licm_pass();
                pass_manager.add_ind_var_simplify_pass();
                pass_manager.add_loop_deletion_pass();
                pass_manager.add_sroa_pass();
            }
            OptLevel::Aggressive => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
                pass_manager.add_gvn_pass();
                pass_manager.add_cfg_simplification_pass();
                pass_manager.add_basic_alias_analysis_pass();
                pass_manager.add_promote_memory_to_register_pass();
                pass_manager.add_early_cse_pass();
                pass_manager.add_licm_pass();
                pass_manager.add_ind_var_simplify_pass();
                pass_manager.add_loop_deletion_pass();
                pass_manager.add_sroa_pass();
                pass_manager.add_function_inlining_pass();
                pass_manager.add_argpromotion_pass();
                pass_manager.add_tail_call_elimination_pass();
                pass_manager.add_simplify_lib_calls_pass();
            }
            OptLevel::Size | OptLevel::SizeZ => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_cfg_simplification_pass();
                pass_manager.add_promote_memory_to_register_pass();
                pass_manager.add_sroa_pass();
                pass_manager.add_merge_functions_pass();
                pass_manager.add_function_inlining_pass();
                pass_manager.add_global_optimizer_pass();
            }
        }

        pass_manager.add_verifier_pass();
        pass_manager.run_on(&self.module);
        Ok(())
    }
}

/// High-level function: compile an Aly Chunk to native code via LLVM.
pub fn compile_chunk_to_native(
    chunk: &Chunk,
    output_name: &str,
    opt_level: &str,
    target: &str,
    emit: &str,
) -> Result<(), String> {
    let context = Context::create();
    let mut backend = LlvmBackend::new(&context, "aly_module");

    backend.set_opt_level(OptLevel::from_str(opt_level));
    backend.set_target_triple(target);
    backend.set_emit_type(EmitType::from_str(emit));

    backend.compile_chunk(chunk, "aly_main")?;
    backend.optimize()?;

    let ext = match backend.emit_type {
        EmitType::Object => "o",
        EmitType::Assembly => "s",
        EmitType::LlvmIr => "ll",
    };
    let out_path = format!("{}.{}", output_name, ext);
    backend.emit_to_file(&out_path)?;

    if emit == "obj" || emit == "o" {
        let status = std::process::Command::new("gcc")
            .args(["-o", output_name, &out_path, "-lm"])
            .spawn()
            .map_err(|e| format!("Failed to spawn linker: {}", e))?
            .wait()
            .map_err(|e| format!("Failed to wait for linker: {}", e))?;

        if !status.success() {
            return Err("Linking failed.".to_string());
        }
    }

    Ok(())
}
