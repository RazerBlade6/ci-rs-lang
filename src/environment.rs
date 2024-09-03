use crate::{callable::Callables, expr::Literal, native::*, Token, TokenType};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: Rc<RefCell<HashMap<String, Literal>>>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: Rc::from(RefCell::from(Self::globals())),
            locals: Rc::from(RefCell::from(locals)),
            enclosing: None,
        }
    }

    fn globals() -> HashMap<String, Literal> {
        let mut globals = HashMap::new();
        let name = Token::new(TokenType::Fun, "clock",  0);
        globals.insert(
            "clock".to_string(),
            Literal::Callable(Callables::NativeFunction {
                name,
                arity: 0,
                fun: Rc::from(clock),
            })
        );
        globals.insert(
            "clear".to_string(),
            Literal::Callable( Callables::NativeFunction {
                name: Token::new(TokenType::Fun, "clear", 0),
                arity: 0,
                fun: Rc::from(clear),
            }),
        );

        globals
    }

    pub fn resolve(&mut self, index: usize, distance: usize) {
        self.locals.borrow_mut().insert(index, distance);
    }

    pub fn enclose(&self) -> Environment {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str, index: usize) -> Result<Literal, String> {
        let distance = self.locals.borrow().get(&index).cloned();
        match self.get_internal(name, distance) {
            Some(literal) => Ok(literal),
            None => Err(format!("Undefined variable {}", name))
        }
    }

    fn get_internal(&self, name: &str, distance: Option<usize>) -> Option<Literal> {
        if let None = distance {
            match &self.enclosing {
                None => self.values.borrow().get(name).cloned(),
                Some(env) => env.get_internal(name, distance),
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow().get(name).cloned()
            } else {
                match &self.enclosing {
                    None => panic!("Tried to resolve a variable that was defined deeper than the current environment depth"),
                    Some(env) => {
                        assert!(distance > 0);
                        env.get_internal(name, Some(distance - 1))
                    }
                }
            }
        }
    }

    pub fn assign(&self, name: &str, value: Literal, index: usize) -> Result<(), String> {
        let distance = self.locals.borrow().get(&index).cloned();
        self.assign_at_distance(name, value, distance)
    }

    fn assign_at_distance(&self, name: &str, value: Literal, distance: Option<usize>) -> Result<(), String> {
        if let Some(distance) = distance {
            if distance == 0 {
                self.values.borrow_mut().insert(name.to_string(), value);
                return Ok(())
            } else {
                match &self.enclosing {
                    None => panic!("Tried to define a variable deeper than max depth"),
                    Some(env) => return env.assign_at_distance(name, value, Some(distance - 1))
                }
            }
        } else {
            match &self.enclosing {
                Some(env) => env.assign_at_distance(name, value, distance)?,
                None => match self.values.borrow_mut().insert(name.to_string(), value) {
                    Some(_) => return Ok(()),
                    None => return Err(format!("Undefined Variable '{}'", name))
                }
            };
            return Ok(())
        }
    }
}
