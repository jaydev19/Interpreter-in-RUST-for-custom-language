// src/lexer.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Identifier(String),
    Assign,
    Print,
    PrintLn,
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Semicolon,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];
        match ch {
            'p' => {
                if self.match_keyword("pn") {
                    return Token::Print;
                } else if self.match_keyword("pnl") {
                    return Token::PrintLn;
                }
            }
            'l' => {
                if self.match_keyword("let") {
                    return Token::Let;
                }
            }
            'a'..='z' | 'A'..='Z' => return self.read_identifier(),
            '0'..='9' => return self.read_number(),
            '=' => {
                self.position += 1;
                return Token::Assign;
            }
            '+' => {
                self.position += 1;
                return Token::Plus;
            }
            '-' => {
                self.position += 1;
                return Token::Minus;
            }
            '*' => {
                self.position += 1;
                return Token::Multiply;
            }
            '/' => {
                self.position += 1;
                return Token::Divide;
            }
            ';' => {
                self.position += 1;
                return Token::Semicolon;
            }
            _ => {}
        }

        Token::EOF
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        if self.position + keyword.len() <= self.input.len()
            && &self.input[self.position..self.position + keyword.len()]
                == keyword.chars().collect::<Vec<_>>()
        {
            self.position += keyword.len();
            true
        } else {
            false
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
            self.position += 1;
        }
        let identifier: String = self.input[start..self.position].iter().collect();
        Token::Identifier(identifier)
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            self.position += 1;
        }
        let number: f64 = self.input[start..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();
        Token::Number(number)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }
}

