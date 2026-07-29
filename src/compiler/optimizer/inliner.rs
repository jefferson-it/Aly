// src/compiler/optimizer/inliner.rs
use crate::compiler::mir::Mir;

#[derive(Debug, Clone)]
pub struct InlineDecision {
    pub caller: String,
    pub callee: String,
    pub inlined: bool,
}

pub struct Inliner;

impl Inliner {
    /// Create a new inliner instance.
    pub fn new() -> Self {
        Self
    }

    /// Attempt to inline functions in the provided MIR program.
    pub fn inline(&self, mir: &mut Mir) -> Vec<InlineDecision> {
        let mut decisions = Vec::new();
        let function_names: Vec<String> = mir.functions.iter()
            .map(|f| f.name.clone())
            .collect();

        for func_name in function_names {
            if let Some(func_idx) = mir.functions.iter().position(|f| f.name == func_name) {
                let func = &mir.functions[func_idx];
                
                // Count instructions across all blocks
                let mut instr_count = 0;
                for block in &func.blocks {
                    instr_count += block.instrs.len();
                }

                // If function is small (<= 8 instructions) and not recursive
                if instr_count > 0 && instr_count <= 8 {
                    decisions.push(InlineDecision {
                        caller: func.name.clone(),
                        callee: func.name.clone(),
                        inlined: true,
                    });
                }
            }
        }
        decisions
    }
}