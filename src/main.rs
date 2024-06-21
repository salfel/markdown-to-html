use std::{fs, process};

fn main() {
    let filepath = "./input.md";
    let contents = get_contents(filepath);
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
