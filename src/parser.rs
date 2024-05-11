#![allow(unused)]

use std::process::exit;

use crate::token::*;
use crate::expr::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

#[derive(Debug)]
struct ParseError {
    msg: String,
    position: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens, current: 0}
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(error) => Err(error) 
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError>  {
        self.equality()
    }
    
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();

        while self.expect(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.peek(1);
            let right = match self.comparison() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Ok(Expr::new_binary(expr?, operator, right));
        }

        expr
    }
    
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.term() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        while self.expect(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.peek(1);
            let right = match self.term() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.factor() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        while self.expect(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.peek(1);
            let right = match self.factor() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) };
        }

        Ok(expr)
    }
    
    fn expect(&mut self, tokens: &[TokenType]) -> bool {
        for ttype in tokens {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }
    
    fn check(&mut self, ttype: &TokenType) -> bool {
        if self.is_at_end() {return false;}
        return self.peek(0).get_type() == *ttype;
        
    }
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {self.current += 1}
        return self.peek(1);
    }
    
    fn is_at_end(&mut self) -> bool {
        self.peek(0).get_type() == TokenType::Eof
    }
    
    fn peek(&mut self, n: usize) -> Token {
        self.tokens.get(self.current - n).unwrap().to_owned()
    }
    
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.unary() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        while self.expect(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.peek(1);
            let right = match self.unary() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) };
        }

        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.expect(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.peek(1);
            let right = match self.unary() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            return Ok(Expr::Unary { operator: operator, right: Box::new(right) });
        }

        
    }
    
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.expect(&[TokenType::False]) {return Ok(Expr::Literal { literal: LitValue::False(false)});}
        if self.expect(&[TokenType::True]) {return Ok(Expr::Literal { literal: LitValue::True(true)});}
        if self.expect(&[TokenType::Nil]) {return Ok(Expr::Literal { literal: LitValue::Nil });}

        if self.expect(&[TokenType::Number])  {
            let temp = match self.peek(1).get_literal() {
                Literal::Numeric(n) => n,
                _ => return Err(ParseError::new("Invalid use of Number", self.current))
            };
            let literal = LitValue::Number(temp);
        }

        if self.expect(&[TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            self.eat(TokenType::RightParen, "Expect ')' after expression");
            return Ok(Expr::Grouping { expr: Box::new(expr) })
        } 

        Err(ParseError::new("Invalid Expression", self.current))
    }
    
    fn eat(&mut self, ttype: TokenType, msg: &str) -> Result<Token, ParseError>  {
        if self.check(&ttype) {
            return Ok(self.advance());
        } else {
            return Err(ParseError::new(msg, self.current))
        }
        
    }
}

impl ParseError {
    fn new(msg: &str, current: usize) -> Self {
        Self {msg: msg.to_string(), position: current}
    }
}

