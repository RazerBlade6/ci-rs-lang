use crate::callable::Callables;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    Str(String),
    Boolean(bool),
    Nil,
    Callable(Callables),
}
use {Callables::*, Literal::*};

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Number(n) => return format!("{:.5}", n),
            Str(s) => return s.to_string(),
            Boolean(b) => return format!("{b}"),
            Nil => return String::from("nil"),
            Self::Callable(other) => other.to_string(),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            Number(_) => return "Number",
            Str(_) => return "String",
            Boolean(_) => return "Boolean",
            Nil => return "nil",
            Callable(LoxFunction {
                name: _,
                params: _,
                arity: _,
                body: _,
                environment: _,
            }) => return "<function>",
            Callable(NativeFunction {
                name: _,
                arity: _,
                fun: _,
            }) => return "<native function>",
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(match token.lexeme.parse::<f64>() {
                Ok(f) => f,
                Err(_) => panic!("Invalid Syntax: attempted to parse non-numeric as f64"),
            }),
            TokenType::String => Self::Str(token.lexeme.to_string()),
            TokenType::Identifier => Self::Str(token.lexeme.to_string()),
            TokenType::True => Self::Boolean(true),
            TokenType::False => Self::Boolean(false),
            TokenType::Nil => Nil,
            other => panic!(
                "Invalid Syntax: Attempted extracting literal alue from non-valued type {}",
                other.to_string()
            ),
        }
    }

    pub fn is_falsy(&self) -> Literal {
        match self {
            Number(x) => Boolean(*x == 0.0),
            Str(s) => Boolean(s.len() == 0),
            Boolean(b) => Boolean(*b),
            Nil => Boolean(true),
            Callable(_) => Boolean(false),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Number(x) => return *x != 0.0,
            Str(s) => return s.len() != 0,
            Boolean(b) => return *b,
            Nil => return false,
            Callable(_) => return true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        args: Vec<Expr>,
    },
    Literal {
        literal: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        index: usize,
        name: Token,
    },
    Assignment {
        name: Token,
        value: Box<Expr>,
        index: usize,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "{} {} {}",
                    operator.lexeme,
                    (*left).to_string(),
                    (*right).to_string()
                )
            }
            Expr::Grouping { expr } => {
                format!("({})", (*expr).to_string())
            }
            Expr::Literal { literal } => {
                format!("{}", literal.to_string())
            }
            Expr::Unary { operator, right } => {
                format!("{}{}", operator.lexeme, (*right).to_string())
            }
            Expr::Variable { index: _, name } => name.lexeme.to_string(),
            Expr::Assignment {
                name,
                value,
                index: _,
            } => {
                format!("{} = {}", name.lexeme, (*value).to_string())
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                format!(
                    "`{}` {} `{}`",
                    (*left).to_string(),
                    operator.lexeme,
                    (*right).to_string()
                )
            }
            Expr::Call {
                callee: _,
                paren,
                args: _,
            } => {
                format!("function {}()", paren.lexeme)
            }
        }
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::from(left),
            operator,
            right: Box::from(right),
        }
    }

    pub fn new_grouping(expr: Expr) -> Self {
        Self::Grouping {
            expr: Box::from(expr),
        }
    }

    pub fn new_literal(literal: Literal) -> Self {
        Self::Literal { literal }
    }

    pub fn new_logical(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Logical {
            left: Box::from(left),
            operator,
            right: Box::from(right),
        }
    }

    pub fn create_unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::from(right),
        }
    }

    pub fn create_variable(name: Token, index: usize) -> Self {
        Self::Variable { index, name }
    }

    pub fn create_assigment(name: Token, value: Expr, index: usize) -> Self {
        Self::Assignment {
            name,
            value: Box::from(value),
            index,
        }
    }

    pub fn create_call(callee: Expr, paren: Token, args: Vec<Expr>) -> Self {
        Self::Call {
            callee: Box::from(callee),
            paren,
            args,
        }
    }

    pub fn evaluate(&self, environment: Environment) -> Result<Literal, String> {
        match &self {
            Expr::Literal { literal } => Ok((*literal).clone()),

            Expr::Grouping { expr } => (*expr).evaluate(environment),

            Expr::Unary { operator, right } => Self::evaluate_unary(environment, operator, right),

            Expr::Binary {
                left,
                operator,
                right,
            } => Self::evaluate_binary(environment, left, operator, right),

            Expr::Variable { name, index } => environment.get(&name.lexeme, *index),

            Expr::Assignment { name, value, index } => {
                Self::evaluate_assignment(environment, name, value, *index)
            }

            Expr::Logical {
                left,
                operator,
                right,
            } => Self::evaluate_logical(environment, left, operator, right),

            Expr::Call {
                callee,
                paren,
                args,
            } => Self::evaluate_call(environment, callee, paren, args),
        }
    }

    fn evaluate_unary(
        environment: Environment,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Literal, String> {
        let right = (*right).evaluate(environment)?;

        match (operator.token_type, &right) {
            (TokenType::Minus, Number(x)) => return Ok(Number(-x)),
            (TokenType::Minus, _) => {
                return Err(format!("negation not implemented for {}", right.to_type()))
            }
            (TokenType::Bang, any) => Ok(any.is_falsy()),
            _ => panic!("Invalid syntax: Parser specified non-unary expression as unary!"),
        }
    }

    fn evaluate_binary(
        environment: Environment,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Literal, String> {
        let left = (*left).evaluate(environment.clone())?;
        let right = (*right).evaluate(environment)?;

        match (&left, operator.token_type, &right) {
            (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),

            (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),

            (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),

            (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),

            (Number(x), TokenType::Percent, Number(y)) => Ok(Number(x % y)),

            (Str(s1), TokenType::Plus, Str(s2)) => Ok(Str(s1.clone() + s2)),

            (Number(x), TokenType::Greater, Number(y)) => Ok(Boolean(x > y)),

            (Number(x), TokenType::GreaterEqual, Number(y)) => Ok(Boolean(x >= y)),

            (Number(x), TokenType::Less, Number(y)) => Ok(Boolean(x < y)),

            (Number(x), TokenType::LessEqual, Number(y)) => Ok(Boolean(x <= y)),

            (x, TokenType::EqualEqual, y) => Ok(Boolean(x == y)),

            (x, TokenType::BangEqual, y) => Ok(Boolean(x != y)),

            (Str(s), TokenType::Plus, other) => Ok(Str(s.to_owned() + &other.to_string())),

            (some, TokenType::Plus, Str(s)) => Ok(Str(some.to_string() + s)),
            
            _ => Err(format!(
                "{} not implemented between {} and {}",
                operator.lexeme,
                left.to_type(),
                right.to_type()
            )),
        }
    }

    fn evaluate_assignment(
        environment: Environment,
        name: &Token,
        value: &Expr,
        index: usize,
    ) -> Result<Literal, String> {
        let value = (*value).evaluate(environment.clone())?;
        environment.assign(&name.lexeme, value.clone(), index)?;
        Ok(value)
    }

    fn evaluate_logical(
        environment: Environment,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, String> {
        let left: Literal = left.evaluate(environment.clone())?;
        if operator.token_type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else {
            if !left.is_truthy() {
                return Ok(left);
            }
        }

        return right.evaluate(environment);
    }

    fn evaluate_call(
        environment: Environment,
        callee: &Expr,
        _paren: &Token,
        args: &[Expr],
    ) -> Result<Literal, String> {
        let callee = (*callee).evaluate(environment.clone())?;

        let mut arguments = vec![];
        for arg in args {
            arguments.push(arg.evaluate(environment.clone())?);
        }

        match callee {
            Callable(callable) => match callable {
                LoxFunction {
                    name,
                    params,
                    arity,
                    body,
                    environment,
                } => return Self::call_function(name, params, arity, body, environment, arguments),
                NativeFunction { name, arity, fun } => {
                    return Self::call_native(name, arity, fun, arguments)
                }
            },
            other => return Err(format!("Could not call {}", other.to_type())),
        }
    }

    fn call_function(
        name: Token,
        params: Vec<Token>,
        arity: usize,
        body: Vec<Stmt>,
        environment: Environment,
        arguments: Vec<Literal>,
    ) -> Result<Literal, String> {
        if arguments.len() != arity {
            return Err(format!(
                "Function {} expected {} arguments but got {}",
                name.lexeme,
                arity,
                arguments.len()
            ));
        }

        let mut environment = environment.enclose();

        for (index, value) in arguments.iter().enumerate() {
            environment.define(params[index].lexeme.clone(), value.clone());
        }

        let mut interpreter = Interpreter::new_with_env(environment);

        for statement in &body {
            let result = interpreter.interpret(vec![statement]);
            if let Err(e) = result {
                return Err(e);
            }
            if let Some(ret) = interpreter.ret.clone() {
                return Ok(ret);
            }
        }
        Ok(Literal::Nil)
    }

    fn call_native(
        name: Token,
        arity: usize,
        fun: Rc<dyn Fn(Vec<Literal>) -> Result<Literal, String>>,
        arguments: Vec<Literal>,
    ) -> Result<Literal, String> {
        if arguments.len() != arity {
            return Err(format!(
                "Native function {} expected {} arguments but got {}",
                name.lexeme,
                arity,
                arguments.len()
            ));
        }
        Ok(fun(arguments)?)
    }
}
