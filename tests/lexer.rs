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
    assert_eq!(tokens, vec![(Token::Let, "let"), (Token::Ident("x"), "x")]);

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
            assert_eq!(s, &"hello");
        } else {
            panic!("Expected literal string part");
        }
    } else {
        panic!("Expected Token::String");
    }
    Ok(())
}
#[test]
fn test_string_literal_plain() -> Result<(), LexingError> {
    let tokens = lex_tokens(r#""hello world""#)?;
    assert_eq!(tokens.len(), 1);

    if let Token::String(parts) = &tokens[0].0 {
        assert_eq!(parts, &[StringPart::Text("hello world".into())]);
    } else {
        panic!("Expected Token::String");
    }

    Ok(())
}
#[test]
fn test_string_literal_escaped_chars() -> Result<(), LexingError> {
    let tokens = lex_tokens(r#""hello\nworld\t!""#)?;
    assert_eq!(tokens.len(), 1);

    let expected_parts = vec![
        StringPart::Text("hello".into()),
        StringPart::EscapeChar('\n'),
        StringPart::Text("world".into()),
        StringPart::EscapeChar('\t'),
        StringPart::Text("!".into()),
    ];

    if let Token::String(parts) = &tokens[0].0 {
        assert_eq!(parts, &expected_parts);
    } else {
        panic!("Expected Token::String");
    }

    Ok(())
}
#[test]
fn test_string_literal_unicode_escape() -> Result<(), LexingError> {
    let tokens = lex_tokens(r#""emoji: \u{1F600}""#)?;
    assert_eq!(tokens.len(), 1);

    if let Token::String(parts) = &tokens[0].0 {
        assert_eq!(
            parts,
            &[
                StringPart::Text("emoji: ".into()),
                StringPart::Unicode('\u{1F600}')
            ]
        );
    } else {
        panic!("Expected Token::String");
    }

    Ok(())
}
#[test]
fn test_string_with_interpolation() -> Result<(), LexingError> {
    let tokens = lex_tokens(r#""hello #{name}""#)?;
    assert_eq!(tokens.len(), 1);

    if let Token::String(parts) = &tokens[0].0 {
        assert_eq!(
            parts,
            &[
                StringPart::Text("hello ".into()),
                StringPart::Expression(vec![(0usize, Token::Ident("name".into()), 4usize)]),
            ]
        );
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

#[test]
fn test_tokens_across_lines() -> Result<(), LexingError> {
    let input = r#"
        let x = 10
        let y = x + 5;
    "#;

    let tokens = lex_tokens(input)?;
    let expected_tokens = vec![
        Token::LineEnd,
        Token::Let,
        Token::Ident("x".into()),
        Token::Equals,
        Token::Int(10),
        Token::LineEnd,
        Token::Let,
        Token::Ident("y".into()),
        Token::Equals,
        Token::Ident("x".into()),
        Token::Plus,
        Token::Int(5),
        Token::LineEnd,
    ];

    assert_eq!(
        tokens.iter().map(|t| t.0.clone()).collect::<Vec<_>>(),
        expected_tokens
    );

    Ok(())
}

#[test]
fn test_comments() -> Result<(), LexingError> {
    let tokens = lex_tokens(
        r#"
        // this is a line comment
        let x = 5; /* block comment */
        "#,
    )?;

    let kinds: Vec<_> = tokens.iter().map(|(t, _)| t.to_owned()).collect();

    let expected_tokens = vec![
        Token::LineEnd,
        Token::LineComment(" this is a line comment".into()),
        Token::LineEnd,
        Token::Let,
        Token::Ident("x".into()),
        Token::Equals,
        Token::Int(5),
        Token::LineEnd,
        Token::BlockComment(" block comment ".into()),
        Token::LineEnd,
    ];

    assert_eq!(kinds, expected_tokens);

    Ok(())
}

#[test]
fn test_token_coverage_full() -> Result<(), LexingError> {
    let input = r#"
        fn f(a: int) -> int {
            let x = 0b1010 + 0xFF - 0o77 * 42 / 2;
            if x == 10 && x != 5 || x >= 3 && x <= 6 {
                x <<= 1; x >>= 1;
                x += 1; x -= 1; x *= 2; x /= 2; x %= 2;
                x &= 1; x |= 2; x ^= 3;
                :> |v| v + 1
            }
        }
    "#;

    let tokens = lex_tokens(input)?;

    let expected_tokens = vec![
        Token::LineEnd,
        Token::Fn,
        Token::Ident("f".into()),
        Token::LParen,
        Token::Ident("a".into()),
        Token::Colon,
        Token::Ident("int".into()),
        Token::RParen,
        Token::Arrow,
        Token::Ident("int".into()),
        Token::LBrace,
        Token::LineEnd,
        Token::Let,
        Token::Ident("x".into()),
        Token::Equals,
        Token::Binary(10),
        Token::Plus,
        Token::Hex(255),
        Token::Minus,
        Token::Octal(63),
        Token::Star,
        Token::Int(42),
        Token::Slash,
        Token::Int(2),
        Token::LineEnd,
        Token::If,
        Token::Ident("x".into()),
        Token::EqualsEquals,
        Token::Int(10),
        Token::And,
        Token::Ident("x".into()),
        Token::NotEquals,
        Token::Int(5),
        Token::Or,
        Token::Ident("x".into()),
        Token::GreaterEquals,
        Token::Int(3),
        Token::And,
        Token::Ident("x".into()),
        Token::LessEquals,
        Token::Int(6),
        Token::LBrace,
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::ShlEquals,
        Token::Int(1),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::ShrEquals,
        Token::Int(1),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::PlusEquals,
        Token::Int(1),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::MinusEquals,
        Token::Int(1),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::StarEquals,
        Token::Int(2),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::SlashEquals,
        Token::Int(2),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::PercentEquals,
        Token::Int(2),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::AndEquals,
        Token::Int(1),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::OrEquals,
        Token::Int(2),
        Token::LineEnd,
        Token::Ident("x".into()),
        Token::XorEquals,
        Token::Int(3),
        Token::LineEnd,
        Token::Return,
        Token::BitOr,
        Token::Ident("v".into()),
        Token::BitOr,
        Token::Ident("v".into()),
        Token::Plus,
        Token::Int(1),
        Token::LineEnd,
        Token::RBrace,
        Token::LineEnd,
        Token::RBrace,
        Token::LineEnd,
    ];

    let actual_tokens: Vec<_> = tokens.iter().map(|(t, _)| t.clone()).collect();

    assert_eq!(actual_tokens, expected_tokens);

    Ok(())
}
