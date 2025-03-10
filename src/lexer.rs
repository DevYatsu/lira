use logos::{Logos, Span};
use string::{StringPart, process_string_literal};

mod string;

type Error = (String, Span);
type Result<T> = std::result::Result<T, Error>;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    #[default]
    NonAsciiCharacter,
    UnterminatedString(String),
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexingError)]
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
#[logos(subpattern ident = r"[a-zA-Z_][a-zA-Z0-9_]*")]
#[logos(subpattern hex = r"0x[0-9a-fA-F]+")]
#[logos(subpattern bin = r"0b[01][_01]*")]
#[logos(subpattern oct = r"0o[0-7][_0-7]*")]
pub enum Token<'source> {
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
    #[token("move")]
    Move,
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
    #[token("inf")]
    Inf,

    // Identifiers
    #[regex(r"(?&ident)", priority = 1)]
    Ident,

    // Literals
    #[regex(r"-?(?&decimal)", |lex| lex.slice().parse::<i32>().unwrap())]
    Int(i32),
    #[regex(r"(-?(?:0|[1-9]\d*))?(?:\.(?&decimal))(?:[eE][+-]?(?&decimal))?", |lex| lex.slice().parse::<f64>().unwrap(), priority = 3)]
    Float(f64),
    #[regex(r"(?&bin)")]
    Binary,
    #[regex(r"(?&oct)")]
    Octal,
    #[regex(r"(?&hex)")]
    Hex,
    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| {
        process_string_literal(&lex.slice()[1..lex.slice().len() - 1])
    })]
    String(Vec<StringPart<'source>>),

    // Symbols and operators
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token("=>")]
    ArrowFat,
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
    #[token(".. ")]
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

    // Comments and whitespace
    #[regex(r"#[^\n]*")]
    LineComment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}
