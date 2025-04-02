use lira::{Lexer, LexingError, Token, lexer::str_litteral::StringPart};

fn lex_tokens(input: &str) -> Result<Vec<(Token, &str)>, LexingError> {
    let lexer = Lexer::new(input);

    lexer
        .into_iter()
        .map(|result| match result {
            Ok((start, tok, end)) => Ok((tok, &input[start..end])),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<_>, _>>()
}

#[test]
fn test_identifier_and_keyword() -> Result<(), LexingError> {
    let tokens = lex_tokens("let x")?;
    assert_eq!(
        tokens,
        vec![(Token::Let, "let"), (Token::Ident("x".to_string()), "x")]
    );

    Ok(())
}

#[test]
fn test_integer_literals() -> Result<(), LexingError> {
    let tokens = lex_tokens("42 -10 0b1010 0xFF 0o77")?;
    assert_eq!(
        tokens,
        vec![
            (Token::Int(42), "42"),
            (Token::Int(-10), "-10"),
            (Token::Binary(10), "0b1010"),
            (Token::Hex(255), "0xFF"),
            (Token::Octal(63), "0o77"),
        ]
    );

    Ok(())
}

#[test]
fn test_string_literal() -> Result<(), LexingError> {
    let tokens = lex_tokens(r#""hello""#)?;
    assert_eq!(tokens.len(), 1);
    if let Token::String(parts) = &tokens[0].0 {
        assert_eq!(parts.len(), 1);
        if let StringPart::Text(s) = &parts[0] {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected literal string part");
        }
    } else {
        panic!("Expected Token::String");
    }
    Ok(())
}

#[test]
fn test_operators() -> Result<(), LexingError> {
    let tokens = lex_tokens("+ - * / = == && || |> -> =>")?;
    assert_eq!(
        tokens.iter().map(|t| &t.0).collect::<Vec<_>>(),
        vec![
            &Token::Plus,
            &Token::Minus,
            &Token::Star,
            &Token::Slash,
            &Token::Equals,
            &Token::EqualsEquals,
            &Token::And,
            &Token::Or,
            &Token::Pipe,
            &Token::Arrow,
            &Token::MatchReturn,
        ]
    );

    Ok(())
}

#[test]
fn test_let_statement_with_expr() -> Result<(), LexingError> {
    let tokens = lex_tokens("let x = 1 + 2;")?;
    assert_eq!(
        tokens,
        vec![
            (Token::Let, "let"),
            (Token::Ident("x".into()), "x"),
            (Token::Equals, "="),
            (Token::Int(1), "1"),
            (Token::Plus, "+"),
            (Token::Int(2), "2"),
            (Token::LineEnd, ";"),
        ]
    );

    Ok(())
}

#[test]
fn test_fn_declaration() -> Result<(), LexingError> {
    let tokens = lex_tokens("fn add(a: int, b: int) -> int { return a + b }")?;
    assert_eq!(
        tokens.iter().map(|t| &t.0).collect::<Vec<_>>(),
        vec![
            &Token::Fn,
            &Token::Ident("add".into()),
            &Token::LParen,
            &Token::Ident("a".into()),
            &Token::Colon,
            &Token::Ident("int".into()),
            &Token::Comma,
            &Token::Ident("b".into()),
            &Token::Colon,
            &Token::Ident("int".into()),
            &Token::RParen,
            &Token::Arrow,
            &Token::Ident("int".into()),
            &Token::LBrace,
            &Token::Return,
            &Token::Ident("a".into()),
            &Token::Plus,
            &Token::Ident("b".into()),
            &Token::RBrace,
        ]
    );

    Ok(())
}

#[test]
fn test_struct_declaration() -> Result<(), LexingError> {
    let tokens = lex_tokens("struct Point { x: int, y: int }")?;
    assert_eq!(
        tokens.iter().map(|t| &t.0).collect::<Vec<_>>(),
        vec![
            &Token::Struct,
            &Token::Ident("Point".into()),
            &Token::LBrace,
            &Token::Ident("x".into()),
            &Token::Colon,
            &Token::Ident("int".into()),
            &Token::Comma,
            &Token::Ident("y".into()),
            &Token::Colon,
            &Token::Ident("int".into()),
            &Token::RBrace,
        ]
    );

    Ok(())
}

#[test]
fn test_if_else_block() -> Result<(), LexingError> {
    let tokens = lex_tokens("if x > 0 { return 1 } elsif x < 0 { return -1 } else { return 0 }")?;
    assert_eq!(
        tokens.iter().map(|t| &t.0).collect::<Vec<_>>(),
        vec![
            &Token::If,
            &Token::Ident("x".into()),
            &Token::Greater,
            &Token::Int(0),
            &Token::LBrace,
            &Token::Return,
            &Token::Int(1),
            &Token::RBrace,
            &Token::Elsif,
            &Token::Ident("x".into()),
            &Token::Less,
            &Token::Int(0),
            &Token::LBrace,
            &Token::Return,
            &Token::Int(-1),
            &Token::RBrace,
            &Token::Else,
            &Token::LBrace,
            &Token::Return,
            &Token::Int(0),
            &Token::RBrace,
        ]
    );

    Ok(())
}
