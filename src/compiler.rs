mod lexer;
mod token;
mod token_stream;

use lexer::Lexer;
use token_stream::TokenStream;

use crate::utils::exit_with_err_msg;

pub struct Compiler {
    file: String,
    phase: Phase,
}

impl Compiler {
    pub fn new(file: String, phase: Phase) -> Self {
        println!("Compiling...\n");

        Compiler {
            file: file,
            phase: phase,
        }
    }

    pub fn compile(&self) {
        let tokens = Lexer::new(&self.file).tokenize();

        let stream = if self.phase == Phase::Parser {
            TokenStream::new(tokens)
        } else {
            exit_with_err_msg("")
        };
    }
}

#[derive(PartialEq)]
pub enum Phase {
    Lexer,
    Parser,
    All,
}
