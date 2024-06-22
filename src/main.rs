pub mod evaluator;
pub mod lexer;
pub mod parser;

use evaluator::Evaluator;
use std::io::stdin;
use std::{fs, process};

fn main() {
    let filepath = read_filepath("Enter the filepath of your markdown file: ", "input.md");
    let contents = get_contents(&filepath);

    let evaluator = Evaluator::new(contents);
    let output = evaluator.evaluate();

    let filepath = read_filepath("Enter the filepath of the output file: ", "index.html");
    write_to_file(&filepath, output);

    println!("Successfully wrote to index.html");
}

fn read_filepath(instruction: &str, default: &str) -> String {
    let stdin = stdin();
    let mut filepath = String::new();
    println!("{instruction} Default: {default}");
    stdin
        .read_line(&mut filepath)
        .expect("Failed reading input");

    if filepath.trim().is_empty() {
        default.to_string()
    } else {
        filepath.trim().to_string()
    }
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
