
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

struct SourceReader {
    source: String,
    start: usize,
    pos: usize,
}

impl SourceReader {

    pub fn new(source: String) -> SourceReader {
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
        self.pos += 1;
    }

    pub fn commit(&mut self) -> String {
        let slice = &self.source[self.start..self.pos];
        self.start = self.pos;
        slice.to_string()
    }

    pub fn rollback(&mut self) {
        self.pos = self.start;
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.pos).unwrap()
    }

    pub fn is_char(&self, c: char) -> bool {
        self.source.chars().nth(self.pos) == Some(c)
    }

    pub fn is_letter(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c.is_alphabetic()
    }

    pub fn is_digit(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c.is_digit(10)
    }

    pub fn is_underscore(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '_'
    }

    pub fn is_whitespace(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c.is_whitespace()
    }

    pub fn is_eol(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '\n'
    }

    pub fn is_operator_symbol(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '+' || c == '-' || c == '*' || c == '<' || c == '>' || c == '&' || c == '.' ||
        c == '@' || c == '/' || c == ':' || c == '=' || c == '~' || c == '|' || c == '$' ||
        c == '!' || c == '#' || c == '%' || c == '^' || c == '_' || c == '[' || c == ']' ||
        c == '{' || c == '}' || c == '"' || c == '\'' || c == '?'
    }

    pub fn is_quote(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '"' || c == '\''
    }

    pub fn is_open_pran(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '('
    }

    pub fn is_close_pran(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == ')'
    }

    pub fn is_semicolon(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == ';'
    }

    pub fn is_comma(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == ','
    }

    pub fn is_dot(&self) -> bool {
        let c = self.source.chars().nth(self.pos).unwrap();
        c == '.'
    }

}


pub fn parse(source: String) -> Vec<Token> {
    let mut reader = SourceReader::new(source);
    let mut tokens = Vec::new();

    while reader.has_more() {
        if reader.is_letter() { // identifer
            while reader.is_letter() || reader.is_digit() || reader.is_underscore() {
                reader.next();
            }
            tokens.push(Token::Identifier(reader.commit()));
        } else if reader.is_digit() { // integer
            while reader.is_digit() {
                reader.next();
            } 
            tokens.push(Token::Integer(reader.commit().parse().unwrap()));
        } else if reader.is_whitespace() || reader.is_eol() { // spaces
            while reader.is_whitespace() || reader.is_eol() {
                reader.next();
            }
            tokens.push(Token::Spaces(reader.commit().chars().nth(0).unwrap()));
        } else if reader.is_operator_symbol() { // operator 
            reader.next();
            tokens.push(Token::Operator(reader.commit()));
        } else if reader.is_quote() { // string
            let margin_ch = reader.peek();
            reader.next();
            while !reader.is_char(margin_ch) {
                if reader.is_eol() {
                    panic!("unterminated string literal");
                }
                if reader.is_char('\\') {
                    //
                }
                reader.next();
            }

        } else {
            break;
        }
    }
    tokens
}

