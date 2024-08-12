use crate::stmt::Stmt;
use crate::{expr::*, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        let result = if self.match_tokens(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(statement) => Ok(statement),
            Err(msg) => {
                self.synchronise();
                Err(msg)
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name: Token = self.consume(TokenType::Identifier, "Expected variable name")?;
        let mut initializer: Option<Expr> = None;
        if self.match_tokens(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::SemiColon, "Expected ';' after variable declaration")?;
    
        Ok(Stmt::Var { name , initializer })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_tokens(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_tokens(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokens(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_tokens(&[TokenType::LeftBrace]) {
            return self.block();
        }
        self.expr_statement()
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition: Expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition")?;
        let body: Stmt = self.statement()?;

        return Ok(Stmt::While { condition, body: Box::from(body) })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition: Expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition")?;
        let then_branch: Stmt = self.statement()?;
        let mut else_branch: Option<Stmt> = None;
        if self.match_tokens(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }
        
        println!("Here!");
        Ok(Stmt::If { condition, then_branch: Box::from(then_branch), else_branch: Box::from(else_branch)})
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expr: Expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expected ';' after value")?;
        Ok(Stmt::Print { expr })
    }

    fn expr_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expected ';' after expression")?;
        Ok(Stmt::Expression { expr })
    }

    fn block(&mut self) -> Result<Stmt, String> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block")?;
        Ok(Stmt::Block{statements: Box::from(statements)})
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assigment()
    }

    fn assigment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assigment()?;

            let name = match &expr {
                Expr::Variable { name } => name.clone(),
                _ => return Err(format!("Invalid assignment target {}", equals.to_string()))
            };

            return Ok(Expr::create_assigment(name, value))
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        if self.match_tokens(&[TokenType::Or]) {
            let operator: Token = self.previous();
            let right: Expr = self.and()?;
            expr = Expr::create_logical(expr, operator, right)
        }
        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.equality()?;

        if self.match_tokens(&[TokenType::And]) {
            let operator: Token = self.previous();
            let right: Expr = self.equality()?;
            expr = Expr::create_logical(expr, operator, right);
        }
        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::create_binary(expr, operator, right);
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::create_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::create_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::create_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::create_unary(operator, right));
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
                return Ok(Expr::create_grouping(expr));
            }
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number
            | TokenType::String => {
                self.advance();
                return Ok(Expr::create_literal(LitValue::from_token(tok)));
            }
            TokenType::Identifier => {
                self.advance();
                return Ok(Expr::create_variable(self.previous()));
            }
            _ => return Err(String::from("Expected Expression")),
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

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().get_type() == TokenType::SemiColon {
                return;
            }

            match self.peek().get_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }
        }

        self.advance();
    }

    fn consume(&mut self, typ: TokenType, msg: &'static str) -> Result<Token, String> {
        let token = self.peek();
        if token.get_type() == typ {
            self.advance();
            Ok(token)
        } else {
            Err(format!("Line {}: {}", self.peek().get_line(), msg).to_string())
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn check(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

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
