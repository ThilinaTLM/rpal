use std::fs;

mod parser;

fn main() {
    let content = fs::read_to_string("assets/hello.rpal").expect("Unable to read file");
    let mut lexer = parser::lexer::Lexer::new(content);
    println!("{:?}", lexer.parse());
}
