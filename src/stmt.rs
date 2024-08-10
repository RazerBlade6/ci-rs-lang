use crate::expr::Expr;
use crate::token::Token;

pub enum Stmt {
    Expression { expr: Expr },
    Print { expr: Expr },
    Var {name: Token, initializer: Option<Expr>}
}

impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        use Stmt::*;

        match self {
            Expression { expr } => expr.to_string(),
            Print { expr } => format!("(print {})", expr.to_string()),
            Var { name, .. } => format!("Variable: {} {} ({})", name.get_lexeme(), name.get_type().to_string(), name.get_literal().to_string())
        }
    }
}
