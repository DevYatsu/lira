use crate::lexer::str_litteral::StringPart;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    FnDecl {
        name: String,
        params: Vec<(String, Option<Type>)>,
        return_type: Option<Type>,
        body: Vec<Statement>,
    },
    Let {
        atomic: bool,
        lazy: bool,
        mutable: bool,
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
    Spawn {
        body: Vec<Statement>,
        with: Option<Expr>,
    },
    ForLoop {
        iterator: String,
        range: Expr,
        body: Vec<Statement>,
    },
    WhileLoop {
        condition: Expr,
        body: Vec<Statement>,
    },
    If {
        condition: Expr,
        body: Vec<Statement>,
        else_ifs: Vec<(Expr, Vec<Statement>)>,
        else_body: Option<Vec<Statement>>,
    },
    Expr(Expr),
    Return(Expr),
    Break,
    TypeAlias {
        name: String,
        ty: Type,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },
    // import stmt
    Use {
        path: Vec<String>,
        alias: Option<String>,
    }
}

#[derive(Debug, PartialEq)]
pub enum EnumVariant {
    Unit(String),
    Tuple(String, Vec<Type>),
    Struct(String, Vec<(String, Option<Type>)>),
}

#[derive(Debug, PartialEq)]
pub enum EnumVariantData {
    Tuple( Vec<Type>),
    Struct( Vec<(String, Option<Type>)>),
}

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Statement>,
    pub guard: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Literal(Literal),
    Ident(String),
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Pipe(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Range(Box<Expr>, Box<Expr>),
    Parenthesized(Box<Expr>),
    Assign(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    FieldAccess(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
    Closure(Vec<(String, Option<Type>)>, Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Ident(String),
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),
    Array(Vec<Type>),
    Union(Box<Type>, Box<Type>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(Vec<StringPart>),
    Inf,
    Bool(bool),
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}
impl From<f64> for Literal {
    fn from(f: f64) -> Self {
        Literal::Float(f)
    }
}
impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Minus,
}
