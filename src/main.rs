mod compiler;
mod utils;

use std::env;
use std::fs;
use std::io::ErrorKind;

use crate::compiler::Compiler;
use crate::compiler::Phase;
use crate::utils::exit_with_err_msg;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit_with_err_msg("No input file.")
    }

    let path = &args[1];
    println!("Opening the file {path}");

    let phase = if let Some(phase) = args.get(2) {
        match phase.as_str() {
            "lexer" => Phase::Lexer,
            "parser" => Phase::Parser,
            _ => Phase::All,
        }
    } else {
        Phase::All
    };

    match fs::read_to_string(path) {
        Ok(file) => {
            Compiler::new(path.to_string(), file, phase).compile();
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => exit_with_err_msg("File not found."),
                _ => exit_with_err_msg(&format!("Unknown error: {e}")),
            };
        }
    }
}
