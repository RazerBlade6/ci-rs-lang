use crate::expr::Expr;

pub enum Stmt {
    Expression {expr: Expr},
    Print {expr: Expr},
}

impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        use Stmt::*;

        match self {
            Expression {expr} => expr.to_string(),
            Print {expr} => format!("(print {})", expr.to_string()) 
        }
    }
}