//! # The container module for the Token Scanner
//!
//! Holds all functionality for token scanning and lexing.
//! Though the module is called `scanner`, it is more properly described as a lexer.
//!
//! The only exposed components of this module are the
//! `Scanner::new(src: &str) -> Self` and `Scanner::scan_tokens() -> Vec<Token>`
//!
//! Note: This method returns a Vector of Tokens from source strings.
//! However, it does not itself contain specifications of said tokens.
//! Those details may be found in `crate::token`
//!
//! ## Usage
//! ```
//! use scanner::Scanner;
//! fn main() {
//!     let src: &str = "some_example_str";
//!     let mut scanner: Scanner = Scanner::new(src);
//!     let tokens: Vec<Token> = scanner.scan_tokens();
//! }
//! ```
use crate::token::{Literal, Token, TokenType as Type};
use lazy_static::*;
use std::collections::HashMap;

pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, Type> = {
        let map = HashMap::<&'static str, Type>::from([
            ("and", Type::And),
            ("class", Type::Class),
            ("else", Type::Else),
            ("false", Type::False),
            ("for", Type::For),
            ("fun", Type::Fun),
            ("if", Type::If),
            ("nil", Type::Nil),
            ("or", Type::Or),
            ("print", Type::Print),
            ("return", Type::Return),
            ("super", Type::Super),
            ("this", Type::This),
            ("true", Type::True),
            ("var", Type::Var),
            ("while", Type::While),
        ]);
        map
    };
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_token = Token::new(Type::Eof, "", Literal::Null, self.line);
        self.tokens.push(eof_token);
        self.tokens.clone()
    }

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_t(Type::LeftParen),
            ')' => self.add_token_t(Type::RightParen),
            '{' => self.add_token_t(Type::LeftBrace),
            '}' => self.add_token_t(Type::RightBrace),
            ',' => self.add_token_t(Type::Comma),
            '.' => self.add_token_t(Type::Dot),
            '-' => self.add_token_t(Type::Minus),
            '+' => self.add_token_t(Type::Plus),
            ';' => self.add_token_t(Type::SemiColon),
            '*' => self.add_token_t(Type::Star),
            '!' => {
                if self.expect('=') {
                    self.add_token_t(Type::BangEqual);
                } else {
                    self.add_token_t(Type::Bang)
                }
            }
            '=' => {
                if self.expect('=') {
                    self.add_token_t(Type::EqualEqual);
                } else {
                    self.add_token_t(Type::Equal);
                }
            }
            '<' => {
                if self.expect('=') {
                    self.add_token_t(Type::LessEqual);
                } else {
                    self.add_token_t(Type::Less);
                }
            }
            '>' => {
                if self.expect('=') {
                    self.add_token_t(Type::GreaterEqual);
                } else {
                    self.add_token_t(Type::Greater);
                }
            }
            '/' => {
                if self.expect('/') {
                    while self.peek(0) != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else if self.expect('*') {
                    self.multi_line_comment();
                } else {
                    self.add_token_t(Type::Slash);
                }
            }
            '"' => self.string(),
            ' ' => (),
            '\r' => (),
            '\n' => self.current += 1,
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.error("", "Unexpected Character {c}")
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.src.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.src.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token_t(&mut self, token_type: Type) {
        self.add_token(token_type, Literal::Null);
    }

    fn add_token(&mut self, token_type: Type, literal: Literal) {
        let text = &self.src[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn add_token_s(&mut self, token_type: Type, literal: Literal, text: &str) {
        self.tokens.push(Token::new(token_type, text, literal, self.line))
    }

    fn number(&mut self) {
        while self.peek(0).is_ascii_digit() {
            self.advance();
        }

        if self.peek(0) == '.' && self.peek(1).is_ascii_digit() {
            self.advance();
            while self.peek(0).is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(
            Type::Number,
            Literal::Numeric(self.src[self.start..self.current].parse::<f64>().unwrap()),
        )
    }

    pub fn error(&self, loc: &str, msg: &str) {
        eprintln!("Error at line {0}, {loc}: {msg}", self.line)
    }

    fn peek(&self, n: usize) -> char {
        match self.src.chars().nth(self.current + n) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn string(&mut self) {
        while self.peek(0) != '"' && !self.at_end() {
            if self.peek(0) == '\n' {
                self.line += 1
            }
            if self.at_end() {
                self.error("Here", "Unterminated String")
            }
            self.advance();
        }

        self.advance();

        let text = self.src[self.start + 1..self.current - 1].to_string();
        self.add_token_s(Type::String, Literal::Str(text.to_string()), &text);
    }

    fn identifier(&mut self) {
        let mut c = self.peek(0);
        while c.is_ascii_alphanumeric() || c == '_' {
            self.advance();
            c = self.peek(0);
        }
        let s = &self.src[self.start..self.current];
        let token_type = match KEYWORD_MAP.get(&s) {
            Some(t) => t.clone(),
            None => Type::Identifier,
        };

        let literal = match token_type {
            Type::Identifier => Literal::Id(s.to_string()),
            _ => Literal::Keyword(s.to_string()),
        };
        self.add_token(token_type, literal)
    }

    fn multi_line_comment(&mut self) {
        while self.peek(0) != '*' && !self.at_end() {
            if self.peek(1) == '/' {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::*;

    #[test]
    fn test_scanner() {
        let src = "123 + 45.67";
        let mut scanner: Scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();

        assert_eq!(
            vec![
                Token::new(Type::Number, "123", Literal::Numeric(123.0), 1),
                Token::new(Type::Plus, "+", Literal::Null, 1),
                Token::new(Type::Number, "45.67", Literal::Numeric(45.67), 1),
                Token::new(Type::Eof, "", Literal::Null, 1)
            ],
            tokens
        );
    }
}
