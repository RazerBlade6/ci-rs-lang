use std::collections::HashMap;
use lazy_static::*;

use crate::token::{Token, TokenType as TType, Literal};

pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, TType> = {
        let m = HashMap::<&'static str, TType>::from([
            ("and",    TType::And),
            ("class",  TType::Class),
            ("else",   TType::Else),
            ("false",  TType::False),
            ("for",    TType::For),
            ("fun",    TType::Fun),
            ("if",     TType::If),
            ("nil",    TType::Nil),
            ("or",     TType::Or),
            ("print",  TType::Print),
            ("return", TType::Return),
            ("super",  TType::Super),
            ("this",   TType::This),
            ("true",   TType::True),
            ("var",    TType::Var),
            ("while",  TType::While)
        ]); 
        m
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

        let eof_token=  Token::new(TType::Eof, "", Literal::Null, self.line);
        self.tokens.push(eof_token);
        self.tokens.clone()
    }

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance(); 
        match c {
            '(' => self.add_token_t(TType::LeftParen),
            ')' => self.add_token_t(TType::RightParen),
            '{' => self.add_token_t(TType::LeftBrace),
            '}' => self.add_token_t(TType::RightBrace),
            ',' => self.add_token_t(TType::Comma),
            '.' => self.add_token_t(TType::Dot),
            '-' => self.add_token_t(TType::Minus),
            '+' => self.add_token_t(TType::Plus),
            ';' => self.add_token_t(TType::SemiColon),
            '*' => self.add_token_t(TType::Star),
            '!' => { 
                if self.expect('=') {
                    self.add_token_t(TType::BangEqual);
                } else {
                    self.add_token_t(TType::Bang)
                }
            },
            '=' => { 
                if self.expect('=') {
                    self.add_token_t(TType::EqualEqual);
                } else {
                    self.add_token_t(TType::Equal);
                }
            },
            '<' => { 
                if self.expect('=') {
                    self.add_token_t(TType::LessEqual);
                } else {
                    self.add_token_t(TType::Less);
                }
            },
            '>' => { 
                if self.expect('=') {
                    self.add_token_t(TType::GreaterEqual);
                } else {
                    self.add_token_t(TType::Greater);
                }
            },
            '/' => {
                if self.expect('/') {
                    while self.peek(0) != '\n' && !self.at_end() {self.advance();}
                } else if self.expect('*'){
                    self.multiline_comment();
                } else {
                    self.add_token_t(TType::Slash);
                }
            }
            '"' => self.string(),
            ' ' => (),
            '\r' => (),
            '\n' => self.current += 1,
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_'{
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
        if self.at_end() { return false; }

        if self.src.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token_t(&mut self, ttype: TType) {
        self.add_token(ttype, Literal::Null);
    }

    fn add_token(&mut self, ttype: TType, literal: Literal) {
        let text = &self.src[self.start..self.current];
        self.tokens.push(Token::new(ttype, text, literal, self.line));
    }

    fn number(&mut self) {
        while self.peek(0).is_ascii_digit() {self.advance();}

        if self.peek(0) == '.' && self.peek(1).is_ascii_digit(){
            self.advance();
            while self.peek(0).is_ascii_digit() {self.advance();}
        }

        self.add_token(TType::Number, Literal::Numeric(self.src[self.start..self.current].parse::<f64>().unwrap()))
    }

    pub fn error(&self, loc: &str, msg: &str) {
        eprintln!("Error at line {0}, {loc}: {msg}", self.line)
    }
    
    fn peek(&self, n: usize) -> char {
        if self.current + n >= self.src.len() { return '\0'}
        return self.src.chars().nth(self.current + n).unwrap()
    }
    
    fn string(&mut self) {
        while self.peek(0) != '"' && !self.at_end() {
            if self.peek(0) == '\n' {self.line += 1}
            if self.at_end() {self.error("Here", "Unterminated String")}
            self.advance();
        }

        self.advance();

        let literal = self.src[self.start + 1..self.current - 1].to_string();
        self.add_token(TType::String, Literal::Str(literal));
    }
    
    fn identifier(&mut self) {
        let mut c = self.peek(0);
        while c.is_ascii_alphanumeric() || c == '_' {
            self.advance();
            c = self.peek(0);
        }
        let s = &self.src[self.start..self.current];

        let token_type = match KEYWORD_MAP.get(&s) {
            Some(t) => t.to_owned(),
            None => TType::Identifier
        };

        let literal = match token_type {
            TType::Identifier => Literal::Id(s.to_string()),
            _                 => Literal::Keyword(s.to_string())
        };
        self.add_token(token_type, literal)
    }
    
    fn multiline_comment(&mut self) {
        while self.peek(0) != '*' && !self.at_end() {
            
        }
    }
}