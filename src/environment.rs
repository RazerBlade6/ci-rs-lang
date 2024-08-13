use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expr::LitValue;

#[derive(Debug)]
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

    pub fn get(&self, name: String) -> Result<Option<LitValue>, String> {
        let value = self.map.get(&name);
        match (value, &self.enclosing) {
            (Some(literal), _) => Ok(Some(literal.clone())),
            (None, Some(env)) => env.borrow().get(name),
            (None, None) => Err(format!("Undefined Variable '{}'", name))
        }
    }

    pub fn assign(&mut self, name: &str, value: &LitValue) -> Result<(), String> {
        let old_value = self.map.get(name);
        match (old_value, &self.enclosing) {
            (Some(_), _) => {
                self.map.insert(name.to_string(), value.clone());
            },
            (None, Some(env)) => {
                env.borrow_mut().assign(name, value)?
            },
            (None, None) => return Err(format!("Undefined variable '{}'", name))
        };

        Ok(())
    }
}