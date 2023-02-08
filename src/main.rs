use std::env;
use std::fs;
use std::io::ErrorKind;

use crate::compiler::Compiler;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No input file.")
    }

    let path = &args[1];
    println!("Opening the file {path}");

    match fs::read_to_string(path) {
        Ok(file)=> {
            Compiler::new(file).compile();
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => println!("File not found."),
                _ => println!("Unknown error: {e}"),
            };
        }
    }
}
