#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, init: Option<Expr>, attrs: Vec<Attribute> },
    Const { name: String, init: Expr, attrs: Vec<Attribute> },
    Assign { target: Expr, value: Expr },
    Expr(Expr),
    Return(Option<Expr>),
    Break,
    Continue,
    Throw(Expr),
    Try {
        body: Vec<Stmt>,
        catch_var: Option<String>,
        catch_body: Vec<Stmt>,
        finally_body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        elifs: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Expr>,
        body: Vec<Stmt>,
    },
    DoWhile {
        body: Vec<Stmt>,
        condition: Expr,
    },
    Foreach {
        item: String,
        collection: Expr,
        body: Vec<Stmt>,
    },
    Fun {
        name: String,
        params: Vec<String>,
        defaults: Vec<Option<Expr>>,
        variadic: Option<String>,
        body: Vec<Stmt>,
        attrs: Vec<Attribute>,
    },
    Match {
        scrutinee: Expr,
        arms: Vec<MatchArm>,
    },
    Destructure {
        names: Vec<String>,
        init: Expr,
        is_const: bool,
        is_array: bool,
    },
    Lazy {
        name: String,
        init: Expr,
    },
    MacroDef {
        name: String,
        patterns: Vec<Vec<Expr>>,
        bodies: Vec<Vec<Stmt>>,
    },
    Coroutine {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Yield(Option<Expr>),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub patterns: Vec<Pattern>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Expr),
    Range(Expr, Expr),
    Wildcard,
    Or(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bool(bool),
    None,
    Void,
    Var(String),
    TemplateStr(Vec<TemplatePart>),
    UnaryOp {
        op: String,
        expr: Box<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        function: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    PropAccess {
        object: Box<Expr>,
        prop: String,
    },
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    Object(Vec<(String, Expr)>),
    Percent(Box<Expr>),
    AnonymousFun {
        params: Vec<String>,
        defaults: Vec<Option<Expr>>,
        variadic: Option<String>,
        body: Vec<Stmt>,
    },
    MacroExpand {
        name: String,
        args: Vec<Expr>,
    },
    YieldExpr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum TemplatePart {
    Text(String),
    Var(String),
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}
