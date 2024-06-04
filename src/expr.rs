#![allow(unused, unused_variables)]

use crate::token::*;

pub enum LitValue {
    Number(f64),
    Str(String),
    True(bool),
    False(bool),
    Nil
}

impl LitValue {
    pub fn to_string(&self) -> String {
        match self {
            LitValue::Number(n) => return format!("{n}"),
            LitValue::Str(s) => return s.to_string(),
            LitValue::True(_) => return String::from("true") ,
            LitValue::False(_) => return String::from("false"),
            LitValue::Nil => return String::from("nil")
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
            TokenType::True => LitValue::True(true),
            TokenType::False => LitValue::False(false),
            _ => panic!("Could Not get literal from {}", token.to_string())
        }
    } 
}

pub enum Expr {
    Binary {left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping {expr: Box<Expr>},
    Literal {literal: LitValue},
    Unary {operator: Token, right: Box<Expr>},
    Operator {token: Token}
}

impl Expr {

    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => {
                format!("{} {} {}", (*left).to_string(), operator.get_lexeme(), (*right).to_string())
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
            }
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
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        
        let soln1 = String::from("-123 * (45.67)");

        let expr1 = Expr::Binary { left: Box::new(Expr::Unary { operator: Token::new(TokenType::Minus, "-", Literal::Null, 1), right: (Box::new(Expr::Literal { literal: LitValue::Number(123.0) }))}), operator: Token::new(TokenType::Star, "*", Literal::Null, 1), right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { literal: LitValue::Number(45.67) }) })};

        let res1 = expr1.to_string();
        assert_eq!(res1, soln1);

    }
}