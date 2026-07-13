/// Centralized error system for the Aly language.
///
/// Design rationale:
/// - All errors from Aly code produce `AlyError`, never a Rust panic.
/// - `AlyErrorKind` classifies errors by category so tooling and users can filter/handle them.
/// - `StackFrame` records Aly-level call-stack information for diagnostics (NOT the Rust backtrace).
/// - The `Display` implementation produces a human-readable, coloured-like diagnostic message
///   pointing directly at the offending source location.
use std::fmt;

// ──────────────────────────────────────────────────────────────────────────────
// Error kind
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum AlyErrorKind {
    /// Invalid token / unexpected token / unclosed brace.
    Syntax,
    /// A value or operation failed at runtime (e.g. division by zero).
    Runtime,
    /// A value has the wrong type for the requested operation.
    Type,
    /// A variable or function name could not be resolved.
    Reference,
    /// A native module could not be found or loaded.
    Import,
    /// An Aly-level exception thrown with `throw`.
    Exception,
    /// An unexpected condition inside the interpreter itself (interpreter bug).
    Internal,
}

impl fmt::Display for AlyErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AlyErrorKind::Syntax => "SyntaxError",
            AlyErrorKind::Runtime => "RuntimeError",
            AlyErrorKind::Type => "TypeError",
            AlyErrorKind::Reference => "ReferenceError",
            AlyErrorKind::Import => "ImportError",
            AlyErrorKind::Exception => "Exception",
            AlyErrorKind::Internal => "InternalError",
        };
        write!(f, "{}", name)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Stack frame
// ──────────────────────────────────────────────────────────────────────────────

/// One frame in the Aly-level call stack.
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function: String,
    pub file: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = self.file.as_deref().unwrap_or("<unknown>");
        write!(
            f,
            "  at {} ({}:{}:{})",
            self.function, file, self.line, self.column
        )
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// AlyError
// ──────────────────────────────────────────────────────────────────────────────

/// The single error type used throughout the Aly interpreter.
///
/// An `AlyError` carries:
/// * `kind`    — the category of the error
/// * `message` — a human-readable description
/// * `line` / `column` — zero-based source position (0 if unknown)
/// * `file`    — optional source file name
/// * `source_line` — the raw source text of the offending line (if available)
/// * `stack`   — Aly call stack at the time of the error
#[derive(Debug, Clone)]
pub struct AlyError {
    pub kind: AlyErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
    /// The raw source line, used to render the ^ pointer in diagnostics.
    pub source_line: Option<String>,
    /// Aly-level stack trace (not the Rust backtrace).
    pub stack: Vec<StackFrame>,
}

impl AlyError {
    // ── Constructors ──────────────────────────────────────────────────────────

    pub fn new(kind: AlyErrorKind, message: impl Into<String>) -> Self {
        AlyError {
            kind,
            message: message.into(),
            line: 0,
            column: 0,
            file: None,
            source_line: None,
            stack: Vec::new(),
        }
    }

    pub fn syntax(message: impl Into<String>) -> Self {
        Self::new(AlyErrorKind::Syntax, message)
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        Self::new(AlyErrorKind::Runtime, message)
    }

    pub fn type_error(message: impl Into<String>) -> Self {
        Self::new(AlyErrorKind::Type, message)
    }

    pub fn reference(message: impl Into<String>) -> Self {
        Self::new(AlyErrorKind::Reference, message)
    }

    pub fn import(message: impl Into<String>) -> Self {
        Self::new(AlyErrorKind::Import, message)
    }

    pub fn exception(message: impl Into<String>) -> Self {
        Self::new(AlyErrorKind::Exception, message)
    }

    /// Used for interpreter-internal bugs. Encourages users to file an issue.
    pub fn internal(message: impl Into<String>) -> Self {
        let msg = format!(
            "{}\n\nO interpretador encontrou um erro interno.\nIsso provavelmente é um bug do Aly.\n\nReporte em:\nhttps://github.com/jefferson-it/Aly/issues",
            message.into()
        );
        Self::new(AlyErrorKind::Internal, msg)
    }

    // ── Builder helpers ───────────────────────────────────────────────────────

    pub fn at(mut self, line: usize, column: usize) -> Self {
        self.line = line;
        self.column = column;
        self
    }

    pub fn in_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn with_source(mut self, source_line: impl Into<String>) -> Self {
        self.source_line = Some(source_line.into());
        self
    }

    pub fn push_frame(mut self, frame: StackFrame) -> Self {
        self.stack.push(frame);
        self
    }

    // ── Formatting helpers ────────────────────────────────────────────────────

    /// Renders the diagnostic pointer:
    /// ```text
    ///   let x = y + 10
    ///           ^
    /// ```
    fn render_pointer(&self) -> String {
        match &self.source_line {
            Some(line_text) => {
                let col = self.column.saturating_sub(1);
                let pointer = " ".repeat(col) + "^";
                format!("\n  {}\n  {}", line_text, pointer)
            }
            None => String::new(),
        }
    }
}

impl fmt::Display for AlyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header
        writeln!(f, "{}", self.kind)?;
        writeln!(f)?;
        writeln!(f, "{}", self.message)?;

        // Location
        if self.line > 0 || self.file.is_some() {
            writeln!(f)?;
            if let Some(file) = &self.file {
                writeln!(f, "arquivo: {}", file)?;
            }
            if self.line > 0 {
                writeln!(f, "linha:   {}", self.line)?;
            }
            if self.column > 0 {
                writeln!(f, "coluna:  {}", self.column)?;
            }

            // Source pointer
            let ptr = self.render_pointer();
            if !ptr.is_empty() {
                write!(f, "{}", ptr)?;
            }
        }

        // Aly stack trace
        if !self.stack.is_empty() {
            writeln!(f)?;
            writeln!(f, "Stack trace (Aly):")?;
            for frame in self.stack.iter().rev() {
                writeln!(f, "{}", frame)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for AlyError {}

// ──────────────────────────────────────────────────────────────────────────────
// Convenience type alias
// ──────────────────────────────────────────────────────────────────────────────

pub type AlyResult<T> = Result<T, AlyError>;

// ──────────────────────────────────────────────────────────────────────────────
// Signal type for non-error control flow
// ──────────────────────────────────────────────────────────────────────────────

/// Signals that can be returned from statement execution.
/// These are **not** errors — they represent normal control-flow changes.
#[derive(Debug, Clone)]
pub enum Signal {
    /// A `return` statement was executed with the given value.
    Return(String),
    /// A `throw` statement was executed with the given value.
    Throw(AlyError),
    /// Normal completion — no special control flow.
    None,
}

impl Signal {
    pub fn is_none(&self) -> bool {
        matches!(self, Signal::None)
    }

    pub fn is_throw(&self) -> bool {
        matches!(self, Signal::Throw(_))
    }

    pub fn is_return(&self) -> bool {
        matches!(self, Signal::Return(_))
    }
}
