use crate::stmt::Stmt;

#[derive(Clone,)]
pub struct Interpreter {
    // Global State
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result <(), String> {
        for statement in statements {
            match statement {
                Stmt::Expression { expr } => {
                    expr.evaluate()?;
                },
                Stmt::Print { expr } => {
                    let result = expr.evaluate()?;
                    println!("{}", result.to_string());
                }
            };
        }

        return Ok(())
    } 
}