// src/compiler/passes.rs
// Optimization pass implementations for the Aly compiler.
use crate::compiler::mir::{Mir, MirOperand, MirInstruction, MirType};
use std::collections::HashMap;

/// Constant propagation pass.
/// Replaces operations with known constant results.
pub struct ConstantPropagator;

impl ConstantPropagator {
    /// Apply constant propagation to the MIR.
    pub fn optimize(&self, mir: &mut Mir) {
        for func in &mut mir.functions {
            let mut known_constants = HashMap::new();
            for block in &mut func.blocks {
                for instr in &mut block.instrs {
                    match instr {
                        MirInstruction::StoreVar { var, value, .. } => {
                            let replacement = if let MirOperand::Var { name, ty } = value {
                                known_constants.get(name).map(|&c| MirOperand::Const { value: c, ty: ty.clone() })
                            } else {
                                None
                            };
                            if let Some(new_val) = replacement {
                                *value = new_val;
                            }
                            if let MirOperand::Const { value: c, .. } = value {
                                known_constants.insert(var.clone(), *c);
                            } else {
                                known_constants.remove(var);
                            }
                        }
                        MirInstruction::BinaryOp { left, right, .. } => {
                            let left_replacement = if let MirOperand::Var { name, ty } = left {
                                known_constants.get(name).map(|&c| MirOperand::Const { value: c, ty: ty.clone() })
                            } else {
                                None
                            };
                            if let Some(new_val) = left_replacement {
                                *left = new_val;
                            }

                            let right_replacement = if let MirOperand::Var { name, ty } = right {
                                known_constants.get(name).map(|&c| MirOperand::Const { value: c, ty: ty.clone() })
                            } else {
                                None
                            };
                            if let Some(new_val) = right_replacement {
                                *right = new_val;
                            }
                        }
                        MirInstruction::Call { args, .. } => {
                            for arg in args {
                                let arg_replacement = if let MirOperand::Var { name, ty } = arg {
                                    known_constants.get(name).map(|&c| MirOperand::Const { value: c, ty: ty.clone() })
                                } else {
                                    None
                                };
                                if let Some(new_val) = arg_replacement {
                                    *arg = new_val;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Copy propagation pass.
/// Eliminates redundant variable copies.
pub struct CopyPropagator;

impl CopyPropagator {
    /// Apply copy propagation to the MIR.
    pub fn optimize(&self, mir: &mut Mir) {
        for func in &mut mir.functions {
            let mut known_copies = HashMap::new();
            for block in &mut func.blocks {
                for instr in &mut block.instrs {
                    match instr {
                        MirInstruction::StoreVar { var, value, .. } => {
                            let replacement = if let MirOperand::Var { name, ty } = value {
                                known_copies.get(name).map(|copied: &String| MirOperand::Var { name: copied.clone(), ty: ty.clone() })
                            } else {
                                None
                            };
                            if let Some(new_val) = replacement {
                                *value = new_val;
                            }
                            if let MirOperand::Var { name, .. } = value {
                                known_copies.insert(var.clone(), name.clone());
                            } else {
                                known_copies.remove(var);
                            }
                        }
                        MirInstruction::BinaryOp { left, right, .. } => {
                            let left_replacement = if let MirOperand::Var { name, ty } = left {
                                known_copies.get(name).map(|copied: &String| MirOperand::Var { name: copied.clone(), ty: ty.clone() })
                            } else {
                                None
                            };
                            if let Some(new_val) = left_replacement {
                                *left = new_val;
                            }

                            let right_replacement = if let MirOperand::Var { name, ty } = right {
                                known_copies.get(name).map(|copied: &String| MirOperand::Var { name: copied.clone(), ty: ty.clone() })
                            } else {
                                None
                            };
                            if let Some(new_val) = right_replacement {
                                *right = new_val;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Strength reduction pass.
/// Replaces expensive operations with cheaper equivalents.
pub struct StrengthReducer;

impl StrengthReducer {
    /// Apply strength reduction to the MIR.
    pub fn optimize(&self, mir: &mut Mir) {
        for func in &mut mir.functions {
            for block in &mut func.blocks {
                for instr in &mut block.instrs {
                    let mut is_mul_pow_two = false;
                    let mut shift_val = 0;
                    if let MirInstruction::BinaryOp { op, right, .. } = instr {
                        if op == "*" {
                            if let MirOperand::Const { value, .. } = right {
                                if *value > 0 && (*value & (*value - 1)) == 0 {
                                    is_mul_pow_two = true;
                                    shift_val = value.trailing_zeros() as i64;
                                }
                            }
                        }
                    }
                    if is_mul_pow_two {
                        if let MirInstruction::BinaryOp { op, right, .. } = instr {
                            *op = "<<".to_string();
                            *right = MirOperand::Const { value: shift_val, ty: MirType::Int };
                        }
                    }
                }
            }
        }
    }
}