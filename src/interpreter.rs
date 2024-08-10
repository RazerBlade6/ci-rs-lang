use crate::expr::LitValue;
use crate::stmt::Stmt;
use crate::environment::Environment;

#[derive(Clone)]
pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Self {environment}
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Stmt::Expression { expr } => {
                    expr.evaluate(&mut self.environment)?;
                }
                Stmt::Print { expr } => {
                    let result = expr.evaluate(&mut self.environment)?;
                    println!("{}", result.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value: LitValue = match initializer {
                        Some(expr) => expr.evaluate(&mut self.environment)?,
                        None => LitValue::Nil
                    };

                    self.environment.define(name.get_lexeme().to_string(), value)
                }
            };
        }

        return Ok(());
    }
}
