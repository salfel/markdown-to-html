use crate::lexer::Lexer;
use std::{fs, process};

pub mod evaluater;
pub mod lexer;
pub mod parser;

fn main() {
    let filepath = "./input.md";
    let contents = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(message) => {
            println!("Got following error message: {}", message);
            process::exit(1);
        }
    };

    Lexer::new(contents);
}
