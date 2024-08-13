use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expr: Expr },
    If {condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>},
    Print { expr: Expr },
    Var {name: Token, initializer: Option<Expr>},
    While {condition: Expr, body: Box<Stmt>},
    Block {statements: Box<Vec<Stmt>>}
}

impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        use Stmt::*;

        match self {
            Expression { expr } => expr.to_string(),
            Print { expr } => format!("(print {})", expr.to_string()),
            Var { name, .. } => format!("Variable: {} {} ({})", name.get_lexeme(), name.get_type().to_string(), name.get_literal().to_string()),
            While { condition, body } => format!("while ({}) {}", condition.to_string(), (*body).to_string()),
            Block { statements } => {
                let mut output: Vec<String> = vec![];
                for statement in &**statements {
                    output.push(statement.to_string())
                }
                output.join("\n")
            },
            If { condition, then_branch, else_branch } => {
                let else_branch = match &(*else_branch) {
                    Some(s) => (*s).to_string(),
                    None => "".to_string()
                };
                format!("if {} then {}\n else {}",
                    condition.to_string(), 
                    (*then_branch).to_string(),
                    else_branch
                )
            }
        }
    }
}
