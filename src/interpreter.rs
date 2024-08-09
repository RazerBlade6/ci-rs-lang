use crate::expr::{Expr, LitValue};

#[derive(Clone,)]
pub struct Interpreter {
    // Global State
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LitValue, String> {
        expr.evaluate()
    }
}