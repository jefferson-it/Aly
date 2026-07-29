// src/compiler/mir.rs
// Middle-level Intermediate Representation (MIR) for optimizer passes

/// MIR Program represents a compiled module
#[derive(Debug, Clone)]
pub struct Mir {
    /// All functions in the program
    pub functions: Vec<MirFunction>,
}

/// MIR Function represents a function definition in MIR
#[derive(Debug, Clone)]
pub struct MirFunction {
    /// Function name
    pub name: String,
    /// Parameter names
    pub params: Vec<String>,
    /// Basic block definitions
    pub blocks: Vec<MirBlock>,
    /// Function type information
    pub ret_type: MirType,
}

/// MIR Block represents a basic block of code
#[derive(Debug, Clone)]
pub struct MirBlock {
    /// Block identifier (used for control flow)
    pub id: usize,
    /// Instructions in the block
    pub instrs: Vec<MirInstruction>,
    /// Predecessor block IDs
    pub preds: Vec<usize>,
    /// Successor block IDs
    pub succs: Vec<usize>,
}

/// MIR Instruction represents a single operation in MIR
#[derive(Debug, Clone)]
pub enum MirInstruction {
    /// Load a constant value
    LoadConst { value: i64, ty: MirType },
    /// Load a variable
    LoadVar { var: String, ty: MirType },
    /// Store a value into a variable
    StoreVar { var: String, value: MirOperand, ty: MirType },
    /// Binary operation
    BinaryOp { op: String, left: MirOperand, right: MirOperand, ty: MirType },
    /// Function call
    Call { func: String, args: Vec<MirOperand>, ty: MirType },
    /// Jump to a block
    Jump { target: usize },
    /// Conditional jump
    CondJump { cond: MirOperand, true_target: usize, false_target: usize },
    /// Return from function
    Return { value: Option<MirOperand>, ty: Option<MirType> },
    // Other placeholders for future ops
    // ... more instructions can be added later
}

/// MIR Operand represents a value that can be used in instructions
#[derive(Debug, Clone)]
pub enum MirOperand {
    /// Variable reference
    Var { name: String, ty: MirType },
    /// Constant literal
    Const { value: i64, ty: MirType },
    /// Temporary register (SSA style)
    Temp { id: usize, ty: MirType },
}

/// MIR Type represents the type of a value in the MIR
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MirType {
    /// 32-bit integer
    Int,
    /// 64-bit integer
    Long,
    /// Floating point
    Float,
    /// Object reference
    Object,
    /// Tuple type
    Tuple(Vec<MirType>),
    /// Function type
    Function { params: Vec<MirType>, ret: Box<MirType> },
    /// Void type
    Void,
}