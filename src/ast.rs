use crate::parser::string::ParserStringPart;

#[derive(Debug, PartialEq, Clone)]
pub struct Program<'i> {
    pub statements: Vec<Statement<'i>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'i> {
    FnDecl {
        name: &'i str,
        _async: bool,
        params: Vec<(&'i str, Option<Type<'i>>)>,
        return_type: Option<Type<'i>>,
        body: Vec<Statement<'i>>,
    },
    Let {
        atomic: bool,
        lazy: bool,
        mutable: bool,
        name: LetName<'i>,
        ty: Option<Type<'i>>,
        value: Expr<'i>,
    },
    Match(Match<'i>),
    Spawn {
        body: Vec<Statement<'i>>,
        with: Option<Expr<'i>>,
    },
    ForLoop {
        iterator: &'i str,
        range: Expr<'i>,
        body: Vec<Statement<'i>>,
    },
    WhileLoop {
        condition: Expr<'i>,
        body: Vec<Statement<'i>>,
    },
    If {
        condition: Expr<'i>,
        body: Vec<Statement<'i>>,
        else_ifs: Vec<(Expr<'i>, Vec<Statement<'i>>)>,
        else_body: Option<Vec<Statement<'i>>>,
    },
    Expr(Expr<'i>),
    Return(Expr<'i>),
    Break,
    TypeAlias {
        name: &'i str,
        ty: Type<'i>,
    },
    Struct {
        name: &'i str,
        fields: Vec<(&'i str, Type<'i>)>,
    },
    Enum {
        name: &'i str,
        variants: Vec<EnumVariant<'i>>,
    },
    // import stmt
    Use {
        path: Vec<&'i str>,
        alias: Option<&'i str>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Match<'i> {
    pub expr: Expr<'i>,
    pub arms: Vec<MatchArm<'i>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LetName<'i> {
    Ident(&'i str),
    ArrayDestructure(Vec<LetName<'i>>),
    TupleDestructure(Vec<LetName<'i>>),
    StructDestructure(&'i str, Vec<(&'i str, Option<&'i str>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariant<'i> {
    Unit(&'i str),
    Tuple(&'i str, Vec<Type<'i>>),
    Struct(&'i str, Vec<(&'i str, Option<Type<'i>>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnumVariantData<'i> {
    Tuple(Vec<Type<'i>>),
    Struct(Vec<(&'i str, Option<Type<'i>>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MatchArm<'i> {
    pub pattern: Pattern<'i>,
    pub body: Vec<Statement<'i>>,
    pub guard: Option<Expr<'i>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern<'i> {
    Literal(Literal<'i>),
    Ident(&'i str),
    FunctionDestructor(&'i str, Vec<&'i str>),
    StructLikeDestructor(&'i str, Vec<(&'i str, Option<&'i str>)>),
    Wildcard,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'i> {
    Literal(Literal<'i>),
    Ident(&'i str),
    Array(Vec<Expr<'i>>),
    Match(Box<Match<'i>>),
    Awaitable(Box<Expr<'i>>),

    // parenthesized expr<'i>ession is a tuple
    Tuple(Vec<Expr<'i>>),
    Binary(Box<Expr<'i>>, BinOp, Box<Expr<'i>>),
    Pipe(Box<Expr<'i>>, Box<Expr<'i>>),
    Call(Box<Expr<'i>>, Vec<Expr<'i>>),
    Range(Box<Expr<'i>>, Box<Expr<'i>>),
    Assign(Box<Expr<'i>>, BinOp, Box<Expr<'i>>),
    Unary(UnaryOp, Box<Expr<'i>>),
    FieldAccess(Box<Expr<'i>>, &'i str),
    Index(Box<Expr<'i>>, Box<Expr<'i>>),
    Closure(Vec<(&'i str, Option<Type<'i>>)>, Vec<Statement<'i>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comment<'i> {
    Line(&'i str),
    Block(&'i str),
    Doc(&'i str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type<'i> {
    Ident(&'i str),
    Tuple(Vec<Type<'i>>),
    Function(Vec<Type<'i>>, Box<Type<'i>>),
    AsyncFunction(Vec<Type<'i>>, Box<Type<'i>>),
    Array(Vec<Type<'i>>),
    Awaitable(Box<Type<'i>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'i> {
    Int(i32),
    Float(f64),
    String(Vec<ParserStringPart<'i>>),
    Inf,
    Bool(bool),
}

impl From<i32> for Literal<'_> {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}
impl From<f64> for Literal<'_> {
    fn from(f: f64) -> Self {
        Literal::Float(f)
    }
}
impl From<bool> for Literal<'_> {
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
    Plus,
    BitNot,
}
