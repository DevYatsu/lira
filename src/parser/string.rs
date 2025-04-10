use crate::{ast::Expr, lexer::str_litteral::StringPart};

use super::grammar::ExprParser;

#[derive(Debug, PartialEq, Clone)]
pub enum ParserStringPart<'a> {
    Text(String),
    Expression(Expr<'a>),
}

pub fn process_parser_string<'a>(parts: Vec<StringPart<'a>>) -> Vec<ParserStringPart<'a>> {
    let mut result = Vec::new();
    let mut current_string = String::new();
    let parser = ExprParser::new();

    for part in parts {
        match part {
            StringPart::Text(text) => {
                current_string.push_str(text);
            }
            StringPart::EscapeChar(c) => {
                current_string.push(c);
            }
            StringPart::Unicode(c) => {
                current_string.push(c);
            }

            StringPart::Expression(tokens) => {
                if !current_string.is_empty() {
                    result.push(ParserStringPart::Text(std::mem::take(&mut current_string)));
                }

                let parsed = parser.parse(tokens.into_iter());
                match parsed {
                    Ok(expr) => result.push(ParserStringPart::Expression(expr)),
                    Err(err) => {
                        panic!("Failed to parse expression in interpolated string: {err:?}")
                    }
                }
            }
        }
    }

    if !current_string.is_empty() {
        result.push(ParserStringPart::Text(current_string));
    }

    result
}
