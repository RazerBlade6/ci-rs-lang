use std::cell::RefCell;
use std::rc::Rc;

use crate::expr::LitValue;
use crate::stmt::Stmt;
use crate::environment::Environment;

#[derive(Clone)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {environment: Rc::from(RefCell::from(Environment::new()))}
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }

        return Ok(());
    }

    fn execute(&mut self, statement: Stmt) -> Result<(), String> {
        match statement {
            Stmt::Expression { expr } => {
                expr.evaluate(self.environment.clone())?;
            }
            Stmt::If { condition, then_branch, else_branch } => {
                if condition.evaluate(self.environment.clone())?.is_truthy() {
                    self.execute(*then_branch)?
                } else if (*else_branch).is_some() {
                    self.execute((*else_branch).unwrap())?
                } else {
                    return Ok(())
                }
            }

            Stmt::Print { expr } => {
                let result = expr.evaluate(self.environment.clone())?;
                println!("{}", result.to_string());
            }
            Stmt::Var { name, initializer } => {
                let value: LitValue = match initializer {
                    Some(expr) => expr.evaluate(self.environment.clone())?,
                    None => LitValue::Nil
                };

                self.environment
                    .borrow_mut()
                    .define(name.get_lexeme().to_string(), value)
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(self.environment.clone())?.is_truthy() {
                    self.execute((*body).clone())?;
                }
            },
            Stmt::Block { statements } => {
                let mut block_environment = Environment::new();
                block_environment.enclosing = Some(self.environment.clone());
                match self.execute_block(*statements, self.environment.clone()) {
                    Ok(_) => (),
                    Err(msg) => println!("{msg}")
                };
            }
        };

        Ok(())
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), String> {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => (),
                Err(_) => {
                    self.environment = previous;
                    return Err("Block failed to execute fully".to_string())
                }
            };
        }
        self.environment = previous;
        Ok(())
    }

    
}
