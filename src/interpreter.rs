use std::cell::RefCell;
use std::rc::Rc;

use crate::expr::LitValue;
use crate::stmt::Stmt;
use crate::environment::Environment;


pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: Rc::new(RefCell::new(Environment::new()))}
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
            Stmt::If { condition, then_branch, else_branch } => {
                let condition = condition.evaluate(self.environment.clone())?;

                match (condition.is_truthy(), else_branch) {
                    (true, _) => self.execute(&then_branch)?,
                    (false, Some(else_branch)) => self.execute(&else_branch)?,
                    (false, None) => return Ok(())
                }
            }
    
            Stmt::Print { expr } => {
                let result = expr.evaluate(self.environment.clone())?;
                println!("{}", result.to_string());
            }
            Stmt::Var { name, initializer } => {
                let value: LitValue = initializer.evaluate(self.environment.clone())?;

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.to_string(), value)
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(self.environment.clone())?.is_truthy() {
                    self.execute(&body)?;
                }
            },
            Stmt::Block { statements } => {
                let mut environment = Environment::new();
                environment.enclosing = Some(self.environment.clone());
                let old_environment = self.environment.clone();
                self.environment = Rc::new(RefCell::new(environment));
                let result = self.interpret((*statements)
                    .iter()
                    .map(|b| b)
                    .collect()
                );
                self.environment = old_environment;
                result?
            }
            #[allow(unused)]
            Stmt::Function { name, params, body } => todo!(),
        };

        Ok(())
    }    
}
