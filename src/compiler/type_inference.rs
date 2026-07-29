// src/compiler/type_inference.rs
use crate::compiler::hir::{HirProgram, HirFunction, HirInstruction, HirOperand, HirType};
use std::collections::HashMap;

/// Type inference engine for Aly High-level Intermediate Representation (HIR).
pub struct TypeInferer {
    var_types: HashMap<String, HirType>,
}

impl TypeInferer {
    /// Create a new type inferer instance.
    pub fn new() -> Self {
        TypeInferer {
            var_types: HashMap::new(),
        }
    }

    /// Perform type inference on a HIR program.
    pub fn infer(&mut self, program: &mut HirProgram) {
        for func in &mut program.functions {
            self.infer_function(func);
        }
    }

    fn infer_function(&mut self, func: &mut HirFunction) {
        self.var_types.clear();
        for param in &func.params {
            // Assume parameters are integers by default if unspecified
            self.var_types.insert(param.clone(), HirType::Int);
        }

        for block in &mut func.body {
            for instr in &mut block.instructions {
                self.infer_instruction(instr);
            }
        }
    }

    fn infer_instruction(&mut self, instr: &mut HirInstruction) {
        match instr {
            HirInstruction::LoadConst { ty, .. } => {
                *ty = HirType::Int;
            }
            HirInstruction::LoadVar { var_name, ty } => {
                if let Some(inferred_ty) = self.var_types.get(var_name) {
                    *ty = inferred_ty.clone();
                } else {
                    *ty = HirType::Int;
                }
            }
            HirInstruction::StoreVar { var_name, value, ty } => {
                self.infer_operand(value);
                let val_ty = self.operand_type(value);
                *ty = val_ty.clone();
                self.var_types.insert(var_name.clone(), val_ty);
            }
            HirInstruction::BinaryOp { op: _, left, right, ty } => {
                self.infer_operand(left);
                self.infer_operand(right);
                let left_ty = self.operand_type(left);
                let right_ty = self.operand_type(right);

                // Type specialization rules
                if left_ty == HirType::Float || right_ty == HirType::Float {
                    *ty = HirType::Float;
                } else if left_ty == HirType::Long || right_ty == HirType::Long {
                    *ty = HirType::Long;
                } else {
                    *ty = HirType::Int;
                }
            }
            HirInstruction::Call { args, ty, .. } => {
                for arg in args {
                    self.infer_operand(arg);
                }
                *ty = HirType::Int; // Default fallback
            }
            HirInstruction::CondJump { condition, .. } => {
                self.infer_operand(condition);
            }
            HirInstruction::Return { value, ty } => {
                if let Some(val) = value {
                    self.infer_operand(val);
                    if let Some(t) = ty {
                        *t = self.operand_type(val);
                    }
                }
            }
            _ => {}
        }
    }

    fn infer_operand(&self, operand: &mut HirOperand) {
        match operand {
            HirOperand::Var { name, ty } => {
                if let Some(inferred_ty) = self.var_types.get(name) {
                    *ty = inferred_ty.clone();
                } else {
                    *ty = HirType::Int;
                }
            }
            HirOperand::Const { ty, .. } => {
                *ty = HirType::Int;
            }
            HirOperand::Temp { ty, .. } => {
                *ty = HirType::Int;
            }
        }
    }

    fn operand_type(&self, operand: &HirOperand) -> HirType {
        match operand {
            HirOperand::Var { ty, .. } => ty.clone(),
            HirOperand::Const { ty, .. } => ty.clone(),
            HirOperand::Temp { ty, .. } => ty.clone(),
        }
    }
}
