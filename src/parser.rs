use crate::token::*;
use crate::expr::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

#[derive(Debug)]
pub struct ParseError {
    msg: String,
    invalid: Token
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens, current: 0}
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(error) => Err(error) 
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError>  {
        match self.equality() {
            Ok(n) => Ok(n),
            Err(e) => Err(e)
        }
    }
    
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.comparison() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        while self.expect(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.peek(1);
            let right = match self.comparison() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.term() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
    
        while self.expect(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.peek(1);
            let right = match self.term() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.factor() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        while self.expect(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.peek(1);
            let right = match self.factor() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }
    
    fn expect(&mut self, tokens: &[TokenType]) -> bool {
        for ttype in tokens {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }
    
    fn check(&mut self, ttype: &TokenType) -> bool {
        if self.is_at_end() {return false;}
        return self.peek(0).get_type() == *ttype;
        
    }
    
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {self.current += 1}
        return self.peek(1);
    }
    
    fn is_at_end(&mut self) -> bool {
        self.peek(0).get_type() == TokenType::Eof
    }
    
    fn peek(&mut self, n: usize) -> Token {
        self.tokens.get(self.current - n).unwrap().to_owned()
    }
    
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = match self.unary() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        while self.expect(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.peek(1);
            let right = match self.unary() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.expect(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.peek(1);
            let right = match self.unary() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            return Ok(Expr::Unary { operator, right: Box::new(right)});
        }

        match self.primary() {
            Ok(n) => Ok(n),
            Err(e) => Err(e)
        }
    }
    
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.expect(&[TokenType::False]) {return Ok(Expr::Literal { literal: LitValue::False(false)});}
        if self.expect(&[TokenType::True]) {return Ok(Expr::Literal { literal: LitValue::True(true)});}
        if self.expect(&[TokenType::Nil]) {return Ok(Expr::Literal { literal: LitValue::Nil });}

        if self.expect(&[TokenType::Number]) {
            let temp = match self.peek(1).get_literal() {
                Literal::Numeric(n) => n,
                _ => return Err(ParseError::new("Invalid use of Number", match self.tokens.get(self.current) {
                    Some(&ref t) => t.clone(),
                    None => Token::new(TokenType::Eof, "", Literal::Null, self.current),
                }))
            };
            let literal = LitValue::Number(temp);
        }

        if self.expect(&[TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            self.eat(TokenType::RightParen, "Expect ')' after expression");
            return Ok(Expr::Grouping { expr: Box::new(expr) })
        } 

        Err(ParseError::new("Invalid Expression", match self.tokens.get(self.current) {
            Some(&ref t) => t.clone(),
            None => Token::new(TokenType::Eof, "", Literal::Null, self.current),
        }))
    }
    
    fn eat(&mut self, ttype: TokenType, msg: &str) -> Result<Token, ParseError>  {
        if self.check(&ttype) {
            return Ok(self.advance());
        } else {
            return Err(ParseError::new(msg, match self.tokens.get(self.current) {
                Some(&ref t) => t.clone(),
                None => Token::new(TokenType::Eof, "", Literal::Null, self.current),
            }))
        }
    }
}

impl ParseError {
    fn new(msg: &str, invalid: Token) -> Self {
        Self {msg: msg.to_string(), invalid }
    }

    fn report_parse_error(&self) {
        println!("[Parser] Error at Token: {}: {}", self.invalid.to_string(), self.msg)
    }
}

#[cfg(test)]
mod tests {
    use crate::Scanner;
    use crate::parser::*;

    #[test]
    fn test_parser() {
        let testvar = "-123 + (45.67)";
        let tokens = Scanner::new(&testvar).scan_tokens();

        for tok in tokens.clone() {
            println!("{}",tok.to_string());
        }

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(e) => println!("{}", e.to_string()),
            Err(er) => er.report_parse_error(),
        }        
    }
}
