#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, init: Option<Expr> },
    Const { name: String, init: Expr },
    Assign { target: Expr, value: Expr },
    Expr(Expr),
    Return(Option<Expr>),
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
    Fun {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Match {
        scrutinee: Expr,
        arms: Vec<MatchArm>,
    },
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
    Str(String),
    Bool(bool),
    None,
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
    Object(Vec<(String, Expr)>),
    Percent(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum TemplatePart {
    Text(String),
    Var(String),
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}
