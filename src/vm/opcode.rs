/// Bytecode instruction opcodes for the Aly VM.
///
/// Every instruction is encoded as a single `u32`:
///
/// ```text
/// bits 0-7   → opcode
/// bits 8-15  → operand A (u8)
/// bits 16-23 → operand B (u8)
/// bits 24-31 → reserved
/// ```
///
/// When a 16-bit signed operand is needed, A and B are combined as a little-endian
/// i16.  Multi-word instructions (e.g. LOAD_INT with a large immediate) use
/// consecutive `u32` words.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpCode {
    // ── Stack ────────────────────────────────────────────────────────────────
    Return = 0,
    Pop,
    Dup,
    // ── Constants / Literals ─────────────────────────────────────────────────
    Const,            // push constant at pool index A
    Nil,              // push nil
    True,             // push true
    False,            // push false
    LoadInt,          // push small int (A = value ∈ 0..255)
    LoadIntLong,      // 3-word: push any i64
    // ── Variables ────────────────────────────────────────────────────────────
    DefineGlobal,     // pop, store as global[A]
    GetGlobal,        // push global[A]
    SetGlobal,        // pop value, set global[A]
    GetLocal,         // push local[A]
    SetLocal,         // pop, store to local[A]
    InitLocal,        // copy stack_top-1 → locals[A] (no pop, for codegen init)
    GetUpvalue,       // push upvalue[A]
    SetUpvalue,       // pop, store to upvalue[A]
    // ── Jumps ────────────────────────────────────────────────────────────────
    Jump,             // unconditional → A+B as i16
    JumpIfFalse,      // pop, jump if falsey    → A+B as i16
    JumpIfTrue,       // pop, jump if truthy    → A+B as i16
    Loop,             // negative loop jump     → A+B as i16
    // ── Functions ────────────────────────────────────────────────────────────
    Call,             // call; A = arg count (incl. receiver)
    Closure,          // create closure; A = function-table index
    // ── Arithmetic ───────────────────────────────────────────────────────────
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Negate,
    // ── Comparison ───────────────────────────────────────────────────────────
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    // ── Logical ──────────────────────────────────────────────────────────────
    Not,
    // ── Strings ──────────────────────────────────────────────────────────────
    Concat,
    // ── Data structures ──────────────────────────────────────────────────────
    BuildList,        // A = number of items on stack to pop
    BuildTuple,       // A = number of items on stack to pop
    BuildMap,         // A = number of key-value pairs (each is 2 stack slots)
    Subscript,        // pop key, pop obj → push obj[key]
    SubscriptSet,     // pop value, pop key, pop obj → obj[key] = value
    // ── Misc ─────────────────────────────────────────────────────────────────
    Print,            // pop and print top (for REPL / debug)
    Assert,           // pop, panic if falsey
    RegisterLoad,
    RegisterStore,
    RegisterAdd,
    RegisterSub,
    RegisterMul,
    RegisterDiv,
    GetRefLocal,
    GetRefGlobal,
    // ── Lazy Evaluation ────────────────────────────────────────────────────
    ForceLazy,        // force evaluation of a lazy value on stack
    // ── Coroutines ──────────────────────────────────────────────────────────
    Yield,            // yield from coroutine
    MakeCoroutine,    // wrap function as coroutine
    Resume,           // resume a suspended coroutine
    Void,             // push void
}

/// Total number of valid opcodes.
pub const OP_COUNT: usize = 66;
pub const OP_UNUSED: u8 = 0xFF;

// ── Encode / decode helpers ─────────────────────────────────────────────────

#[inline]
pub fn encode(op: OpCode, a: u8, b: u8) -> u32 {
    (op as u32) | ((a as u32) << 8) | ((b as u32) << 16)
}

#[inline]
pub fn decode_op(insn: u32) -> OpCode {
    // Safety: we guarantee only valid opcodes are written.
    unsafe { std::mem::transmute((insn & 0xFF) as u8) }
}

#[inline]
pub fn decode_a(insn: u32) -> u8 {
    ((insn >> 8) & 0xFF) as u8
}

#[inline]
pub fn decode_b(insn: u32) -> u8 {
    ((insn >> 16) & 0xFF) as u8
}

/// Signed 16-bit offset from A+B bytes (little-endian).
#[inline]
pub fn decode_offset(insn: u32) -> i16 {
    let lo = ((insn >> 8) & 0xFF) as u16;
    let hi = ((insn >> 16) & 0xFF) as u16;
    i16::from_le_bytes([lo as u8, hi as u8])
}

// ── Metadata ────────────────────────────────────────────────────────────────

/// How many `u32` words each opcode occupies.
pub fn opcode_width(op: OpCode) -> usize {
    match op {
        OpCode::LoadIntLong => 3, // opcode word + two data words
        OpCode::Closure => 1,     // opcode word only; upvalue data follows as extra words
        _ => 1,
    }
}

/// Human name for debugging / disassembly.
pub fn opcode_name(op: OpCode) -> &'static str {
    match op {
        OpCode::Return => "RETURN",
        OpCode::Pop => "POP",
        OpCode::Dup => "DUP",
        OpCode::Const => "CONST",
        OpCode::Nil => "NIL",
        OpCode::True => "TRUE",
        OpCode::False => "FALSE",
        OpCode::LoadInt => "LOAD_INT",
        OpCode::LoadIntLong => "LOAD_INT_LONG",
        OpCode::DefineGlobal => "DEF_GLOBAL",
        OpCode::GetGlobal => "GET_GLOBAL",
        OpCode::SetGlobal => "SET_GLOBAL",
        OpCode::GetLocal => "GET_LOCAL",
        OpCode::SetLocal => "SET_LOCAL",
        OpCode::InitLocal => "INIT_LOCAL",
        OpCode::GetUpvalue => "GET_UPVALUE",
        OpCode::SetUpvalue => "SET_UPVALUE",
        OpCode::Jump => "JUMP",
        OpCode::JumpIfFalse => "JUMP_IF_FALSE",
        OpCode::JumpIfTrue => "JUMP_IF_TRUE",
        OpCode::Loop => "LOOP",
        OpCode::Call => "CALL",
        OpCode::Closure => "CLOSURE",
        OpCode::Add => "ADD",
        OpCode::Subtract => "SUB",
        OpCode::Multiply => "MUL",
        OpCode::Divide => "DIV",
        OpCode::Modulus => "MOD",
        OpCode::Negate => "NEG",
        OpCode::Equal => "EQ",
        OpCode::NotEqual => "NEQ",
        OpCode::Less => "LT",
        OpCode::Greater => "GT",
        OpCode::LessEqual => "LTE",
        OpCode::GreaterEqual => "GTE",
        OpCode::Not => "NOT",
        OpCode::Concat => "CONCAT",
        OpCode::BuildList => "BUILD_LIST",
        OpCode::BuildTuple => "BUILD_TUPLE",
        OpCode::BuildMap => "BUILD_MAP",
        OpCode::Subscript => "SUBSCRIPT",
        OpCode::SubscriptSet => "SUBSCRIPT_SET",
        OpCode::Print => "PRINT",
        OpCode::Assert => "ASSERT",
        OpCode::RegisterLoad => "REG_LOAD",
        OpCode::RegisterStore => "REG_STORE",
        OpCode::RegisterAdd => "REG_ADD",
        OpCode::RegisterSub => "REG_SUB",
        OpCode::RegisterMul => "REG_MUL",
        OpCode::RegisterDiv => "REG_DIV",
        OpCode::GetRefLocal => "GET_REF_LOCAL",
        OpCode::GetRefGlobal => "GET_REF_GLOBAL",
        OpCode::ForceLazy => "FORCE_LAZY",
        OpCode::Yield => "YIELD",
        OpCode::MakeCoroutine => "MAKE_COROUTINE",
        OpCode::Resume => "RESUME",
        OpCode::Void => "VOID",
    }
}
