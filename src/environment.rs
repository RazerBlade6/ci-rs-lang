use crate::expr::Literal;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    map: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enclose(&self) -> Self {
        Self { 
            enclosing: Some(Rc::new(RefCell::new(self.clone()))), 
            map: self.map.clone() 
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Literal, String> {
        let value = self.map.get(&name);

        match (value, &self.enclosing) {
            (Some(literal), _) => Ok(literal.clone()),
            (None, Some(env)) => env.borrow().get(name),
            (None, None) => Err(format!("Undefined Variable '{}'", name)),
        }
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<(), String> {
        let old_value = self.map.get(name);
        match (old_value, &self.enclosing) {
            (Some(_), _) => {
                self.map.insert(name.to_string(), value);
                Ok(())
            }
            (None, Some(env)) => (env.borrow_mut()).assign(name, value),
            (None, None) => Err(format!("Undefined Variable {}", name)),
        }
    }
}
