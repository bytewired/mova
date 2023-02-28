use crate::compiler::ast::stmt::Stmt;
use crate::compiler::token::{TokenKind, Value};
use crate::compiler::token_stream::TokenStream;
use std::vec;

pub struct StmtParser<'a> {
    stream: &'a mut TokenStream,
}

impl<'a> StmtParser<'a> {
    pub fn new(stream: &'a mut TokenStream) -> Self {
        StmtParser { stream: stream }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        self.stream.next_expected(TokenKind::LeftBrace, "expceted '{' after function parameters");
        self.stream.next_expected(TokenKind::RightBrace, "expceted '}' after function body");
        vec![]
    }
}
