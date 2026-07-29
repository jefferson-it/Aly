// src/compiler/hir_to_mir.rs
use crate::compiler::hir::{HirProgram, HirInstruction, HirOperand, HirType};
use crate::compiler::mir::{Mir, MirFunction, MirBlock, MirInstruction, MirOperand, MirType};

/// Convert a HIR program to a MIR program.
/// This is a basic conversion that preserves structure but uses MIR types.
pub fn hir_to_mir(program: &HirProgram) -> Mir {
    let mut functions = Vec::new();
    for hir_func in &program.functions {
        // Convert each HIR function to MIR function
        // Simplified: create a single basic block for now
        let mut block_instrs = Vec::new();
        for inst in &hir_func.body[0].instructions {
            match inst {
                HirInstruction::LoadConst { value, ty } => {
                    block_instrs.push(MirInstruction::LoadConst {
                        value: *value,
                        ty: ty_to_mir_type(ty),
                    });
                }
                HirInstruction::BinaryOp { op, left, right, ty } => {
                    block_instrs.push(MirInstruction::BinaryOp {
                        op: op.clone(),
                        left: operand_to_mir_operand(left, ty),
                        right: operand_to_mir_operand(right, ty),
                        ty: ty_to_mir_type(ty),
                    });
                }
                _ => {
                    // Ignore other instructions for this simple conversion
                }
            }
        }
        // Create MIR function
        let mir_func = MirFunction {
            name: hir_func.name.clone(),
            params: hir_func.params.clone(),
            blocks: vec![MirBlock {
                id: 0,
                instrs: block_instrs.clone(),
                preds: vec![],
                succs: vec![],
            }],
            ret_type: ty_to_mir_type(&hir_func.ret_type),
        };
        functions.push(mir_func);
    }
    Mir { functions }
}

/// Convert HIR type to MIR type
fn ty_to_mir_type(hir_ty: &HirType) -> MirType {
    match hir_ty {
        HirType::Int => MirType::Int,
        HirType::Long => MirType::Long,
        HirType::Float => MirType::Float,
        HirType::Object => MirType::Object,
        HirType::Tuple(v) => MirType::Tuple(v.iter().map(|p| ty_to_mir_type(p)).collect()),
        HirType::Function { params, ret } => MirType::Function {
            params: params.iter().map(|p| ty_to_mir_type(p)).collect(),
            ret: Box::new(ty_to_mir_type(ret)),
        },
        HirType::Void => MirType::Void,
    }
}

/// Convert HIR operand to MIR operand
fn operand_to_mir_operand(op: &HirOperand, ty: &HirType) -> MirOperand {
    match op {
        HirOperand::Var { name, ty: _ } => MirOperand::Var { name: name.clone(), ty: ty_to_mir_type(ty) },
        HirOperand::Const { value, ty } => MirOperand::Const { value: *value, ty: ty_to_mir_type(ty) },
        HirOperand::Temp { id, ty: _ } => MirOperand::Temp { id: *id, ty: ty_to_mir_type(ty) },
    }
}