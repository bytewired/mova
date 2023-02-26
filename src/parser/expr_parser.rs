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
        None
    }
}
