use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub use grammar::ProgramParser;
pub mod string;
