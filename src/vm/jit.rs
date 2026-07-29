// src/vm/jit.rs
use std::collections::HashMap;

/// JIT compilation tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitTier {
    Interpreter,
    BaselineJit,
    OptimizedJit,
}

/// Tracks hot code triggers for functions and loop backedges.
pub struct HotCodeDetector {
    /// Maps function index to invocation count.
    invocation_counts: HashMap<usize, usize>,
    /// Maps instruction pointer (loop start) to trip count.
    loop_trip_counts: HashMap<usize, usize>,
    /// Threshold for hot function compilation.
    function_threshold: usize,
    /// Threshold for loop compilation.
    loop_threshold: usize,
}

impl HotCodeDetector {
    /// Create a new hot code detector.
    pub fn new(function_threshold: usize, loop_threshold: usize) -> Self {
        Self {
            invocation_counts: HashMap::new(),
            loop_trip_counts: HashMap::new(),
            function_threshold,
            loop_threshold,
        }
    }

    /// Record a function invocation. Returns true if it should be tiered up.
    pub fn record_invocation(&mut self, func_idx: usize) -> bool {
        let count = self.invocation_counts.entry(func_idx).or_insert(0);
        *count += 1;
        *count >= self.function_threshold
    }

    /// Record a loop backedge execution. Returns true if it should be tiered up.
    pub fn record_loop(&mut self, ip: usize) -> bool {
        let count = self.loop_trip_counts.entry(ip).or_insert(0);
        *count += 1;
        *count >= self.loop_threshold
    }
}

/// Core JIT compilation and adaptive runtime controller.
pub struct JitController {
    pub detector: HotCodeDetector,
    pub compiled_functions: HashMap<usize, JitTier>,
}

impl JitController {
    /// Create a new JIT controller.
    pub fn new() -> Self {
        Self {
            detector: HotCodeDetector::new(50, 100), // Tiering thresholds
            compiled_functions: HashMap::new(),
        }
    }

    /// Adaptively tier up the execution of a function.
    pub fn check_and_tier(&mut self, func_idx: usize) -> JitTier {
        if let Some(&tier) = self.compiled_functions.get(&func_idx) {
            return tier;
        }

        if self.detector.record_invocation(func_idx) {
            eprintln!("Tiering up function {} to BaselineJit!", func_idx);
            self.compiled_functions.insert(func_idx, JitTier::BaselineJit);
            JitTier::BaselineJit
        } else {
            JitTier::Interpreter
        }
    }
}
