//! # Callables
//!
//! Since I couldn't figure out a more elegant solution, this module exists solely
//! as a store for Callable Literals, as they are quite large in comparison to other
//! Literals. It also implements `Debug` and `PartialEq` on callables, as `#[derive()]` macros
//! cannot generate them.
//!
//! This enum only exists within `expr::Literal`, all details for Usage are found there.

use crate::{environment::Environment, expr::Literal, stmt::Stmt, Token};
use std::{fmt::Debug, rc::Rc};

#[derive(Clone)]
pub enum Callables {
    LoxFunction {
        name: Token,
        params: Vec<Token>,
        arity: usize,
        body: Vec<Stmt>,
        environment: Environment,
    },
    NativeFunction {
        name: String,
        arity: usize,
        fun: Rc<dyn Fn(Vec<Literal>) -> Result<Literal, String>>,
    },
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

impl Callables {
    pub fn to_string(&self) -> String {
        match self {
            Callables::LoxFunction {
                name,
                params: _,
                arity: _,
                body: _,
                environment: _,
            } => format!("<function> {}", name.lexeme),
            Callables::NativeFunction {
                name,
                arity: _,
                fun: _,
            } => format!("<native function> {name}"),
        }
    }
}
