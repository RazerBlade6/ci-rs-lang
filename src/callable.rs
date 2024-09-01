use std::{cell::RefCell, rc::Rc};
use crate::{environment::Environment, expr::Literal, Token, stmt::Stmt};

#[derive(Clone)]
pub enum Callables {
    #[allow(dead_code)]
    LoxFunction {
        name: Token,
        params: Vec<Token>,
        arity: usize,
        body: Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    },
    NativeFunction {
        name: Token,
        arity: usize,
        fun: Rc<dyn Fn(Vec<Literal>) -> Result<Literal, String>>
    }
}