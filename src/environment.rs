use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{expr::LitValue, Token};

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    map: HashMap<String, LitValue>
}

impl Environment {
    pub fn new() -> Self {
        Self {map: HashMap::new(), enclosing: None}
    }

    pub fn define(&mut self, name: String, value: LitValue) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Option<LitValue>, String> {
        let value = self.map.get(name.get_lexeme());
        match (value, &self.enclosing) {
            (Some(v), _) => Ok(Some(v.clone())),
            (None, Some(env)) => env.borrow().get(name),
            (None, None) => Err(format!("Undefined Variable '{}'", name.get_lexeme()))
        }
    }

    pub fn assign(&mut self, name: &str, value: LitValue) -> Result<(), String> {
        let old_value = self.map.get(name);
        match (old_value, &self.enclosing) {
            (Some(_), _) => {
                self.map.insert(name.to_string(), value);
            },
            (None, Some(env)) => {
                env.borrow_mut().assign(name, value)?
            },
            (None, None) => return Err(format!("Undefined variable '{}'", name))
        };

        Ok(())
    }
}