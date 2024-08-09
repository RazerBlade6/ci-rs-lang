use crate::{ expr::*, Token, TokenType};
use crate::stmt::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens, current: 0}
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        while !self.is_at_end() {
            let statement = self.statement();
            match statement {
                Ok(statement) => statements.push(statement),
                Err(error) => errors.push(error)
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors.join("\n"))
        }
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_tokens(&[TokenType::Print]) {
            return Ok(self.print_statement()?);
        }

        Ok(self.expr_statement()?)
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expr: Expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expected ';' after value");
        Ok(Stmt::Print { expr })
    }

    fn expr_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expected ';' after expression");
        Ok(Stmt::Expression { expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let tok = self.peek();
        match tok.get_type() {
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                let _ = self.consume(TokenType::RightParen, "Expected `)`");
                return Ok(Expr::new_grouping(expr))
            },
            TokenType::False | TokenType::True | TokenType::Nil | TokenType::Number | TokenType::String => {
                self.advance();
                return Ok(Expr::new_literal(LitValue::from_token(tok)))
            },
            TokenType::Identifier => {
                self.advance();
                return Ok(Expr::new_literal(LitValue::from_token(tok)))
            }
            _ => return Err(String::from("Expected Expression"))
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if !self.check(typ.clone()) {
               continue; 
            }
            self.advance();
            return true;
        }

        false
    }
    #[allow(dead_code)]
    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().get_type() == TokenType::SemiColon {return;}

            match self.peek().get_type() {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => ()
            }
        }

        self.advance();
    }

    fn consume(&mut self, typ: TokenType, msg: &'static str) -> Result<Token, String> {
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

    fn check(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {return false}

        self.peek().get_type() == typ
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().get_type() == TokenType::Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].to_owned()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].to_owned()
    }
}