use std::{fs, process};

pub mod lexer;
pub mod parser;

fn main() {
    let filepath = "./input.md";
    let _contents = get_contents(filepath);
}

fn get_contents(filepath: &str) -> String {
    match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(message) => {
            println!("Got following error message: {}", message);
            process::exit(1);
        }
    }
}
