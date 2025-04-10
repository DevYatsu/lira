use super::{
    Lexer,
    tokens::{LexingError, Token},
};

#[derive(Debug, PartialEq, Clone)]
pub enum StringPart<'input> {
    Text(&'input str),
    EscapeChar(char),
    Unicode(char),
    Expression(Vec<(usize, Token<'input>, usize)>),
}

pub fn process_string_literal<'i>(input: &'i str) -> Result<Vec<StringPart<'i>>, LexingError> {
    let mut parts = Vec::with_capacity(4);
    let mut chars = input.char_indices().peekable();
    let mut last_pos = 0;

    while let Some((i, c)) = chars.next() {
        match c {
            '\\' => {
                if last_pos < i {
                    parts.push(StringPart::Text(&input[last_pos..i]));
                }
                parts.push(process_escape_sequence(&mut chars)?);
                last_pos = chars.peek().map_or(input.len(), |&(j, _)| j);
            }

            '#' => {
                if matches!(chars.peek(), Some(&(_, '{'))) {
                    if last_pos < i {
                        parts.push(StringPart::Text(&input[last_pos..i]));
                    }
                    let (expr, end) = process_interpolation(&mut chars, input)?;
                    parts.push(expr);
                    last_pos = end;
                }
            }

            _ => {}
        }
    }

    if last_pos < input.len() {
        parts.push(StringPart::Text(&input[last_pos..]));
    }

    Ok(parts)
}

fn process_escape_sequence<I>(chars: &mut I) -> Result<StringPart<'static>, LexingError>
where
    I: Iterator<Item = (usize, char)>,
{
    match chars.next() {
        Some((_, esc)) => match esc {
            'n' => Ok(StringPart::EscapeChar('\n')),
            'r' => Ok(StringPart::EscapeChar('\r')),
            't' => Ok(StringPart::EscapeChar('\t')),
            '\\' => Ok(StringPart::EscapeChar('\\')),
            '0' => Ok(StringPart::EscapeChar('\0')),
            '"' => Ok(StringPart::EscapeChar('"')),
            '\'' => Ok(StringPart::EscapeChar('\'')),
            '#' => Ok(StringPart::EscapeChar('#')),
            'u' => process_unicode_escape(chars),
            _ => Err(LexingError::UnterminatedString(format!(
                "Unknown escape: \\{}",
                esc
            ))),
        },
        None => Err(LexingError::UnterminatedString(
            "Escape at end of string".into(),
        )),
    }
}

fn process_unicode_escape<I>(chars: &mut I) -> Result<StringPart<'static>, LexingError>
where
    I: Iterator<Item = (usize, char)>,
{
    match chars.next() {
        Some((_, '{')) => {
            let mut value = 0u32;
            let mut count = 0;

            while let Some((_, c)) = chars.next() {
                if c == '}' {
                    break;
                }
                if !c.is_ascii_hexdigit() || count == 6 {
                    return Err(LexingError::UnterminatedString(
                        "Invalid Unicode escape".into(),
                    ));
                }
                value = value * 16 + c.to_digit(16).unwrap();
                count += 1;
            }

            char::from_u32(value)
                .map(StringPart::Unicode)
                .ok_or_else(|| LexingError::UnterminatedString("Bad Unicode code point".into()))
        }
        Some((_, c)) => Err(LexingError::UnterminatedString(format!(
            "Invalid Unicode start: {}",
            c
        ))),
        None => Err(LexingError::UnterminatedString(
            "EOF in Unicode escape".into(),
        )),
    }
}

fn process_interpolation<'i, I>(
    chars: &mut I,
    input: &'i str,
) -> Result<(StringPart<'i>, usize), LexingError>
where
    I: Iterator<Item = (usize, char)>,
{
    let (open_pos, _) = chars.next().unwrap(); // consume '{'
    let expr_start = open_pos + 1;
    let mut depth = 1;

    let mut end_pos = expr_start;
    while let Some((j, c)) = chars.next() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end_pos = j;
                    break;
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(LexingError::UnterminatedString(
            "Unclosed interpolation".into(),
        ));
    }

    let tokens = Lexer::new(&input[expr_start..end_pos])
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok((StringPart::Expression(tokens), end_pos + 1))
}

fn parse_interpolated_expression(
    expr_str: &str,
) -> Result<Vec<(usize, Token, usize)>, LexingError> {
    Lexer::new(expr_str).into_iter().collect()
}
