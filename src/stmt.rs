use crate::expr::{Expr, LitValue};
use crate::token::Token;

pub enum Stmt {
    Expression {expr: Expr},
    Print {expr: Expr},
}

impl Stmt {
    pub fn to_string(&self) -> String {
        use Stmt::*;

        match self {
            Expression {expr} => expr.to_string(),
            Print {expr} => format!("(print {})", expr.to_string()) 
        }
    }

    pub fn evaluate(&mut self) -> Result<LitValue, String> {
        match self {
            Self::Expression { expr } => return expr.evaluate(),
            Self::Print { expr } => todo!()
        }
    }
}