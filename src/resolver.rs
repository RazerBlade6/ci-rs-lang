#![allow(dead_code)]
use std::collections::HashMap;
use crate::{expr::Expr, interpreter::Interpreter, stmt::Stmt, Token};

struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {interpreter, scopes: vec![]}
    }

    pub fn resolve(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: Stmt) -> Result<(), String> {
        match statement {
            Stmt::Expression { expr } => {
                self.resolve_expr(expr)?;
            },
            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(*then_branch)?;
                if let Some(s) = else_branch {
                    self.resolve_statement(*s)?;
                }
            },
            Stmt::Print { expr } => {
                self.resolve_expr(expr)?;
            },
            Stmt::Return { value } => {
                if let Some(v) = value {
                    self.resolve_expr(v)?;
                }
            },
            Stmt::Var { name, initializer } => {
                self.declare(&name);
                if let Some(e) = initializer {
                    self.resolve_expr(e)?;
                }
                self.define(&name);
            },
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_statement(*body)?;
            },
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            },
            Stmt::Function { name: _, params, body } => {
                self.begin_scope();
                for param in &params {
                    self.declare(param);
                    self.define(param);
                }

                self.resolve(body)?;
                self.end_scope();
            },
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }    

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        match self.scopes.last_mut() {
            Some(s) => s.insert(name.lexeme.clone(), false),
            None => return
        };
    }

    fn define(&mut self, name: &Token) {
        match self.scopes.last_mut() {
            Some(s) => s.insert(name.lexeme.clone(), true),
            None => return
        };
    }
  
    fn resolve_expr(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::Binary { left, operator: _, right } => {
                self.resolve_expr(*left)?;
                self.resolve_expr(*right)?;
            },
            Expr::Grouping { expr } => {
                self.resolve_expr(*expr)?;
            },
            Expr::Call { callee, paren: _, args } => {
                self.resolve_expr(*callee)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
            },
            Expr::Literal { literal: _ } => (),
            Expr::Logical { left, operator: _, right } => {
                self.resolve_expr(*left)?;
                self.resolve_expr(*right)?;
            },
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(*right)?;
            },
            Expr::Variable { name, index } =>  {
                if !self.scopes.is_empty() {
                    if let Some(false) = self.scopes.last().unwrap().get(&name.lexeme) {
                        return Err("Can't read local variable in its own initializer".to_string());
                    }
                }
                self.resolve_local(name, index);
            },
            Expr::Assignment { name: _, value } => {
                self.resolve_expr(*value)?;
            },
        }

        Ok(())
    }

    fn resolve_local(&mut self, name: Token, index: usize) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(index, self.scopes.len() - 1 - i);
                return;
            }
        }
    }
}