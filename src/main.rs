use std::env;
use std::fs;
use std::io::ErrorKind;

use crate::compiler::Compiler;
use crate::utils::exit_with_err_msg;
mod compiler;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit_with_err_msg("No input file.")
    }

    let path = &args[1];
    println!("Opening the file {path}");

    match fs::read_to_string(path) {
        Ok(file) => {
            Compiler::new(file).compile();
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => exit_with_err_msg("File not found."),
                _ => exit_with_err_msg(&format!("Unknown error: {e}")),
            };
        }
    }
}
