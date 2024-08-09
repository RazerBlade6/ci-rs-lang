#![allow(unused, unused_variables)]

use std::{fmt::format};

use crate::token::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LitValue {
    Number(f64),
    Str(String),
    True,
    False,
    Nil
}

use LitValue::*;

impl LitValue {
    pub fn to_string(&self) -> String {
        match self {
            Number(n) => return format!("{:.5}", n),
            Str(s) => return s[1 .. s.len() - 1].to_string(),
            True => return String::from("true") ,
            False => return String::from("false"),
            Nil => return String::from("nil")
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.get_type() {
            TokenType::Number => Self::Number(match token.get_lexeme().parse::<f64>() {
                Ok(f) => f,
                Err(_) => panic!("Could not parse as Number")
            }),

            TokenType::String => Self::Str(token.get_lexeme().to_string()),
            TokenType::Identifier => Self::Str(token.get_lexeme().to_string()),
            TokenType::True => True,
            TokenType::False => False,
            _ => panic!("Could Not get literal from {}", token.to_string())
        }
    } 

    pub fn is_falsy(&self) -> LitValue {
        match self {
            Self::Number(x) => if *x == 0.0 {True} else {False},
            Self::Str(s) => if s.len() == 0 {True} else {False},
            Self::True => return False,
            Self::False => return True,
            Self::Nil => return True
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Nil => true,
            _ => false
        }
    }
}

pub enum Expr {
    Binary {left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping {expr: Box<Expr>},
    Literal {literal: LitValue},
    Unary {operator: Token, right: Box<Expr>},
    Operator {token: Token},
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => {
                format!("{} {} {}", operator.get_lexeme(), (*left).to_string(), (*right).to_string())
            },
            Expr::Grouping { expr } => {
                format!("({})", (*expr).to_string())
            },
            Expr::Literal { literal } => {
                format!("{}", literal.to_string())
            },
            Expr::Unary { operator, right } => { 
                format!("{}{}",operator.get_lexeme(), (*right).to_string())
            },
            Expr::Operator { token } => {
                token.get_lexeme().to_string()
            },
        }
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary { left: Box::from(left), operator, right: Box::from(right) }
    }    

    pub fn new_grouping(expr: Expr) -> Self {
        Self::Grouping { expr: Box::from(expr) }
    }

    pub fn new_literal(literal: LitValue) -> Self {
        Self::Literal { literal }
    }

    pub fn new_unary(operator: Token, right: Expr) -> Self {
        Self::Unary { operator, right: Box::from(right) }
    }

    pub fn new_operator(token: Token) -> Self {
        Self::Operator { token }
    }

    pub fn evaluate(&self) -> Result<LitValue, String> {
        match self {
            Expr::Literal { literal } => Ok((*literal).clone()),
            Expr::Grouping { expr } => (*expr).evaluate(),
            Expr::Unary { operator, right } => {
                Self::evaluate_unary(operator.clone(), right)
            }
            Expr::Binary { left, operator, right } => {
                Self::evaluate_binary(left, operator.clone(), right)
            },
            _ => return Err(format!("Raw operators are not supported"))
        }
    }

    fn evaluate_unary(operator: Token, right: &Box<Expr>) -> Result<LitValue, String> {
        let right = (*right).evaluate()?;

        match (&right, operator.get_type()) {
            (Number(x), TokenType::Minus) => return Ok(Number(-x)),
            (_, TokenType::Minus) => return Err(format!("negation not implemented for {}", right.to_string())),
            (any, TokenType::Bang) => Ok(any.is_falsy()),
            _ => todo!()
        }
    }

    fn evaluate_binary(left: &Box<Expr>, operator: Token, right: &Box<Expr>) -> Result<LitValue, String> {
        let left = (*left).evaluate()?;
        let right = (*right).evaluate()?;

        match (&left, operator.get_type(), &right) {
            (Number(x), TokenType::Minus, Number(y)) => return Ok(Number(x - y)),

            (Number(x), TokenType::Star, Number(y)) => return Ok(Number(x * y)),

            (Number(x), TokenType::Slash, Number(y)) => return Ok(Number(x / y)),

            (Number(x), TokenType::Plus, Number(y)) => return Ok(Number(x + y)),

            (Str(s1), TokenType::Plus, Str(s2)) => return Ok(Str(s1.to_owned() + s2)),

            (Number(x), TokenType::Greater, Number(y)) => if x > y {return Ok(True); } else {return Ok(False);},

            (Number(x), TokenType::GreaterEqual, Number(y)) => if x >= y {return Ok(True);} else {return Ok(False);},

            (Number(x), TokenType::Less, Number(y)) => if x < y {return Ok(True);} else {return Ok(False);},

            (Number(x), TokenType::LessEqual, Number(y)) => if x <= y {return Ok(True);} else {return Ok(False);},

            (x, TokenType::EqualEqual,y) => if x == y {return Ok(True);} else {return Ok(False);},

            (x, TokenType::BangEqual, y) => if x != y {return Ok(True);} else {return Ok(False);},

            _ => return Err(format!("{} not implemented between {} and {}", operator.to_string(), left.to_string(), right.to_string()))
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::parser::Parser;


    #[test]
    fn test_to_string() {
        let soln1 = String::from("-123 * (45.67)");
        let expr1 = Expr::Binary { left: Box::new(Expr::Unary { operator: Token::new(TokenType::Minus, "-", Literal::Null, 1), right: (Box::new(Expr::Literal { literal: Number(123.0)}))}), operator: Token::new(TokenType::Star, "*", Literal::Null, 1), right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { literal: Number(45.67)})})};
        let res1 = expr1.to_string();
        assert_eq!(res1, soln1);
    }

    #[test]
    fn test_evaluate() {
        let expr1 = Expr::Binary { left: Box::new(Expr::Unary { operator: Token::new(TokenType::Minus, "-", Literal::Null, 1), right: (Box::new(Expr::Literal { literal: Number(123.0)}))}), operator: Token::new(TokenType::Star, "*", Literal::Null, 1), right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { literal: Number(45.67)})})};
        let soln = LitValue::Number(-5617.41);
        let result = expr1.evaluate().unwrap();
        assert_eq!(soln, result);
    }

    #[test]
    fn test_stringify() {
        let token = Token::new(TokenType::String, "Hello World", Literal::Str("Hello World".to_string()), 1);
        let literal_value = LitValue::from_token(token);
        println!("{}", literal_value.to_string());
    }
}