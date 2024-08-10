use std::collections::HashMap;

use crate::{expr::LitValue, Token};

#[derive(Clone, Debug)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    map: HashMap<String, LitValue>
}

impl Environment {
    pub fn new() -> Self {
        Self {map: HashMap::new(), enclosing: None}
    }

    pub fn define(&mut self, name: String, value: LitValue) {
        self.map.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> Result<LitValue, String> {
        match (self.map.get(name.get_lexeme()), self.enclosing.clone()) {
            (Some(v), _) => Ok(v.clone()),
            (None, Some(mut e)) => (*e).get(name),
            (None, None) => Err(format!("Undefined Variable '{}'", name.get_lexeme()))
        }
    }

    pub fn assign(&mut self, name: Token, value: LitValue) -> Result<(), String> {
        match (self.map.get_mut(name.get_lexeme()), self.enclosing.clone()) {
            (Some(val), _) => {
                *val = value;
                Ok(())
            }
            (None, Some(mut e)) => (*e).assign(name, value),
            (None, None) => Err(format!("Undefined variable `{}`", name.get_lexeme()))
        }
    }
}

mod tests {
    #![allow(unused)]
    use super::*;
    use crate::TokenType::*;

    #[test]
    fn test_assign() {
        let mut environment: Environment = Environment::new();
        let token = Token::new(Identifier, "hello", crate::Literal::Str("Hello World".to_string()), 1);
        let token2 = Token::new(Identifier, "lit", crate::Literal::Str("Hello World".to_string()), 1);
        environment.define(token.get_lexeme().to_string(), LitValue::Str("Hello World".to_string()));

        assert_eq!(Ok(()), environment.assign(token, LitValue::Number(4.0)));
        assert_eq!(Err(format!("Undefined variable `{}`", token2.get_lexeme())), environment.assign(token2, LitValue::Number(4.0)));
    }
}