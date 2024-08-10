//! # Module for the Token Struct and TokenType Enum
//!
//! Contains definitions and implementations of Token and TokenType
//!
//! TokenType implements basic `to_string()` functionality, though it actually returns
//! an `&str`
//!
//! ## Usage
//! ```
//! use token::*;
//!
//! fn main() {
//!     let token  = Token::new(
//!         token_type: LeftParen,
//!         lexeme: "(",
//!         literal: Literal::Null,
//!         line: 0
//!     )
//! }
//! ```
//!
//! Further, in the translation from Java to Rust, the `Literal` enum, which holds
//! values for various Tokens.
//!

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub fn to_string(&self) -> &str {
        match self {
            TokenType::LeftParen => return "Left Parenthesis",
            TokenType::RightParen => return "Right Parenthesis",
            TokenType::LeftBrace => return "Left Brace",
            TokenType::RightBrace => return "Right Brace",
            TokenType::Comma => return "Comma",
            TokenType::Dot => return "Dot",
            TokenType::Minus => return "Minus",
            TokenType::Plus => return "Plus",
            TokenType::SemiColon => return "Semicolon",
            TokenType::Slash => return "Slash",
            TokenType::Star => return "Star",
            TokenType::Bang => return "Not",
            TokenType::BangEqual => return "Not Equal",
            TokenType::Equal => return "Assignment",
            TokenType::EqualEqual => return "Equals",
            TokenType::Greater => return "Greater",
            TokenType::GreaterEqual => return "Greater Equal",
            TokenType::Less => return "Less",
            TokenType::LessEqual => return "Less Equal",
            TokenType::Identifier => return "Identifier",
            TokenType::String => return "String",
            TokenType::Number => return "Number",
            TokenType::And => return "And",
            TokenType::Class => return "Class",
            TokenType::Else => return "Else",
            TokenType::False => return "False",
            TokenType::Fun => return "Fun",
            TokenType::For => return "For",
            TokenType::If => return "If",
            TokenType::Nil => return "Nil",
            TokenType::Or => return "Or",
            TokenType::Print => return "Print",
            TokenType::Return => return "Return",
            TokenType::Super => return "Super",
            TokenType::This => return "This",
            TokenType::True => return "True",
            TokenType::Var => return "Var",
            TokenType::While => return "While",
            TokenType::Eof => return "Eof",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Numeric(f64),
    Str(String),
    Id(String),
    Keyword(String),
    Null,
}

impl Literal {
    pub fn to_string(&self) -> &str {
        match self {
            Literal::Numeric(_) => return "Numeric",
            Literal::Str(s) => return s.as_str(),
            Literal::Id(_) => return "Identifier",
            Literal::Keyword(s) => return s.as_str(),
            Literal::Null => return "Null",
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Literal::Null => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        if self.literal.is_null() {
            return format!("`{} ({})`", self.lexeme, self.token_type.to_string());
        } else {
            return format!("`{}, {}`", self.lexeme, self.literal.to_string());
        }
    }

    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }

    // // TODO: MAKE THIS GARBAGE SOLUTION BETTER AT SOME POINT

    // pub fn get_literal(&self) -> String {
    //     match &self.literal {
    //         Literal::Str(literal) => literal.to_string(),
    //         _ => panic!("Cannot access literal values")
    //     }
    // }

    pub fn get_line(&self) -> usize {
        self.line
    }
}
