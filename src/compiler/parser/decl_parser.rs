use crate::compiler::token::{TokenKind, Value};
use crate::compiler::token_stream::TokenStream;
use crate::compiler::utils::compare_token_kind;
use crate::utils::exit_with_err_msg;

use super::expr_parser::{Expr, ExprParser};
use super::stmt_parser::{Stmt, StmtParser};

pub struct VarDecl {
    name: String,
    expr: Expr,
}

pub struct LetDecl {
    name: String,
    expr: Expr,
}

pub struct FnDecl {
    name: String,
    params: Vec<FnParam>,
    stmts: Vec<Stmt>,
}

pub struct FnParam {
    // TODO add type, currently by default it's int
    // type_info: TypeInfo
    name: String,
    is_mutable: bool,
}

pub enum Decl {
    Var(VarDecl),
    Let(LetDecl),
    Fn(FnDecl),
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
            } else {
                self.stream.next();
            }
        }

        decls
    }

    fn parse_var(&mut self) -> Option<Decl> {
        let name = self.stream.next_expected(TokenKind::Identifier, "expected variable name").clone();

        self.stream.next_expected(TokenKind::Colon, "expected ':' after variable declaration");

        if self.stream.advance_if_match(TokenKind::Equal) {
            let expr = ExprParser::new(self.stream).parse().expect("failed parsing expression");

            self.expect_semicolor();

            return Some(Decl::Var(VarDecl {
                name: name.value.get_str(),
                expr: expr,
            }));
        }

        self.expect_semicolor();

        None
    }

    fn parse_let(&mut self) -> Option<Decl> {
        let name = self.stream.next_expected(TokenKind::Identifier, "expected variable name").clone();

        self.stream.next_expected(TokenKind::Colon, "expected ':' after variable declaration");

        if self.stream.advance_if_match(TokenKind::Equal) {
            let expr = ExprParser::new(self.stream).parse().expect("failed parsing expression");

            self.expect_semicolor();

            return Some(Decl::Let(LetDecl {
                name: name.value.get_str(),
                expr: expr,
            }));
        }

        self.expect_semicolor();

        None
    }

    fn parse_fn(&mut self) -> Option<Decl> {
        let name = self.stream.next_expected(TokenKind::Identifier, "expected function name").clone();

        self.stream.next_expected(TokenKind::LeftParen, "expected '(' after function name");

        let mut fn_params = Vec::<FnParam>::new();

        while !compare_token_kind(self.stream.peek(), TokenKind::RightParen) {
            let next = self.stream.next().unwrap();

            match next.kind {
                TokenKind::Let => fn_params.push(FnParam {
                    name: self.stream.next_expected(TokenKind::Identifier, "expected identifier after 'let' keyword").value.get_str(),
                    is_mutable: false,
                }),
                TokenKind::Var => fn_params.push(FnParam {
                    name: self.stream.next_expected(TokenKind::Identifier, "expected identifier after 'var' keyword").value.get_str(),
                    is_mutable: true,
                }),
                _ => continue,
            }
        }

        self.stream.next_expected(TokenKind::RightParen, "expected ')' after parameters");
        // TODO add checking function return type

        let stmts = StmtParser::new(self.stream).parse();

        Some(Decl::Fn(FnDecl {
            name: name.value.get_str(),
            params: fn_params,
            stmts: stmts,
        }))
    }

    fn expect_semicolor(&mut self) {
        self.stream.next_expected(TokenKind::Semicolon, "expected ';' at the end of declaration");
    }
}
