use evaluater::Evaluator;

use std::{fs, process};

pub mod evaluater;
pub mod lexer;
pub mod parser;

fn main() {
    let filepath = "./input.md";
    let contents = get_contents(filepath);

    let evaluater = Evaluator::new(contents);
    let evaluated = evaluater.eval();

    write_to_file("./index.html", evaluated)
}

fn write_to_file(filepath: &str, contents: String) {
    match fs::write(filepath, contents) {
        Ok(()) => {}
        Err(message) => {
            println!("Err, message: {}", message);
            process::exit(1);
        }
    }
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
