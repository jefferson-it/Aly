use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Physical registers available for allocation
#[allow(missing_docs)]
pub enum PhysicalRegister {
    /// General purpose registers
    RAX, RBX, RCX, RDX, RSI, RDI, R8, R9, R10, R11,
    /// Float/SIMD registers
    XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7,
    /// Special purpose registers
    RSP, RBP, RFLAGS, RIP, CS, DS, ES, FS, GS,
    /// Vector registers (AVX)
    YMM0, YMM1, YMM2, YMM3, YMM4, YMM5, YMM6, YMM7,
}

/// Virtual register for register allocation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct VirtualRegister {
    /// Unique identifier
    pub id: usize,
    /// Source instruction index
    pub def_at: usize,
    /// Def site line number for debugging
    pub def_line: usize,
    /// Use sites for this virtual register
    pub uses: Vec<usize>,
    /// Current spill weight (importance)
    pub weight: usize,
}

/// Spill information for when registers are constrained
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct SpillInfo {
    /// Virtual register that needs to be spilled
    pub virtual_id: usize,
    /// Target stack slot
    pub stack_slot: i32,
    /// Cost/benefit of spilling
    pub cost: usize,
    /// Reason for spilling
    pub reason: SpillReason,
}

/// Reasons for spilling a virtual register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum SpillReason {
    /// No registers available
    NoRegisters,
    /// Register pressure too high
    RegisterPressure,
    /// Interference with critical registers
    Interference,
    /// Spill for performance (hot path optimization)
    Performance,
}

/// Register allocator for the new register-based VM.
/// 
/// Implements graph coloring algorithm with live range analysis and spill optimization.
/// Key features:
/// - Graph coloring for register allocation
/// - Live range analysis using interference graphs
/// - Spill code generation when registers are constrained
/// - Optimization for both interpreted and compiled modes
/// 
/// ## Tasks Supported
/// - Design graph coloring register allocator (Task 2.1)
/// - Implement live range analysis (Task 2.1)
/// - Add spill code generation (Task 2.1)
/// 
/// ## Performance Targets
/// - Target: <10% spilling for typical functions (Task 2.1 validation)
/// - Compare: Against LLVM register allocator (Task 2.1 validation)
pub struct Allocator {
    /// Physical registers available
    registers: Vec<PhysicalRegister>,
    /// Virtual registers currently allocated
    virt_regs: Vec<VirtualRegister>,
    /// Interference graph (register conflicts)
    interference: HashMap<usize, Vec<usize>>,
    /// Spilling decisions for constrained virtual regs
    spills: Vec<SpillInfo>,
}

impl Allocator {
    /// Create a new register allocator with default configuration
    pub fn new() -> Self {
        Self {
            registers: Vec::new(),
            virt_regs: Vec::new(),
            interference: HashMap::new(),
            spills: Vec::new(),
        }
    }
    
    /// Initialize allocator with architecture-specific registers
    pub fn init(&mut self) {
        // Initialize with common x86_64 registers
        self.registers = vec![
            PhysicalRegister::RAX, PhysicalRegister::RBX, PhysicalRegister::RCX,
            PhysicalRegister::RDX, PhysicalRegister::RSI, PhysicalRegister::RDI,
            PhysicalRegister::R8, PhysicalRegister::R9, PhysicalRegister::R10,
            PhysicalRegister::R11, PhysicalRegister::XMM0, PhysicalRegister::XMM1,
            PhysicalRegister::XMM2, PhysicalRegister::XMM3, PhysicalRegister::XMM4,
            PhysicalRegister::XMM5, PhysicalRegister::XMM6, PhysicalRegister::XMM7,
        ];
        // Initialize interference graph
        for i in 0..(self.registers.len()) {
            self.interference.insert(i, Vec::new());
        }
    }
    
    /// Add a virtual register to be allocated
    pub fn add_virtual(&mut self, id: usize, def_at: usize, def_line: usize) {
        self.virt_regs.push(VirtualRegister {
            id,
            def_at,
            def_line,
            uses: Vec::new(),
            weight: 1,
        });
    }
    
    /// Add a use relationship between virtual registers
    pub fn add_use(&mut self, virt_id: usize, use_at: usize) {
        if let Some(vreg) = self.virt_regs.iter_mut().find(|v| v.id == virt_id) {
            vreg.uses.push(use_at);
        }
    }
    
    /// Add interference between two virtual registers
    pub fn add_interference(&mut self, v1: usize, v2: usize) {
        self.interference.entry(v1).or_default().push(v2);
        self.interference.entry(v2).or_default().push(v1);
    }
    
    /// Compute live ranges using simple algorithm
    /// 
    /// Live range: interval [def_at, last_use_at]
    fn compute_live_ranges(&self) -> Vec<LiveRange> {
        let mut ranges: Vec<LiveRange> = Vec::new();
        
        for vreg in &self.virt_regs {
            if vreg.uses.is_empty() {
                // Single def, no uses - register can be reused
                ranges.push(LiveRange {
                    id: vreg.id,
                    start: vreg.def_at,
                    end: vreg.def_at,
                });
            } else {
                // Find last use
                let last_use = *vreg.uses.iter().max().unwrap();
                ranges.push(LiveRange {
                    id: vreg.id,
                    start: vreg.def_at,
                    end: last_use,
                });
            }
        }
        
        ranges
    }
    
    /// Perform graph coloring register allocation
    /// 
    /// This is the main allocation routine that:
    /// 1. Builds interference graph
    /// 2. Attempts to color using available registers
    /// 3. Generates spills when needed
    /// 4. Returns allocation decisions
    pub fn allocate(&mut self) -> Vec<AllocationDecision> {
        let mut decisions: Vec<AllocationDecision> = Vec::new();
        let ranges = self.compute_live_ranges();
        
        // Sort ranges by live interval length (shortest first - classic heuristic)
        let mut sorted_ranges = ranges;
        sorted_ranges.sort_by(|a, b| (b.end - b.start).cmp(&(a.end - a.start)));
        
        let mut allocated: HashMap<usize, PhysicalRegister> = HashMap::new();
        let mut stack_pos: i32 = -1; // Stack slot position
        
        for range in &sorted_ranges {
            if allocated.contains_key(&range.id) {
                continue; // Already allocated
            }
            
            // Try to find an available register that doesn't interfere
            let mut assigned = false;
            
            // Try each register from the preferred order
            for &reg in self.registers.iter().rev() {
                let _reg_idx = self.get_register_index(reg);
                let interferes = self.interference.get(&range.id)
                    .map_or(false, |interfere_list| {
                        interfere_list.iter().any(|&interfere| allocated.contains_key(&interfere))
                    });
                
                if !interferes {
                    allocated.insert(range.id, reg);
                    assigned = true;
                    
                    decisions.push(AllocationDecision {
                        virtual_id: range.id,
                        physical_reg: Some(reg),
                        stack_slot: None,
                    });
                    
                    break;
                }
            }
            
            if !assigned {
                // Need to spill
                stack_pos -= 1;
                allocated.insert(range.id, *self.registers.last().unwrap());
                
                self.spills.push(SpillInfo {
                    virtual_id: range.id,
                    stack_slot: stack_pos,
                    cost: range.end - range.start,
                    reason: SpillReason::NoRegisters,
                });
                
                decisions.push(AllocationDecision {
                    virtual_id: range.id,
                    physical_reg: Some(*self.registers.last().unwrap()),
                    stack_slot: Some(stack_pos),
                });
            }
        }
        
        decisions
    }
    
    /// Helper to get register index for interference checking
    fn get_register_index(&self, reg: PhysicalRegister) -> usize {
        self.registers.iter()
            .position(|&r| r == reg)
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
struct LiveRange {
    id: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
pub struct AllocationDecision {
    pub virtual_id: usize,
    pub physical_reg: Option<PhysicalRegister>,
    pub stack_slot: Option<i32>,
}