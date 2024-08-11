use crate::expr::LitValue;
use crate::stmt::Stmt;
use crate::environment::Environment;

#[derive(Clone)]
pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Self {environment: Environment::new()}
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
            Stmt::Block { statements } => {
                let enclosing = Environment::new();
                self.environment.set_enclosing(enclosing.clone());
                match self.execute_block(*statements, self.environment.clone()) {
                    Ok(_) => (),
                    Err(msg) => println!("{msg}")
                };
            }
        };

        Ok(())
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) -> Result<(), String> {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => (),
                Err(_) =>{
                    self.environment = previous;
                    return Err("Block failed to execute fully".to_string())
                }
            };
        }
        self.environment = previous;
        Ok(())
    }

    
}
