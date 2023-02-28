use crate::compiler::token_stream::TokenStream;

pub struct Expr {}

pub struct ExprParser<'a> {
    stream: &'a mut TokenStream,
}

impl<'a> ExprParser<'a> {
    pub fn new(stream: &'a mut TokenStream) -> Self {
        ExprParser { stream: stream }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> Option<Expr> {
        None
    }

    fn parse_or(&mut self) -> Option<Expr> {
        None
    }

    fn parse_and(&mut self) -> Option<Expr> {
        None
    }

    fn parse_cmp(&mut self) -> Option<Expr> {
        None
    }

    fn parse_temp(&mut self) -> Option<Expr> {
        None
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        None
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        None
    }

    fn parse_base(&mut self) -> Option<Expr> {
        None
    }

    fn parse_operand(&mut self) -> Option<Expr> {
        None
    }
}
