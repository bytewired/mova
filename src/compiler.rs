pub struct Compiler {
    file: String
}

impl Compiler {
    pub fn new(file: String) -> Self {
        println!("Compiling file:\n{file}");
        Compiler { file: file }
    }

   pub fn compile(&self) {}
}