#![allow(unused)]

use crate::expr::*;
use crate::expr::Expr::*;

struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visit_literal_expr(&self, lit: Literal) -> LitValue {
        match lit {
            Literal::Numeric(a) => todo!(),
            Literal::Str(a) => todo!(),
            Literal::Id(a) => todo!(),
            Literal::Keyword(a) => todo!(),
            Literal::Null => todo!(),
        }
    }

    pub fn visit_grouping_expr(&self, expr: Expr::Grouping) {
        self.evaluate
    }
}