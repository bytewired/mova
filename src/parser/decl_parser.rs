use crate::compiler::token::{TokenKind, Value};
use crate::compiler::token_stream::TokenStream;
use crate::utils::exit_with_err_msg;

use super::expr_parser::{Expr, ExprParser};

pub enum Decl {
    VarDecl(String, Expr),
}

pub struct DeclParser<'a> {
    stream: &'a mut TokenStream,
}

impl<'a> DeclParser<'a> {
    pub fn new(stream: &'a mut TokenStream) -> Self {
        DeclParser { stream: stream }
    }

    pub fn parse(&mut self) -> Vec<Decl> {
        let mut decls = Vec::<Decl>::new();

        while !self.stream.is_at_end() {
            let current = self.stream.current();

            if self.stream.check(current, TokenKind::Var) {
                decls.push(self.parse_var().expect("expected var declaration"))
            } else if self.stream.check(current, TokenKind::Let) {
                decls.push(self.parse_let().expect("expected let declaration"))
            } else if self.stream.check(current, TokenKind::Fn) {
                decls.push(self.parse_fn().expect("expected fn declaration"))
            }
        }

        decls
    }

    fn parse_var(&mut self) -> Option<Decl> {
        let name = self
            .stream
            .next_expected(TokenKind::Identifier, "expected variable name")
            .clone();

        self.stream
            .next_expected(TokenKind::Colon, "expected ':' after variable declaration");

        if self.stream.advance_if_match(TokenKind::Equal) {
            let expr = ExprParser::new(self.stream)
                .parse()
                .expect("failed parsing expression");

            self.expect_semicolor();

            let var_name = match &name.value {
                Value::Str(s) => s.clone(),
                _ => exit_with_err_msg("expected Value::Str"),
            };

            return Some(Decl::VarDecl(var_name, expr));
        }

        self.expect_semicolor();

        None
    }

    fn parse_let(&self) -> Option<Decl> {
        None
    }

    fn parse_fn(&self) -> Option<Decl> {
        None
    }

    fn expect_semicolor(&mut self) {
        self.stream
            .next_expected(TokenKind::Semicolon, "expected ';' at the end of declaration");
    }
}
