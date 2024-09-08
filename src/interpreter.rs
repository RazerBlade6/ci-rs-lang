
use crate::{
    callable::Callables,
    environment::Environment,
    expr::Literal,
    stmt::Stmt
};

use std::collections::HashMap;

pub struct Interpreter {
    pub environment: Environment,
    pub ret: Option<Literal>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(HashMap::new()),
            ret: None,
        }
    }

    pub fn new_with_env(environment: Environment) -> Self {
        Self {
            environment,
            ret: None,
        }
    }

    pub fn resolve(&mut self, locals: HashMap<usize, usize>) {
        for (k, v) in locals {
            self.environment.resolve(k, v);
        }
    }

    pub fn interpret(&mut self, statements: Vec<&Stmt>) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), String> {
        match statement {
            Stmt::Expression { expr } => {
                expr.evaluate(self.environment.clone())?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = condition.evaluate(self.environment.clone())?;

                match (condition.is_truthy(), else_branch) {
                    (true, _) => self.execute(&then_branch)?,
                    (false, Some(else_branch)) => self.execute(&else_branch)?,
                    (false, None) => return Ok(()),
                }
            }
            Stmt::Var { name, initializer } => {
                let value: Literal = match initializer {
                    Some(e) => e.evaluate(self.environment.clone())?,
                    None => Literal::Nil,
                };

                self.environment.define(name.lexeme.to_string(), value)
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(self.environment.clone())?.is_truthy() {
                    self.execute(&body)?;
                }
            }
            Stmt::Block { statements } => {
                let environment = self.environment.enclose();
                let old_environment = self.environment.clone();
                self.environment = environment;
                let result = self.interpret((*statements).iter().map(|b| b).collect());
                self.environment = old_environment;
                result?
            }
            Stmt::Function { name, params, body } => {
                let value = Callables::LoxFunction {
                    name: name.clone(),
                    params: params.clone(),
                    arity: params.len(),
                    body: body.clone(),
                    environment: self.environment.clone(),
                };

                self.environment
                    .define(name.lexeme.to_string(), Literal::Callable(value));
            }
            Stmt::Return { value } => {
                self.ret = match value {
                    Some(e) => Some(e.evaluate(self.environment.clone())?),
                    None => Some(Literal::Nil),
                };
            }
        };

        Ok(())
    }
}
