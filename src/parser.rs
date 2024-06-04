use crate::{expr::*, Token, TokenType as Type};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens, current: 0}
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }
    
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[Type::BangEqual, Type::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[Type::Greater, Type::GreaterEqual, Type::Less, Type::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[Type::Minus, Type::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[Type::Slash, Type::Star]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[Type::Bang, Type::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::new_unary(operator, right);
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[Type::LeftParen]) {
            let expr = self.expression();
            self.consume(Type::RightParen, "Expected ')'");
            return Expr::new_grouping(expr)
        }
        
        let expr = Expr::new_literal(LitValue::from_token(self.peek()));
        self.advance();
        expr

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

    fn consume(&mut self, typ: Type, msg: &str) {
        if self.peek().get_type() == typ {self.advance();}
        else {panic!("{}", msg);}
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