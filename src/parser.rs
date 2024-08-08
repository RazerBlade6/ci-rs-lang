use crate::{
    expr::*, 
    Token, 
    TokenType as Type
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens, current: 0}
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[Type::BangEqual, Type::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_tokens(&[Type::Greater, Type::GreaterEqual, Type::Less, Type::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Type::Minus, Type::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[Type::Slash, Type::Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Type::Bang, Type::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let tok = self.peek();
        match tok.get_type() {
            Type::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                let _ = self.consume(Type::RightParen, "Expected `)`");
                return Ok(Expr::new_grouping(expr))
            },
            Type::False | Type::True | Type::Nil | Type::Number | Type::String => {
                self.advance();
                return Ok(Expr::new_literal(LitValue::from_token(tok)))
            },
            Type::Identifier => {
                self.advance();
                return Ok(Expr::new_literal(LitValue::from_token(tok)))
            }
            _ => return Err(String::from("Expected Expression"))
        }
    }

    fn match_tokens(&mut self, types: &[Type]) -> bool {
        for typ in types {
            if !self.check(typ.clone()) {
               continue; 
            }
            self.advance();
            return true;
        }

        false
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().get_type() == Type::SemiColon {return;}

            match self.peek().get_type() {
                Type::Class | Type::Fun | Type::Var | Type::For | Type::If | Type::While | Type::Print | Type::Return => return,
                _ => ()
            }
        }

        self.advance();
    }

    fn consume(&mut self, typ: Type, msg: &'static str) -> Result<Token, String> {
        let token = self.peek();
        if token.get_type() == typ {self.advance(); Ok(token)}
        else {
            Err(format!("Line {}: {}", self.peek().get_line(), msg).to_string())
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {self.current += 1}
        self.previous()
    }

    fn check(&mut self, typ: Type) -> bool {
        if self.is_at_end() {return false}

        self.peek().get_type() == typ
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().get_type() == Type::Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].to_owned()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].to_owned()
    }
}