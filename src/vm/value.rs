use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// A dynamic value in the Aly VM.
///
/// This uses a straightforward tagged enum.  Future optimisations may replace
/// this with NaN-boxing for better cache locality, but the enum approach keeps
/// the first implementation simple and safe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefTarget {
    Local(usize),
    Global(String),
}

/// Memoized lazy value cell.
#[derive(Clone, Debug)]
pub struct LazyCell {
    pub closure: Closure,
    pub computed: Rc<RefCell<Option<Value>>>,
}

impl LazyCell {
    pub fn new(closure: Closure) -> Self {
        LazyCell {
            closure,
            computed: Rc::new(RefCell::new(None)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Vec(Rc<RefCell<Vec<Value>>>),
    Tuple(Vec<Value>),
    Map(Vec<(Value, Value)>),
    /// A user-defined or native callable.
    Fun(Closure),
    Native(fn(&[Value]) -> Value, &'static str),
    Ref(RefTarget),
    /// A lazily-evaluated value: stored as a closure that produces the value on first access.
    /// Memoized so the closure is evaluated only once.
    Lazy(LazyCell),
    /// A coroutine (resumable function).
    Coroutine(CoroutineState),
}

/// State of a coroutine: running, suspended (with saved stack/IP), or done.
#[derive(Clone, Debug)]
pub enum CoroutineState {
    Suspended {
        func_idx: usize,
        ip: usize,
        stack: Vec<Value>,
        return_ip: usize,
        base: usize,
    },
    Running(usize),
    Done,
}

/// A closure captures a function prototype index and its upvalues.
#[derive(Clone, Debug)]
pub struct Closure {
    /// Index into the VM's function-table (the Chunk with the function's code).
    pub func_idx: usize,
    /// Captured upvalues (closed-over locals from enclosing scopes).
    pub upvalues: Vec<Value>,
}

impl Closure {
    pub fn new(func_idx: usize, upvalue_count: usize) -> Self {
        Closure {
            func_idx,
            upvalues: Vec::with_capacity(upvalue_count),
        }
    }
}

// ── Traits ──────────────────────────────────────────────────────────────────

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "None"),
            Value::Void => write!(f, "void"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Char(c) => write!(f, "{c}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::Vec(v) => {
                let borrowed = v.borrow();
                write!(f, "[")?;
                for (i, item) in borrowed.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Value::Tuple(v) => {
                write!(f, "(")?;
                for (i, item) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, ")")
            }
            Value::Map(pairs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k}: {v}")?;
                }
                write!(f, "}}")
            }
            Value::Fun(c) => write!(f, "<fn {}>", c.func_idx),
            Value::Native(_, name) => write!(f, "<native {name}>"),
            Value::Ref(r) => write!(f, "<ref {:?}>", r),
            Value::Lazy(c) => {
                let guard = c.computed.borrow();
                if guard.is_some() {
                    write!(f, "{}", guard.as_ref().unwrap())
                } else {
                    write!(f, "<lazy fn {}>", c.closure.func_idx)
                }
            }
            Value::Coroutine(s) => match s {
                CoroutineState::Suspended { func_idx, .. } => write!(f, "<coroutine {} suspended>", func_idx),
                CoroutineState::Running(func_idx) => write!(f, "<coroutine {} running>", func_idx),
                CoroutineState::Done => write!(f, "<coroutine done>"),
            },
        }
     }
 }
 
 impl PartialEq for Value {
     fn eq(&self, other: &Self) -> bool {
         match (self, other) {
             (Value::Nil, Value::Nil) => true,
             (Value::Bool(a), Value::Bool(b)) => a == b,
             (Value::Int(a), Value::Int(b)) => a == b,
             (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
             (Value::Char(a), Value::Char(b)) => a == b,
             (Value::Str(a), Value::Str(b)) => a == b,
(Value::Vec(a), Value::Vec(b)) => a.borrow().as_slice() == b.borrow().as_slice(),
               (Value::Tuple(a), Value::Tuple(b)) => a == b,
               (Value::Map(a), Value::Map(b)) => a == b,
             (Value::Fun(_), Value::Fun(_)) => false,
             (Value::Lazy(a), Value::Lazy(b)) => {
                 let ga = a.computed.borrow();
                 let gb = b.computed.borrow();
                 ga.as_ref() == gb.as_ref()
             }
             (Value::Coroutine(_), Value::Coroutine(_)) => false,
             (Value::Native(_, a), Value::Native(_, b)) => a == b,
             (Value::Ref(a), Value::Ref(b)) => a == b,
             _ => false,
         }
     }
 }
 
 /// Truthiness for conditionals: only `false`, `nil`, and `void` are falsy.
/// Lazy values and coroutines are truthy if they haven't failed.
 pub fn is_truthy(v: &Value) -> bool {
     !matches!(v, Value::Nil | Value::Void | Value::Bool(false))
 }
 
 /// Convert a Value to its primitive bool equivalent (used for `not`).
 pub fn to_bool(v: &Value) -> bool {
     matches!(v, Value::Bool(true))
 }
 
 // ── Helpers ─────────────────────────────────────────────────────────────────
 
 impl Value {
     pub fn type_name(&self) -> &'static str {
         match self {
             Value::Nil => "None",
             Value::Void => "void",
             Value::Bool(_) => "bool",
             Value::Int(_) => "int",
             Value::Float(_) => "float",
             Value::Char(_) => "char",
             Value::Str(_) => "string",
              Value::Vec(_) => "vector",
              Value::Tuple(_) => "tuple",
              Value::Map(_) => "object",
              Value::Fun(_) => "function",
              Value::Native(_, _) => "native",
              Value::Ref(_) => "reference",
              Value::Lazy(c) => {
                  if c.computed.borrow().is_some() {
                      c.computed.borrow().as_ref().unwrap().type_name()
                  } else {
                      "lazy"
                  }
              }
              Value::Coroutine(_) => "coroutine",
          }
      }
  }
