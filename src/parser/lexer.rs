
enum Token {
    Identifier(String),
    Integer(i32),
    Operator(String),
    Spaces(char),
    Comment(String),
    Punction(char),
}

pub struct Lexer {
    source: String,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn parse(source: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        tokens
    }

    fn identifier() -> Option<String> {
        let mut identifier = String::new();
        while self.source[self.pos] != ' ' && self.source[self.pos] != '\n' {
            identifier.push(self.source[self.pos]);
            self.pos += 1;
        }
        Some(identifier)
    }

    }

    fn next_char(&mut self) -> char {
        let c = self.source.chars().nth(self.pos).unwrap();
        self.pos += 1;
        c
    }

    fn peek_char(&mut self, by: usize) -> char {
        let c = self.source.chars().nth(self.pos + by).unwrap();
        c
    }


}

