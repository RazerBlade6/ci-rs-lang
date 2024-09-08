use crate::{
    expr::Expr, 
    stmt::Stmt, 
    token::Token
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum FunctionType {
    Function,
}

#[derive(Debug)]
pub struct Resolver {
    pub locals: HashMap<usize, usize>,
    scopes: Vec<HashMap<String, bool>>,
    function_type: Option<FunctionType>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            scopes: vec![],
            function_type: None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<HashMap<usize, usize>, String> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }

        let locals = self.locals.clone();
        Ok(locals)
    }

    fn resolve_statement(&mut self, statement: &Stmt) -> Result<(), String> {
        match statement {
            Stmt::Expression { expr } => {
                self.resolve_expr(expr)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(s) = else_branch {
                    self.resolve_statement(s)?;
                }
            }
            Stmt::Return { value } => {
                if let None = self.function_type {
                    return Err(format!("Can't return from global scope"));
                } else if let Some(v) = value {
                    self.resolve_expr(v)?;
                }
            }
            Stmt::Var { name, initializer } => {
                self.declare(&name);
                if let Some(e) = initializer {
                    self.resolve_expr(e)?;
                }
                self.define(&name);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(body)?;
            }
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Stmt::Function { name, params, body } => {
                let parent_function = self.function_type.clone();
                self.function_type = Some(FunctionType::Function);
                self.declare(name);
                self.define(name);
                self.begin_scope();
                for param in params {
                    self.declare(param);
                    self.define(param);
                }

                self.resolve(body)?;
                self.end_scope();
                self.function_type = parent_function;
            }
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Grouping { expr } => {
                self.resolve_expr(expr)?;
            }
            Expr::Call {
                callee,
                paren: _,
                args,
            } => {
                self.resolve_expr(callee)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Literal { literal: _ } => (),
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(right)?;
            }
            Expr::Variable { name, index } => {
                if let Some(last) = self.scopes.last() {
                    if Some(&false) == last.get(&name.lexeme) {
                        return Err(format!(
                            "Tried to read local variable {} in own initialization",
                            name.lexeme
                        ));
                    }
                }
                self.resolve_local(name, *index);
            }
            Expr::Assignment { name, value, index } => {
                self.resolve_expr(value)?;
                self.resolve_local(name, *index);
            }
        }

        Ok(())
    }

    fn resolve_local(&mut self, name: &Token, index: usize) {
        let len = self.scopes.len();
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.locals.insert(index, len - i - 1);
                return;
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        match self.scopes.last_mut() {
            Some(scope) => scope.insert(name.lexeme.clone(), false),
            None => return,
        };
    }

    fn define(&mut self, name: &Token) {
        match self.scopes.last_mut() {
            Some(scope) => scope.insert(name.lexeme.clone(), true),
            None => return,
        };
    }
}
