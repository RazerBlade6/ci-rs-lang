use crate::expr::LitValue;

pub trait Callable {
    #![allow(dead_code)]
    fn call(&mut self, arguments: Vec<LitValue>) -> Result<LitValue, String>;
    fn arity(&mut self) -> usize;
}

impl Callable for LitValue {
    fn call(&mut self, arguments: Vec<LitValue>) -> Result<LitValue, String> {
        let _ = arguments;
        match self {
            LitValue::Callable => (),
            _ => return Err(format!("Could not call {}, Can only call functions and classes", self.to_string()))
        }

        println!("Hello");
        
        todo!()
    }

    fn arity(&mut self) -> usize {
        todo!()
    }  
}