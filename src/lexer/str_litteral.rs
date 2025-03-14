use super::{
    Lexer,
    tokens::{LexingError, Token},
};

// Modified StringPart to use borrowed string slices
#[derive(Debug, PartialEq, Clone)]
pub enum StringPart {
    Text(String),
    EscapeChar(char),
    Unicode(char),
    Expression(Vec<(usize, Token, usize)>),
}

// Process string literal into categorized parts with interpolation support
pub fn process_string_literal(input: &'_ str) -> Result<Vec<StringPart>, LexingError> {
    let mut parts = Vec::new();
    let mut text_start = 0; // Track the start of the current text segment
    let mut chars = input.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        match c {
            '\\' => {
                // Save any text before this escape sequence
                if i > text_start {
                    parts.push(StringPart::Text(input[text_start..i].to_owned()));
                }

                // Get the next character for the escape sequence
                match chars.next() {
                    Some((_, 'n')) => parts.push(StringPart::EscapeChar('\n')),
                    Some((_, 'r')) => parts.push(StringPart::EscapeChar('\r')),
                    Some((_, 't')) => parts.push(StringPart::EscapeChar('\t')),
                    Some((_, '\\')) => parts.push(StringPart::EscapeChar('\\')),
                    Some((_, '0')) => parts.push(StringPart::EscapeChar('\0')),
                    Some((_, '"')) => parts.push(StringPart::EscapeChar('"')),
                    Some((_, '\'')) => parts.push(StringPart::EscapeChar('\'')),
                    Some((_, '#')) => parts.push(StringPart::EscapeChar('#')),
                    Some((escape_pos, 'u')) => {
                        // Handle Unicode escape sequences \uXXXX
                        let start_pos = escape_pos + 1;
                        let mut hex_chars = String::with_capacity(4);

                        for _ in 0..4 {
                            match chars.next() {
                                Some((_, hex_char)) => hex_chars.push(hex_char),
                                None => {
                                    return Err(LexingError::UnterminatedString(
                                        "Incomplete Unicode escape sequence".to_string(),
                                    ));
                                }
                            }
                        }

                        let code_point = u32::from_str_radix(&hex_chars, 16).map_err(|_| {
                            LexingError::UnterminatedString(
                                "Invalid Unicode escape sequence".to_string(),
                            )
                        })?;

                        match char::from_u32(code_point) {
                            Some(unicode_char) => parts.push(StringPart::Unicode(unicode_char)),
                            None => {
                                return Err(LexingError::UnterminatedString(
                                    "Invalid Unicode code point".to_string(),
                                ));
                            }
                        }
                    }
                    Some((_, c)) => {
                        return Err(LexingError::UnterminatedString(format!(
                            "Unknown escape sequence: \\{}",
                            c
                        )));
                    }
                    None => {
                        return Err(LexingError::UnterminatedString(
                            "String terminated with escape character".to_string(),
                        ));
                    }
                }

                // Update text_start to the position after this escape sequence
                if let Some((next_pos, _)) = chars.peek() {
                    text_start = *next_pos;
                } else {
                    text_start = input.len();
                }
            }
            '#' => {
                // Check if we have a #{...} expression
                if chars.peek().map(|(_, c)| *c) == Some('{') {
                    // Save any text before this interpolation
                    if i > text_start {
                        parts.push(StringPart::Text(input[text_start..i].to_owned()));
                    }

                    // Consume the '{'
                    let (open_brace_pos, _) = chars.next().unwrap();
                    let expr_start = open_brace_pos + 1;

                    // Read until matching '}'
                    let mut brace_count = 1; // Track nested braces
                    let mut expr_end = input.len(); // Default to end if no closing brace

                    while let Some((j, expr_char)) = chars.next() {
                        match expr_char {
                            '{' => {
                                brace_count += 1;
                            }
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    // End of expression
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

                    let expression = StringPart::Expression(parse_interpolated_expression(
                        &input[expr_start..expr_end],
                    )?);

                    // Add the expression slice
                    parts.push(expression);

                    // Update text_start to after this interpolation
                    text_start = expr_end + 1;
                } else {
                    // This is just a regular # character, continue
                    continue;
                }
            }
            _ => {
                // Continue accumulating regular text characters
                continue;
            }
        }
    }

    // Add any remaining text
    if text_start < input.len() {
        parts.push(StringPart::Text(input[text_start..].to_owned()));
    }

    Ok(parts)
}

fn parse_interpolated_expression(
    expr_str: &str,
) -> Result<Vec<(usize, Token, usize)>, LexingError> {
    Lexer::new(expr_str).into_iter().collect()
}
