mod lexer;
mod token;
mod token_stream;

use lexer::Lexer;
use token_stream::TokenStream;

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
        let _stream = TokenStream::new(tokens);
    }
}

pub enum Phase {
    Lexer,
    Parser,
    All,
}
