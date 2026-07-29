use std::collections::{HashMap, HashSet};
use crate::compiler::mir::{Mir, MirFunction, MirInstruction, MirOperand};

pub struct EscapeAnalysis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    Stack,
    Heap,
}

impl EscapeAnalysis {
    pub fn new() -> Self {
        Self
    }

    /// Run escape analysis on the entire MIR program.
    /// Returns a map of variable names that escape (value = true) vs stack-allocatable (false).
    pub fn analyze_program(&self, mir: &Mir) -> HashMap<String, bool> {
        let mut escaping = HashMap::new();

        for func in &mir.functions {
            let func_escapes = self.analyze_function(func);
            for (var, does_escape) in func_escapes {
                escaping.insert(var, does_escape);
            }
        }

        escaping
    }

    /// Analyze a single function for escaping variables.
    fn analyze_function(&self, func: &MirFunction) -> HashMap<String, bool> {
        let mut escapes: HashMap<String, bool> = HashMap::new();
        let mut written_vars: HashSet<String> = HashSet::new();

        // Pass 1: collect all variables and find initial escape sources
        for block in &func.blocks {
            for instr in &block.instrs {
                match instr {
                    MirInstruction::StoreVar { var, value, .. } => {
                        written_vars.insert(var.clone());

                        let val_escapes = match value {
                            MirOperand::Var { name, .. } => *escapes.get(name).unwrap_or(&false),
                            MirOperand::Const { .. } | MirOperand::Temp { .. } => false,
                        };
                        if val_escapes {
                            escapes.insert(var.clone(), true);
                        }

                        match value {
                            MirOperand::Var { name, .. } => {
                                if *escapes.get(var).unwrap_or(&false) {
                                    escapes.insert(name.clone(), true);
                                }
                            }
                            _ => {}
                        }
                    }

                    MirInstruction::Return { value, .. } => {
                        if let Some(MirOperand::Var { name, .. }) = value {
                            // Returned values escape
                            escapes.insert(name.clone(), true);
                        }
                    }

                    MirInstruction::Call { args, .. } => {
                        for arg in args {
                            if let MirOperand::Var { name, .. } = arg {
                                // Function arguments conservatively escape
                                // (the callee might store them in a global)
                                escapes.insert(name.clone(), true);
                            }
                        }
                    }

                    MirInstruction::CondJump { cond, .. } => {
                        if let MirOperand::Var { name: _, .. } = cond {
                            // Condition variables don't escape just because they're used in a jump
                        }
                    }

                    _ => {}
                }
            }
        }

        // Pass 2: propagate escapes through assignments (transitive closure)
        // If `a = b` and `a` escapes, then `b` escapes too.
        let mut changed = true;
        while changed {
            changed = false;
            for block in &func.blocks {
                for instr in &block.instrs {
                    if let MirInstruction::StoreVar { var, value, .. } = instr {
                        let var_escapes = *escapes.get(var).unwrap_or(&false);
                        if var_escapes {
                            if let MirOperand::Var { name, .. } = value {
                                if !*escapes.get(name).unwrap_or(&false) {
                                    escapes.insert(name.clone(), true);
                                    changed = true;
                                }
                            }
                        }
                        match value {
                            MirOperand::Var { name, .. } => {
                                let val_escapes = *escapes.get(name).unwrap_or(&false);
                                if val_escapes && !*escapes.get(var).unwrap_or(&false) {
                                    escapes.insert(var.clone(), true);
                                    changed = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Pass 3: variables that are never written (only read) don't escape by default
        // But if they're returned or passed to calls, they already escaped above.

        escapes
    }

    /// Check if a specific variable escapes.
    pub fn analyze_escape(var_name: &str, mir: &Mir) -> bool {
        let analysis = Self::new();
        let results = analysis.analyze_program(mir);
        *results.get(var_name).unwrap_or(&true)
    }

    /// Determine allocation strategy for a given variable.
    pub fn allocate_strategy(var_name: &str, mir: &Mir) -> Option<AllocationStrategy> {
        if Self::analyze_escape(var_name, mir) {
            Some(AllocationStrategy::Heap)
        } else {
            Some(AllocationStrategy::Stack)
        }
    }
}
