#![allow(unused, unused_variables)]
use std::cell::RefCell;
use std::rc::Rc;
use crate::callable::Callable;
use crate::{environment, token::*};
use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LitValue {
    Number(f64),
    Str(String),
    True,
    False,
    Nil,
    Callable
}

use LitValue::*;

impl LitValue {
    pub fn to_string(&self) -> String {
        match self {
            Number(n) => return format!("{:.5}", n),
            Str(s) => return s.to_string(),
            True => return String::from("true"),
            False => return String::from("false"),
            Nil => return String::from("nil"),
            Callable => return String::from("Callable")
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(match token.lexeme.parse::<f64>() {
                Ok(f) => f,
                Err(_) => panic!("Could not parse as Number"),
            }),
            TokenType::String => Self::Str(token.lexeme.to_string()),
            TokenType::Identifier => Self::Str(token.lexeme.to_string()),
            TokenType::True => True,
            TokenType::False => False,
            _ => panic!("Could Not get literal from {}", token.lexeme),
        }
    }

    pub fn is_falsy(&self) -> LitValue {
        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    True
                } else {
                    False
                }
            }
            Self::Str(s) => {
                if s.len() == 0 {
                    True
                } else {
                    False
                }
            }
            Self::True => return False,
            Self::False => return True,
            Self::Nil => return True,
            Self::Callable => return False
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    false
                } else {
                    true
                }
            }
            Self::Str(s) => {
                if s.len() == 0 {
                    false
                } else {
                    true
                }
            }
            Self::True => return true,
            Self::False => return false,
            Self::Nil => return false,
            Self::Callable => return true
        }
    }    

    pub fn is_nil(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    }, Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    },
    Literal {
        literal: LitValue,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Operator {
        token: Token,
    },
    Variable {
        name: Token
    },
    Assignment {
        name: Token,
        value: Box<Expr>
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!("{} {} {}", operator.lexeme, (*left).to_string(), (*right).to_string())
            }
            Expr::Grouping { expr } => {
                format!("({})", (*expr).to_string())
            }
            Expr::Literal { literal } => {
                format!("{}", literal.to_string())
            }
            Expr::Unary { operator, right } => {
                format!("{}{}", operator.lexeme, (*right).to_string())
            }
            Expr::Operator { token } => token.lexeme.to_string(),
            Expr::Variable { name } => name.lexeme.to_string(),
            Expr::Assignment { name, value } => {
                format!("{} = {}", (*name).to_string(), (*value).to_string())
            },
            Expr::Logical { left, operator, right } => {
                format!("`{}` {} `{}`", (*left).to_string(), operator.to_string(), (*right).to_string())
            }
            Expr::Call { callee, paren, arguments } => {
                format!("function {}()", paren.lexeme)
            },
        }
    }

    pub fn create_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::from(left),
            operator,
            right: Box::from(right),
        }
    }

    pub fn create_grouping(expr: Expr) -> Self {
        Self::Grouping {
            expr: Box::from(expr),
        }
    }

    pub fn create_literal(literal: LitValue) -> Self {
        Self::Literal { literal }
    }

    pub fn create_logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical { left: Box::from(left), operator, right: Box::from(right)}
    }

    pub fn create_unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::from(right),
        }
    }

    pub fn create_operator(token: Token) -> Self {
        Self::Operator { token }
    }

    pub fn create_variable(name: Token) -> Self {
        Self::Variable { name }
    }

    pub fn create_assigment(name: Token, value: Expr) -> Self {
        Self::Assignment { name, value: Box::from(value) }
    }

    pub fn create_call(callee: Expr,
        paren: Token,
        arguments: Vec<Expr>) -> Self {
            Self::Call { callee: Box::from(callee), 
                paren, 
                arguments
            }
        }
 
    pub fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Result<LitValue, String> {
        match &self {
            Expr::Literal { literal } => Ok((*literal).clone()),
            Expr::Grouping { expr } => (*expr).evaluate(environment),
            Expr::Unary { operator, right } => Self::evaluate_unary(environment, operator, right),
            Expr::Binary {left, operator, right} 
            => Self::evaluate_binary(environment, left, operator, right),
            Expr::Variable { name } => {
                match environment.borrow().get(name.lexeme.to_string())? {
                    Some(v) => Ok(v),
                    None => return Err(format!("Variable {} not found", name.lexeme)),
                }
            },
            Expr::Assignment { name, value } => {
                let new_value = (*value).evaluate(environment.clone())?;
                environment.borrow_mut().assign(&name.lexeme, new_value.clone())?;
                Ok(new_value)
            }
            Expr::Logical { left, operator, right } => {
                let left: LitValue = left.evaluate(environment.clone())?;
                if operator.token_type == TokenType::Or {
                    if left.is_truthy() {return Ok(left)}
                } else {
                    if !left.is_truthy() {return Ok(left)}
                }

                return right.evaluate(environment);
            },
            Expr::Call { callee, paren, arguments } => {
                let mut callee = &mut callee.evaluate(environment.clone())? as &mut dyn Callable;
                // let mut callee = (&mut callee) as &mut dyn Callable;
                
                let mut argument_literals: Vec<LitValue> = Vec::new();
                for arg in &**arguments {
                    argument_literals.push(arg.evaluate(environment.clone())?);
                }
                let arguments = argument_literals;

                if arguments.len() != callee.arity() {
                    return Err(format!("Expected {} arguments but got {}", callee.arity(), arguments.len()))
                }

                callee.call(arguments)
            }  
            _ => Err(format!("Raw operators are not supported")),
        }
    }

    fn evaluate_unary(environment: Rc<RefCell<Environment>>, operator: &Token, right: &Box<Expr>) -> Result<LitValue, String> {
        let right = (*right).evaluate(environment)?;

        match (&right, operator.token_type) {
            (Number(x), TokenType::Minus) => return Ok(Number(-x)),
            (_, TokenType::Minus) => {
                return Err(format!("negation not implemented for {}", right.to_string()))
            }
            (any, TokenType::Bang) => Ok(any.is_falsy()),
            _ => todo!(),
        }
    }

    fn evaluate_binary(environment: Rc<RefCell<Environment>>, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Result<LitValue, String> {
        let left = (*left).evaluate(environment.clone())?;
        let right = (*right).evaluate(environment)?;

        match (&left, operator.token_type, &right) {
            (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),

            (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),

            (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),

            (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),

            (Number(x), TokenType::Percent, Number(y)) => Ok(Number(x % y)),

            (Str(s1), TokenType::Plus, Str(s2)) => Ok(Str(s1.to_owned() + s2)),

            (Number(x), TokenType::Greater, Number(y)) => { if x > y { Ok(True) } else { Ok(False)}}

            (Number(x), TokenType::GreaterEqual, Number(y)) => { if x >= y { Ok(True) } else { Ok(False) }}

            (Number(x), TokenType::Less, Number(y)) => { if x < y { Ok(True) } else { Ok(False) }}

            (Number(x), TokenType::LessEqual, Number(y)) => {if x <= y {  Ok(True) } else { Ok(False) }}

            (x, TokenType::EqualEqual, y) => { if x == y { Ok(True) } else { Ok(False) }}

            (x, TokenType::BangEqual, y) => { if x != y { Ok(True) } else { Ok(False) }}

            _ => {Err(format!("{} not implemented between {} and {}", operator.to_string(), left.to_string(), right.to_string()))
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn test_to_string() {
        let soln1 = String::from("-123 * (45.67)");
        let expr1 = Expr::Binary { left: Box::new(Expr::Unary { operator: Token::new(TokenType::Minus, "-", Literal::Null, 1), right: (Box::new(Expr::Literal {literal: Number(123.0),})),}), operator: Token::new(TokenType::Star, "*", Literal::Null, 1), right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { literal: Number(45.67), }),}),};        
        let res1 = expr1.to_string();
        assert_eq!(res1, soln1);
    }

    #[test]
    fn test_evaluate() {
        let expr1 = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-", Literal::Null, 1),
                right: (Box::new(Expr::Literal {
                    literal: Number(123.0),
                })),
            }),
            operator: Token::new(TokenType::Star, "*", Literal::Null, 1),
            right: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Literal {
                    literal: Number(45.67),
                }),
            }),
        };
        let soln = LitValue::Number(-5617.41);
        let result = expr1.evaluate(Rc::from(RefCell::from(Environment::new()))).unwrap();
        assert_eq!(soln, result);
    }

    #[test]
    fn test_stringify() {
        let token = Token::new(
            TokenType::String,
            "Hello World",
            Literal::Str("Hello World".to_string()),
            1,
        );
        let literal_value = LitValue::from_token(token);
        println!("{}", literal_value.to_string());
    }
}
