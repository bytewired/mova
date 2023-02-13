use crate::compiler::token::{Token, TokenKind};

pub struct TokenStream {
    current: usize,
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { current: 0, tokens: tokens}
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len() || matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
