//! # Tree-Walk Interpreter
//!
//! The Interpreter is the Execution block of the program, and is the final step in the runnning
//! of a Lox Program. A Tree-Walk Interpreter is an algorithm that relies on matching statement types
//! against the parsed statements, and executing them as matched.
//!
//! The biggest advantage of a Tree-Walk Interpreter is that it is very simple to implement, and the execution
//! mirrors the semantics of the language. The disadvantage is that it is extremely slow. As the tree must be
//! traversed with each call to execute, the program can rapidly consume stack memory and is REALLY slow to
//! run.
//!
//! If I cared enough, I might rewrite this with a better algorithm, either interpreting to assembly or to
//! some intermediate form like LLVM.
//!
//! The Interpreter's public API is exactly one constructor `new() -> Self` and two
//! methods: `resolve(locals: HashMap<usize, usize)` and `interpret(statements: Vec<&Stmt>)`. The `resolve()`
//! method is to be called first, and fed with the resolved statements from the resolver.
//!
//! `execute()` accepts the parsed statements, and executes them.
//!
//! ### Usage
//! `Interpreter` is created before `run()` is invoked, as it contains the Runtime Environment and
//! as such, must persist between consecutive invocations of `run()`
//!
//! ### Example
//! ```
//! use interpter::Interpter;
//!
//! fn main() {
//!     let interpreter = Interpreter::new();
//!     let src = "Example code";
//!     run(src, interpreter);
//! }
//! ```

use crate::{callable::Callables, environment::Environment, expr::Literal, stmt::Stmt};

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

    /// Interprets/executes the provided statements.
    pub fn interpret(&mut self, statements: Vec<&Stmt>) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), String> {
        match statement {
            Stmt::Expression { expr } => {
                expr.evaluate(&self.environment)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = condition.evaluate(&self.environment)?;

                match (condition.is_truthy(), else_branch) {
                    (true, _) => self.execute(&then_branch)?,
                    (false, Some(else_branch)) => self.execute(&else_branch)?,
                    (false, None) => return Ok(()),
                }
            }
            Stmt::Var { name, initializer } => {
                let value: Literal = match initializer {
                    Some(e) => e.evaluate(&self.environment)?,
                    None => Literal::Nil,
                };

                self.environment.define(name.lexeme.to_string(), value)
            }
            Stmt::While { condition, body } => {
                while condition.evaluate(&self.environment)?.is_truthy() {
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
                    Some(e) => Some(e.evaluate(&self.environment)?),
                    None => Some(Literal::Nil),
                };
            }
        };

        Ok(())
    }
}
