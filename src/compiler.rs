mod token;
mod token_stream;
mod lexer;

use token_stream::TokenStream;
use lexer::Lexer;

pub struct Compiler {
    file: String,
}

impl Compiler {
    pub fn new(file: String) -> Self {
        println!("Compiling file:\n{file}");
        Compiler { file: file }
    }

    pub fn compile(&self) {
        let tokens = Lexer::new(&self.file).tokenize();
        let _stream = TokenStream::new(tokens);
    }
}
