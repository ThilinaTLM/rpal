#![warn(dead_code, unused_variables)]

const OPERATOR_SYMBOLS: [char; 26] = [
    '+', '-', '*', '<', '>', '&', '.', '@', '/', ':', '=', '~', ',', '$', '!', '#', '%', 'ˆ', '_',
    '[', ']', '{', '}', '"', '‘', '?',
];

const PUNCTION_SYMBOLS: [char; 4] = ['(', ')', ';', ','];

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Integer(i32),
    Operator(String),
    String(String),
    Spaces(char),
    Comment(String),
    Punction(char),
}

#[derive(Debug)]
struct SourceReader {
    source: String,
    start: usize,
    pos: usize,
}

impl SourceReader {
    pub fn new(source: String) -> SourceReader {
        if source.len() == 0 {
            panic!("Source is empty");
        }
        SourceReader {
            source,
            start: 0,
            pos: 0,
        }
    }

    pub fn has_more(&self) -> bool {
        self.pos < self.source.len()
    }

    pub fn next(&mut self) {
        if self.has_more() {
            self.pos += 1;
        }
    }

    pub fn commit(&mut self) -> String {
        let slice = &self.source[self.start..self.pos];
        self.start = self.pos;
        slice.to_string()
    }

    pub fn rollback(&mut self) {
        self.pos = self.start;
    }

    pub fn is_char(&self, c: char) -> bool {
        self.source.chars().nth(self.pos) == Some(c)
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.pos).unwrap()
    }

    pub fn is_letter(&self) -> bool {
        let c = self.peek();
        c.is_alphabetic()
    }

    pub fn is_digit(&self) -> bool {
        let c = self.peek();
        c.is_digit(10)
    }

    pub fn is_whitespace(&self) -> bool {
        let c = self.peek();
        c.is_whitespace()
    }

    pub fn is_eol(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '\n'
    }
}

#[derive(Debug)]
pub struct Lexer {
    reader: SourceReader,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            reader: SourceReader::new(source),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        macro_rules! push_token {
            ($token:expr) => {
                if let Some(token) = $token {
                    tokens.push(token);
                    continue;
                }
            };
        }

        while self.reader.has_more() {
            self.reader.rollback();
            push_token!(self.identifier());
            push_token!(self.integer());
            push_token!(self.operator());
            push_token!(self.string());
            push_token!(self.spaces());
            push_token!(self.comment());
            push_token!(self.punction());
            break;
        }
        tokens
    }

    fn identifier(&mut self) -> Option<Token> {
        if !self.reader.is_letter() {
            return None;
        }
        while self.reader.is_letter() || self.reader.is_digit() || self.reader.is_char('_') {
            self.reader.next();
        }
        Some(Token::Identifier(self.reader.commit()))
    }

    fn integer(&mut self) -> Option<Token> {
        if !self.reader.is_digit() {
            return None;
        }
        while self.reader.is_digit() {
            self.reader.next();
        }
        Some(Token::Integer(self.reader.commit().parse().unwrap()))
    }

    fn operator(&mut self) -> Option<Token> {
        if OPERATOR_SYMBOLS.contains(&self.reader.peek()) {
            self.reader.next();
            return Some(Token::Operator(self.reader.commit()));
        }
        None
    }

    fn string(&mut self) -> Option<Token> {
        let brk = if self.reader.is_char('"') {
            self.reader.next();
            '"'
        } else if self.reader.is_char('\'') {
            self.reader.next();
            '\''
        } else {
            return None;
        };
        while !self.reader.is_char(brk) {
            self.reader.next();
        }
        Some(Token::String(self.reader.commit()))
    }

    fn spaces(&mut self) -> Option<Token> {
        if !self.reader.is_whitespace() && !self.reader.is_eol() {
            return None;
        }
        while self.reader.is_whitespace() || self.reader.is_eol() {
            self.reader.next();
        }
        Some(Token::Spaces(self.reader.commit().chars().nth(0).unwrap()))
    }

    fn comment(&mut self) -> Option<Token> {
        if !self.reader.is_char('/') {
            return None;
        }
        self.reader.next();
        if !self.reader.is_char('/') {
            return None;
        }
        self.reader.next();

        while !self.reader.is_eol() {
            self.reader.next();
        }
        Some(Token::Comment(self.reader.commit()))
    }

    fn punction(&mut self) -> Option<Token> {
        if PUNCTION_SYMBOLS.contains(&self.reader.peek()) {
            self.reader.next();
            return Some(Token::Punction(
                self.reader.commit().chars().nth(0).unwrap(),
            ));
        }
        None
    }
}
