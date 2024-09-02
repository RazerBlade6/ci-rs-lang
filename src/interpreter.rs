use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::environment::Environment;
use crate::expr::Literal;
use crate::native::*;
use crate::stmt::Stmt;
use crate::token::*;
use crate::callable::Callables;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    pub locals: HashMap<usize, usize>,
    pub ret: Option<Literal>
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        let name = Token::new(TokenType::Fun, "clock",  0);
        globals.define(
            "clock".to_string(),
            Literal::Callable(Callables::NativeFunction {
                name,
                arity: 0,
                fun: Rc::from(clock),
            })
        );
        globals.define(
            "clear".to_string(),
            Literal::Callable( Callables::NativeFunction {
                name: Token::new(TokenType::Fun, "clear", 0),
                arity: 0,
                fun: Rc::from(clear),
            }),
        );
        Self {
            locals: HashMap::new(),
            environment: Rc::new(RefCell::new(globals)),
            ret: None
        }
    }

    pub fn new_with_env(environment: Environment) -> Self {
        Self { environment: Rc::new(RefCell::new(environment)), ret: None, locals: HashMap::new() }
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

            Stmt::Print { expr } => {
                let result = expr.evaluate(self.environment.clone())?;
                println!("{}", result.to_string());
            }
            Stmt::Var { name, initializer } => {
                let value: Literal = match initializer {
                    Some(e) => e.evaluate(self.environment.clone())?,
                    None => Literal::Nil
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.to_string(), value)
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(self.environment.clone())?.is_truthy() {
                    self.execute(&body)?;
                }
            }
            Stmt::Block { statements } => {
                let mut environment = Environment::new();
                environment.enclosing = Some(self.environment.clone());

                // let environment = self.environment.borrow_mut().enclose();
                let old_environment = self.environment.clone();
                self.environment = Rc::new(RefCell::new(environment));
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
                    environment: self.environment.clone() 
                };

                self.environment.borrow_mut().define(name.lexeme.to_string(), Literal::Callable(value));
            },
            Stmt::Return { value } => {
                self.ret = match value {
                    Some(e) => Some(e.evaluate(self.environment.clone())?),
                    None => Some(Literal::Nil)
                };
            }   
        };

        Ok(())
    }

    pub fn resolve(&mut self, index: usize, depth: usize) {
        self.locals.insert(index, depth);
        
    }
}
