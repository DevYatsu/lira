use super::{
    Lexer,
    tokens::{LexingError, Token},
};

#[derive(Debug, PartialEq, Clone)]
pub enum StringPart {
    Text(String),
    EscapeChar(char),
    Unicode(char),
    Expression(Vec<(usize, Token, usize)>),
}

pub fn process_string_literal(input: &str) -> Result<Vec<StringPart>, LexingError> {
    let mut parts = Vec::new();
    let mut text_start = 0;
    let mut chars = input.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        match c {
            '\\' => {
                // Save text accumulated so far
                if i > text_start {
                    parts.push(StringPart::Text(input[text_start..i].to_owned()));
                }

                // Process escape sequence
                let escape_part = process_escape_sequence(&mut chars)?;
                parts.push(escape_part);

                // Update text_start to after this escape sequence
                text_start = chars.peek().map_or(input.len(), |&(pos, _)| pos);
            }
            '#' => {
                // Process possible interpolation expression
                if chars.peek().map_or(false, |&(_, next_c)| next_c == '{') {
                    // Save text accumulated so far
                    if i > text_start {
                        parts.push(StringPart::Text(input[text_start..i].to_owned()));
                    }

                    // Process interpolated expression
                    let (expression, next_pos) = process_interpolation(&mut chars, input)?;
                    parts.push(expression);
                    text_start = next_pos;
                }
                // Otherwise, treat # as normal text character
            }
            _ => {} // Continue accumulating regular text characters
        }
    }

    // Add any remaining text
    if text_start < input.len() {
        parts.push(StringPart::Text(input[text_start..].to_owned()));
    }

    Ok(parts)
}

fn process_escape_sequence<I>(chars: &mut I) -> Result<StringPart, LexingError>
where
    I: Iterator<Item = (usize, char)>,
{
    match chars.next() {
        Some((_, 'n')) => Ok(StringPart::EscapeChar('\n')),
        Some((_, 'r')) => Ok(StringPart::EscapeChar('\r')),
        Some((_, 't')) => Ok(StringPart::EscapeChar('\t')),
        Some((_, '\\')) => Ok(StringPart::EscapeChar('\\')),
        Some((_, '0')) => Ok(StringPart::EscapeChar('\0')),
        Some((_, '"')) => Ok(StringPart::EscapeChar('"')),
        Some((_, '\'')) => Ok(StringPart::EscapeChar('\'')),
        Some((_, '#')) => Ok(StringPart::EscapeChar('#')),
        Some((_, 'u')) => process_unicode_escape(chars),
        Some((_, c)) => Err(LexingError::UnterminatedString(format!(
            "Unknown escape sequence: \\{}",
            c
        ))),
        None => Err(LexingError::UnterminatedString(
            "String terminated with escape character".to_string(),
        )),
    }
}

fn process_unicode_escape<I>(chars: &mut I) -> Result<StringPart, LexingError>
where
    I: Iterator<Item = (usize, char)>,
{
    // Check if we have the new format \u{XXXX}
    match chars.next() {
        Some((_, '{')) => {
            // New format: \u{XXXX} with variable length hex
            let mut hex_chars = String::new();

            // Read hex digits until closing brace
            loop {
                match chars.next() {
                    Some((_, '}')) => break, // End of Unicode escape
                    Some((_, c)) if c.is_ascii_hexdigit() => hex_chars.push(c),
                    Some((_, c)) => {
                        return Err(LexingError::UnterminatedString(format!(
                            "Invalid character in Unicode escape: {}",
                            c
                        )));
                    }
                    None => {
                        return Err(LexingError::UnterminatedString(
                            "Incomplete Unicode escape sequence".to_string(),
                        ));
                    }
                }
            }

            // Validate hex length
            if hex_chars.is_empty() || hex_chars.len() > 6 {
                return Err(LexingError::UnterminatedString(
                    "Unicode escape must have 1-6 hex digits".to_string(),
                ));
            }

            // Parse hex digits into Unicode code point
            let code_point = u32::from_str_radix(&hex_chars, 16).map_err(|_| {
                LexingError::UnterminatedString("Invalid Unicode escape sequence".to_string())
            })?;

            // Convert code point to character
            char::from_u32(code_point)
                .map(StringPart::Unicode)
                .ok_or_else(|| {
                    LexingError::UnterminatedString("Invalid Unicode code point".to_string())
                })
        }
        // // Legacy format: \uXXXX with exactly 4 hex digits
        // Some((_, first_hex)) if first_hex.is_ascii_hexdigit() => {
        //     let mut hex_chars = String::with_capacity(4);
        //     hex_chars.push(first_hex);

        //     // Collect exactly 3 more hex digits
        //     for _ in 0..3 {
        //         match chars.next() {
        //             Some((_, hex_char)) if hex_char.is_ascii_hexdigit() => hex_chars.push(hex_char),
        //             Some((_, c)) => return Err(LexingError::UnterminatedString(
        //                 format!("Invalid hexadecimal digit in Unicode escape: {}", c)
        //             )),
        //             None => return Err(LexingError::UnterminatedString(
        //                 "Incomplete Unicode escape sequence".to_string()
        //             )),
        //         }
        //     }

        //     // Parse hex digits into Unicode code point
        //     let code_point = u32::from_str_radix(&hex_chars, 16).map_err(|_| {
        //         LexingError::UnterminatedString("Invalid Unicode escape sequence".to_string())
        //     })?;

        //     // Convert code point to character
        //     char::from_u32(code_point)
        //         .map(StringPart::Unicode)
        //         .ok_or_else(|| {
        //             LexingError::UnterminatedString("Invalid Unicode code point".to_string())
        //         })
        // },
        Some((_, c)) => Err(LexingError::UnterminatedString(format!(
            "Invalid start of Unicode escape sequence: {}",
            c
        ))),
        None => Err(LexingError::UnterminatedString(
            "Incomplete Unicode escape sequence".to_string(),
        )),
    }
}

fn process_interpolation<I>(chars: &mut I, input: &str) -> Result<(StringPart, usize), LexingError>
where
    I: Iterator<Item = (usize, char)>,
{
    // Consume the '{'
    let (open_brace_pos, _) = chars.next().unwrap();
    let expr_start = open_brace_pos + 1;

    // Find matching closing brace, accounting for nesting
    let mut brace_count = 1;
    let mut expr_end = input.len(); // Default to end if no closing brace

    while let Some((j, expr_char)) = chars.next() {
        match expr_char {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;
                if brace_count == 0 {
                    expr_end = j;
                    break;
                }
            }
            _ => {}
        }
    }

    if brace_count > 0 {
        return Err(LexingError::UnterminatedString(
            "Unterminated expression in string interpolation".to_string(),
        ));
    }

    // Parse the expression within the braces
    let expression =
        StringPart::Expression(parse_interpolated_expression(&input[expr_start..expr_end])?);

    Ok((expression, expr_end + 1))
}

fn parse_interpolated_expression(
    expr_str: &str,
) -> Result<Vec<(usize, Token, usize)>, LexingError> {
    Lexer::new(expr_str).into_iter().collect()
}
