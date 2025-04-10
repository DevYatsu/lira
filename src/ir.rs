// ir stands for intermediate representation
// This module defines the intermediate representation of the language
// The IR is a simplified version of the source code that is easier to analyze and manipulate

use interner::Symbol;

use crate::ast::{BinOp, UnaryOp};
pub mod context;
pub mod interner;
mod table;

// Lowered IR representation for interpreted or compiled execution
#[derive(Debug, PartialEq, Clone)]
pub struct IrProgram {
    pub statements: Vec<IrStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrStatement {
    FnDecl {
        name: Symbol,
        _async: bool,
        params: Vec<(Symbol, Option<IrType>)>,
        return_type: Option<IrType>,
        body: Vec<IrStatement>,
    },
    Let {
        atomic: bool,
        lazy: bool,
        mutable: bool,
        name: IrLetPattern,
        ty: Option<IrType>,
        value: IrExpr,
    },
    Match(IrMatch),
    Spawn {
        body: Vec<IrStatement>,
        with: Option<IrExpr>,
    },
    ForLoop {
        iterator: Symbol,
        range: IrExpr,
        body: Vec<IrStatement>,
    },
    WhileLoop {
        condition: IrExpr,
        body: Vec<IrStatement>,
    },
    If {
        condition: IrExpr,
        body: Vec<IrStatement>,
        else_ifs: Vec<(IrExpr, Vec<IrStatement>)>,
        else_body: Option<Vec<IrStatement>>,
    },
    Expr(IrExpr),
    Return(IrExpr),
    Break,
    TypeAlias {
        name: Symbol,
        ty: IrType,
    },
    Struct {
        name: Symbol,
        fields: Vec<(Symbol, IrType)>,
    },
    Enum {
        name: Symbol,
        variants: Vec<IrEnumVariant>,
    },
    Use {
        path: Vec<Symbol>,
        alias: Option<Symbol>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrLetPattern {
    Ident(Symbol),
    Array(Vec<IrLetPattern>),
    Tuple(Vec<IrLetPattern>),
    Struct(Symbol, Vec<(Symbol, Option<Symbol>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrEnumVariant {
    Unit(Symbol),
    Tuple(Symbol, Vec<IrType>),
    Struct(Symbol, Vec<(Symbol, Option<IrType>)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IrMatch {
    pub expr: IrExpr,
    pub arms: Vec<IrMatchArm>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IrMatchArm {
    pub pattern: IrPattern,
    pub body: Vec<IrStatement>,
    pub guard: Option<IrExpr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrPattern {
    Literal(IrLiteral),
    Ident(Symbol),
    FunctionDestructor(Symbol, Vec<Symbol>),
    StructLikeDestructor(Symbol, Vec<(Symbol, Option<Symbol>)>),
    Wildcard,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrExpr {
    Literal(IrLiteral),
    Symbol(Symbol),
    Array(Vec<IrExpr>),
    Match(Box<IrMatch>),
    Awaitable(Box<IrExpr>),
    Tuple(Vec<IrExpr>),
    Binary(Box<IrExpr>, BinOp, Box<IrExpr>),
    Pipe(Box<IrExpr>, Box<IrExpr>),
    Call(Box<IrExpr>, Vec<IrExpr>),
    Range(Box<IrExpr>, Box<IrExpr>),
    Assign(Box<IrExpr>, BinOp, Box<IrExpr>),
    Unary(UnaryOp, Box<IrExpr>),
    FieldAccess(Box<IrExpr>, Symbol),
    Index(Box<IrExpr>, Box<IrExpr>),
    Closure(Vec<(Symbol, Option<IrType>)>, Vec<IrStatement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrLiteral {
    Int(i32),
    Float(f64),
    String(Vec<IrStringPart>),
    Inf,
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrStringPart {
    Text(String),
    Expression(IrExpr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrType {
    Ident(Symbol),
    Tuple(Vec<IrType>),
    Function(Vec<IrType>, Box<IrType>),
    AsyncFunction(Vec<IrType>, Box<IrType>),
    Array(Vec<IrType>),
    Awaitable(Box<IrType>),
}
