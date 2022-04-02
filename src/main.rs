use std::fs;

mod parser;
use parser::Lexer;

fn main() {
    let content = fs::read_to_string("assets/hello.rpal").expect("Unable to read file");
    println!("{}", content);
}
