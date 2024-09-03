use std::{cell::RefCell, rc::Rc, fmt::Debug};
use crate::{environment::Environment, expr::Literal, Token, stmt::Stmt};

#[derive(Clone)]
pub enum Callables {
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

impl Debug for Callables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoxFunction {
                name,
                params: _,
                arity: _,
                body: _,
                environment: _,
            } => f.debug_struct("<function>").field("name", name).finish(),
            Self::NativeFunction {
                name,
                arity: _,
                fun: _,
            } => f.debug_struct("<native>").field("name", name).finish(),
        }
    }
}

impl PartialEq for Callables {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::LoxFunction {
                    name: l_name,
                    params: _,
                    arity: l_arity,
                    body: _,
                    environment: _,
                },
                Self::LoxFunction {
                    name: r_name,
                    params: _,
                    arity: r_arity,
                    body: _,
                    environment: _,
                },
            ) => l_name == r_name && l_arity == r_arity,
            (
                Self::NativeFunction {
                    name: l_name,
                    arity: l_arity,
                    fun: _,
                },
                Self::NativeFunction {
                    name: r_name,
                    arity: r_arity,
                    fun: _,
                },
            ) => l_name == r_name && l_arity == r_arity,
            _ => false,
        }
    }
}