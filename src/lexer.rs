pub mod str_litteral;
pub mod tokens;

use logos::{Logos, SpannedIter};
use tokens::{LexingError, Token};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone)]
pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }

    pub fn slice(&self) -> &'input str {
        self.token_stream.slice()
    }
    pub fn span(&self) -> (usize, usize) {
        let span = self.token_stream.span();
        (span.start, span.end)
    }
    pub fn start(&self) -> usize {
        self.token_stream.span().start
    }
    pub fn end(&self) -> usize {
        self.token_stream.span().end
    }

    pub fn source(&self) -> &'input str {
        self.token_stream.source()
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}
