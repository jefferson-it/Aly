// src/compiler/hir.rs
// High-level Intermediate Representation (HIR) with typed information

/// HIR Program represents the root of a compiled module
#[derive(Debug, Clone)]
pub struct HirProgram {
    /// All top-level functions in the program
    pub functions: Vec<HirFunction>,
}

/// HIR Function represents a function definition
#[derive(Debug, Clone)]
pub struct HirFunction {
    /// Function name
    pub name: String,
    /// Parameters
    pub params: Vec<String>,
    /// Function body as basic blocks
    pub body: Vec<HirBlock>,
    /// Type information for the function
    pub ret_type: HirType,
}

/// HIR Block represents a basic block of code
#[derive(Debug, Clone)]
pub struct HirBlock {
    /// Name of the block (e.g., for control flow)
    pub name: Option<String>,
    /// Instructions in the block
    pub instructions: Vec<HirInstruction>,
    /// Incoming edges from predecessor blocks
    pub predecessors: Vec<usize>,
    /// Outgoing edges to successor blocks
    pub successors: Vec<usize>,
}

/// HIR Instruction represents a single operation
#[derive(Debug, Clone)]
pub enum HirInstruction {
    /// Load a constant value
    LoadConst { value: i64, ty: HirType },
    /// Load a variable by name
    LoadVar { var_name: String, ty: HirType },
    /// Store a value into a variable
    StoreVar { var_name: String, value: HirOperand, ty: HirType },
    /// Binary operation (+, -, *, /, etc.)
    BinaryOp { op: String, left: HirOperand, right: HirOperand, ty: HirType },
    /// Function call
    Call { func_name: String, args: Vec<HirOperand>, ty: HirType },
    /// Jump to another block
    Jump { target: usize },
    /// Conditional jump
    CondJump { condition: HirOperand, true_target: usize, false_target: usize },
    /// Return from function
    Return { value: Option<HirOperand>, ty: Option<HirType> },
    // Other placeholders for future ops
    // ... more instructions can be added later
}

/// HIR Operand represents a value that can be used in instructions
#[derive(Debug, Clone)]
pub enum HirOperand {
    /// Variable reference
    Var { name: String, ty: HirType },
    /// Constant literal
    Const { value: i64, ty: HirType },
    /// Temporary register (SSA style)
    Temp { id: usize, ty: HirType },
}

/// HIR Type represents the type of a value in the HIR
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HirType {
    /// 32-bit integer
    Int,
    /// 64-bit integer
    Long,
    /// Floating point
    Float,
    /// Object reference
    Object,
    /// Tuple type
    Tuple(Vec<HirType>),
    /// Function type (used for function pointers)
    Function { params: Vec<HirType>, ret: Box<HirType> },
    /// Void type
    Void,
}