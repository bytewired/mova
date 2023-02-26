use crate::compiler::token::{Token, TokenKind};

pub struct TokenStream {
    current: usize,
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream {
            current: 0,
            tokens: tokens,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current == self.tokens.len() || matches!(self.current().kind, TokenKind::Eof)
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current)
    }

    pub fn next_expected(&mut self, kind: TokenKind, msg: &str) -> &Token {
        self.next().filter(|token| token.kind == kind).expect(msg)
    }

    pub fn check(&self, token: &Token, kind: TokenKind) -> bool {
        token.kind == kind
    }

    pub fn advance_if_match(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            self.next();
            true
        } else {
            false
        }
    }

    fn advance_if_match_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.advance_if_match(*kind) {
                return true;
            }
        }

        return false;
    }

    fn advance_if_cmp(&mut self) -> bool {
        self.advance_if_match_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::BangEqual,
            TokenKind::EqualEqual,
        ])
    }

    fn advance_if_unary(&mut self) -> bool {
        self.advance_if_match_any(&[
            TokenKind::Minus,
            TokenKind::Bang,
            TokenKind::Inc,
            TokenKind::Dec,
            TokenKind::At,
        ])
    }

    fn is_assign(&mut self) -> bool {
        let assign_kinds = [
            TokenKind::Equal,
            TokenKind::PlusEqual,
            TokenKind::MinusEqual,
            TokenKind::SlashEqual,
            TokenKind::StarEqual,
        ];

        for kind in assign_kinds {
            if self.check(self.current(), kind) {
                return true;
            }
        }

        return false;
    }
}
