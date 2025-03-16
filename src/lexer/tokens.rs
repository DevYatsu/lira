use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
};

use super::str_litteral::{StringPart, process_string_literal};
use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    InvalidFloat(String),
    InvalidBinary(String),
    InvalidOctal(String),
    InvalidHex(String),

    #[default]
    InvalidToken,
    UnterminatedString(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        LexingError::InvalidInteger(err.to_string())
    }
}
impl From<ParseFloatError> for LexingError {
    fn from(err: ParseFloatError) -> Self {
        LexingError::InvalidFloat(err.to_string())
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
#[logos(subpattern keywords = r"(fn|let|struct|if|elsif|else|spawn|shared|with|atomic|match|lazy|inf|for|in|while|true|false)")]
#[logos(subpattern ident = r"[a-zA-Z_][a-zA-Z0-9_]*")]
#[logos(subpattern hex = r"0x[0-9a-fA-F]+")]
#[logos(subpattern bin = r"0b[01][_01]*")]
#[logos(subpattern oct = r"0o[0-7][_0-7]*")]
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("let")]
    Let,
    #[token("struct")]
    Struct,
    #[token("if")]
    If,
    #[token("elsif")]
    Elsif,
    #[token("else")]
    Else,
    #[token("spawn")]
    Spawn,
    #[token("shared")]
    Shared,
    #[token("with")]
    With,
    #[token("atomic")]
    Atomic,
    #[token("match")]
    Match,
    #[token("lazy")]
    Lazy,
    #[token("mut")]
    Mut,
    #[token("inf")]
    Inf,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Identifiers
    #[regex(r"(?&ident)", |lex| lex.slice().to_string(), priority = 1)]
    Ident(String),

    // Literals
    #[regex(r"-?(?&decimal)", |lex| lex.slice().parse::<i32>())]
    Int(i32),
    #[regex(r"(-?(?:0|[1-9]\d*))?(?:\.(?&decimal))(?:[eE][+-]?(?&decimal))?", |lex| lex.slice().parse::<f64>(), priority = 3)]
    Float(f64),

    #[regex(r"(?&bin)", |lex| i32::from_str_radix(&lex.slice()[2..].replace('_', ""), 2).map_err(|e| LexingError::InvalidBinary(e.to_string())))]
    Binary(i32),
    #[regex(r"(?&oct)", |lex| i32::from_str_radix(&lex.slice()[2..].replace('_', ""), 8).map_err(|e| LexingError::InvalidOctal(e.to_string())))]
    Octal(i32),
    #[regex(r"(?&hex)", |lex| i32::from_str_radix(&lex.slice()[2..].replace('_', ""), 16).map_err(|e| LexingError::InvalidHex(e.to_string())))]
    Hex(i32),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| {
        process_string_literal(&lex.slice()[1..lex.slice().len() - 1])
    })]
    String(Vec<StringPart>),

    // Symbols and operators
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("..")]
    Range,
    #[token("_")]
    Wildcard,

    // Assignment operators
    #[token("=")]
    Equals,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    StarEquals,
    #[token("/=")]
    SlashEquals,
    #[token("%=")]
    PercentEquals,
    #[token("&=")]
    AndEquals,
    #[token("|=")]
    OrEquals,
    #[token("^=")]
    XorEquals,
    #[token("<<=")]
    ShlEquals,
    #[token(">>=")]
    ShrEquals,

    // Arithmetic operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    // Logical operators
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,

    // Comparison operators
    #[token("==")]
    EqualsEquals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEquals,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEquals,

    // Bitwise operators
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("^")]
    BitXor,
    #[token("~")]
    BitNot,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,

    // Pipe operators
    #[token("|>")]
    Pipe,

    #[regex("(=>|return)")]
    Return,

    // Comments and whitespace
    #[regex(r"//[^\n]*")]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,
    #[regex(r"[\n;]")]
    LineEnd,
}
