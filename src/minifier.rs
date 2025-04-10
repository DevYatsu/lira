use crate::{Lexer, Token};

pub fn minify(mut lexer: Lexer<'_>) -> String {
    let source = lexer.source();
    let mut result = String::with_capacity(source.len());

    let mut last_token: Option<Token> = None;
    let mut space_pending = false;

    while let Some(Ok((start, token, end))) = lexer.next() {
        let slice = &source[start..end];

        match &token {
            Token::LineComment(text) => {
                flush_space(&mut result, space_pending);
                result.push_str("//");
                result.push_str(text.trim_end());
                result.push('\n');
            }
            Token::BlockComment(text) => {
                flush_space(&mut result, space_pending);
                result.push_str("/*");
                result.push_str(text.trim());
                result.push_str("*/\n");
            }
            Token::DocComment(text) => {
                flush_space(&mut result, space_pending);
                result.push_str("///");
                result.push_str(text.trim_end());
                result.push('\n');
            }
            Token::LineEnd => {
                if let Some(t) = &last_token {
                    if ends_expr(t) {
                        result.push(';');
                    }
                }
            }
            Token::Whitespace => {}
            Token::LParen => result.push('('),
            Token::RParen => result.push(')'),
            Token::LBrace => result.push('{'),
            Token::RBrace => result.push('}'),
            Token::LBracket => result.push('['),
            Token::RBracket => result.push(']'),
            Token::Dot => result.push('.'),
            Token::Comma => result.push(','),
            Token::Colon => result.push(':'),
            Token::Wildcard => result.push('_'),
            Token::Arrow => result.push_str("->"),
            Token::Range => result.push_str(".."),

            _ => {
                if space_pending || needs_space_between(&last_token, &token) {
                    flush_space(&mut result, true);
                }
                result.push_str(slice);
            }
        }

        if !matches!(
            token,
            Token::Whitespace
                | Token::LineEnd
                | Token::LineComment(_)
                | Token::BlockComment(_)
                | Token::DocComment(_)
        ) {
            last_token = Some(token);
        }

        space_pending = false;
    }

    result.shrink_to_fit();
    result
}

#[inline(always)]
fn flush_space(result: &mut String, insert: bool) {
    if insert
        && !matches!(
            result.as_bytes().last(),
            Some(b' ' | b'\n' | b';' | b'{' | b'(' | b'[')
        )
    {
        result.push(' ');
    }
}

#[inline(always)]
fn ends_expr(token: &Token) -> bool {
    matches!(
        token,
        Token::Ident(_)
            | Token::Int(_)
            | Token::Octal(_)
            | Token::Binary(_)
            | Token::Hex(_)
            | Token::String(_)
            | Token::Float(_)
            | Token::RParen
            | Token::RBracket
            | Token::RBrace
    )
}

fn needs_space_between(prev: &Option<Token>, next: &Token) -> bool {
    use Token::*;
    match (prev, next) {
        (
            Some(
                Fn | Let | Use | Struct | Enum | Async | Await | While | If | Elsif | For | Match
                | TypeAlias | Return | As | In | With,
            ),
            Ident(_),
        )
        | (Some(Async), Fn)
        | (Some(If | While | Return), LParen)
        | (Some(Spawn), With)
        | (Some(Let | While | For | Match), Atomic | Lazy | Mut)
        | (Some(Ident(_)), Ident(_) | In)
        | (Some(Int(_) | Octal(_) | Hex(_) | Binary(_) | Float(_) | True | False), Ident(_))
        | (
            Some(In | While | If | Elsif | Await | Async | Return),
            Int(_) | Octal(_) | Hex(_) | Binary(_) | Float(_) | True | False,
        )
        | (Some(Ident(_)), As | If)
        | (Some(Atomic | Lazy | Mut), Ident(_) | Atomic | Mut | Lazy) => true,

        _ => false,
    }
}
