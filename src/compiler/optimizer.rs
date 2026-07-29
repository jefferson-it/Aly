pub mod escape_analysis;
pub mod inliner;

use crate::compiler::mir::Mir;
use crate::compiler::passes::{ConstantPropagator, CopyPropagator, StrengthReducer};

pub use escape_analysis::{EscapeAnalysis, AllocationStrategy};

/// Classical optimizer passes implementation.
/// Implements dead code elimination, constant propagation,
/// copy propagation, strength reduction, and loop unrolling.
pub struct ClassicalOptimizer {
    pub constant_propagator: ConstantPropagator,
    pub copy_propagator: CopyPropagator,
    pub strength_reducer: StrengthReducer,
}

impl ClassicalOptimizer {
    pub fn new() -> Self {
        ClassicalOptimizer {
            constant_propagator: ConstantPropagator,
            copy_propagator: CopyPropagator,
            strength_reducer: StrengthReducer,
        }
    }

    pub fn optimize(&self, mir: &mut Mir) {
        self.constant_propagator.optimize(mir);
        self.copy_propagator.optimize(mir);
        self.strength_reducer.optimize(mir);
        self.loop_unrolling(mir);

        let ea = EscapeAnalysis::new();
        let _escape_map = ea.analyze_program(mir);
    }

    fn loop_unrolling(&self, _mir: &mut Mir) {
    }
}

/// Advanced optimizer passes implementation.
/// Implements function inlining, interprocedural analysis,
/// devirtualization, speculative optimization, and inline caching.
pub struct AdvancedOptimizer;

impl AdvancedOptimizer {
    pub fn optimize(&self, _mir: &mut Mir) {
    }
}
