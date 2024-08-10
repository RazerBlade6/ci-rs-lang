use std::collections::HashMap;

use crate::{expr::LitValue, Token};

#[derive(Clone, Debug)]
pub struct Environment {
    map: HashMap<String, LitValue>
}

impl Environment {
    pub fn new() -> Self {
        Self {map: HashMap::new()}
    }
    pub fn define(&mut self, name: String, value: LitValue) {
        self.map.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> Result<LitValue, String> {
        match self.map.get(name.get_lexeme()) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Undefined Variable '{}'", name.get_lexeme()))
        }
    }
}