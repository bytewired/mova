use crate::compiler::token::{Token, TokenKind};

pub fn compare_token_kind(token: Option<&Token>, token_kind: TokenKind) -> bool {
    match token {
        Some(t) => t.kind == token_kind,
        None => false
    }
}