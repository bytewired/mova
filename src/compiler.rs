mod lexer;
mod parser;
pub mod token;
pub mod token_stream;
pub mod utils;

use lexer::Lexer;
use token_stream::TokenStream;

use parser::decl_parser::DeclParser;
use crate::utils::exit_with_err_msg;

pub struct Compiler {
    file_path: String,
    file_source: String,
    phase: Phase,
}

impl Compiler {
    pub fn new(file_path: String, file_source: String, phase: Phase) -> Self {
        println!("Compiling...\n");

        Compiler {
            file_path: file_path,
            file_source: file_source,
            phase: phase,
        }
    }

    pub fn compile(&self) {
        let tokens = Lexer::new(&self.file_path, &self.file_source).tokenize();

        let mut stream = if self.phase == Phase::Parser || self.phase == Phase::All {
            TokenStream::new(tokens)
        } else {
            exit_with_err_msg("")
        };

        let decls = DeclParser::new(&mut stream).parse();
    }
}

#[derive(PartialEq)]
pub enum Phase {
    Lexer,
    Parser,
    All,
}
