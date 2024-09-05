use crate::expr::Literal;
use crate::token::*;
use crate::callable::Callables;
use std::rc::Rc;
use std::collections::HashMap;

pub fn globals() -> HashMap<String, Literal> {
    let mut globals = HashMap::new();
    let mut name = Token::new(TokenType::Fun, "clock",  0);
    globals.insert(
        "clock".to_string(),
        Literal::Callable(Callables::NativeFunction {
            name,
            arity: 0,
            fun: Rc::from(clock),
        })
    );
    name = Token::new(TokenType::Fun, "clear", 0);
    globals.insert(
        "clear".to_string(),
        Literal::Callable( Callables::NativeFunction {
            name,
            arity: 0,
            fun: Rc::from(clear),
        }),
    );
    name = Token::new(TokenType::Fun, "input", 0);
    globals.insert(
        "input".to_string(),
        Literal::Callable( Callables::NativeFunction {
            name,
            arity: 1,
            fun: Rc::from(input),
        }),
    );
    name = Token::new(TokenType::Fun, "parse", 0);
    globals.insert(
        "parse".to_string(),
        Literal::Callable(Callables::NativeFunction { 
            name, 
            arity: 2, 
            fun: Rc::from(parse) 
        }) 
    );

    globals
}

fn clock(_args: Vec<Literal>) -> Result<Literal, String> {
    use std::time::SystemTime;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => Ok(Literal::Number(d.as_secs_f64())),
        Err(msg) => Err(msg.to_string())
    }
}

fn clear(_args: Vec<Literal>) -> Result<Literal, String> {
    use std::{process::Command, env::consts::OS};

    match Command::new(match OS {
        "windows" => "powershell",
        "macos" => "terminal",
        "linux" => "sh",
        _ => return Err("Commands only implemented for windows, macos and linux".to_string()),
    })
    .arg("clear")
    .output() {
        Ok(_) => Ok(Literal::Nil),
        Err(msg) => Err(msg.to_string()),
    }
}

fn input(args: Vec<Literal>) -> Result<Literal, String> {
    use std::io::{stdin, stdout, Write};
    print!("{}", args[0].to_string());
    stdout().flush().unwrap();
    let mut buf = String::new();
    match stdin().read_line(&mut buf) {
        Ok(_) => Ok(Literal::Str(buf.trim().to_string())),
        Err(msg) => Err(msg.to_string())
    }
}

fn parse(args: Vec<Literal>) -> Result<Literal, String> {
    match (&args[0], &args[1]) {
        (Literal::Number(n), Literal::Str(typ)) => {
            match typ.to_lowercase().as_str() {
                "number" => return Ok(Literal::Number(*n)),
                "string" => return Ok(Literal::Str(n.to_string())),
                "boolean" => return Ok(Literal::Boolean(*n != 0.0)),
                "nil" => (),
                _ => return Err(format!("Type should be one of number, string, boolean, nil"))
            }
        },
        (Literal::Str(x), Literal::Str(typ)) => {
            match typ.to_lowercase().as_str() {
                "number" => {
                    match x.parse::<f64>() {
                        Ok(n) => return Ok(Literal::Number(n)),
                        Err(e) => return Err(e.to_string())
                    };
                },
                "string" => return Ok(Literal::Str(x.to_string())),
                "boolean" => {
                    match x.parse::<bool>() {
                        Ok(b) => return Ok(Literal::Boolean(b)),
                        Err(e) => return Err(e.to_string())
                    };
                },
                "nil" => (),
                _ => return Err(format!("Type should be one of number, string, boolean, nil"))
            }
        },
        (Literal::Boolean(b), Literal::Str(typ)) => {
            match typ.to_lowercase().as_str() {
                "number" => (),
                "string" => return Ok(Literal::Str(b.to_string())),
                "boolean" => return Ok(Literal::Boolean(*b)),
                "nil" => (),
                _ => return Err(format!("Type should be one of number, string, boolean, nil"))
            }
        },
        (Literal::Nil, Literal::Str(typ)) => {
            match typ.to_lowercase().as_str() {
                "number" => (),
                "string" => return Ok(Literal::Str(String::from("nil"))),
                "boolean" => (),
                "nil" => return Ok(Literal::Nil),
                _ => return Err(format!("Type should be one of number, string, boolean, nil"))
            }
        },
        _ => return Err(format!("Please use as parse(`variable`, `\"Type\"` (Number, String, Boolean, Nil))"))
    }
    
    Err(format!("Cannot parse {} to {}", args[0].to_type(), args[1].to_type()))
}