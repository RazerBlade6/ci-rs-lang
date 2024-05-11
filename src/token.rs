
#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,
  
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, String, Number,
  
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    Eof
}


impl TokenType {
    pub fn to_string(&self) -> &str {
        match self {
            TokenType::LeftParen =>    return "Left Parenthesis",
            TokenType::RightParen =>   return "Right Parenthesis",
            TokenType::LeftBrace =>    return "Left Brace",
            TokenType::RightBrace =>   return "Right Brace",
            TokenType::Comma =>        return "Comma",
            TokenType::Dot =>          return "Dot",
            TokenType::Minus =>        return "Minus",
            TokenType::Plus =>         return "Plus",
            TokenType::SemiColon =>    return "Semicolon",
            TokenType::Slash =>        return "Slash",
            TokenType::Star =>         return "Star",
            TokenType::Bang =>         return "Not",
            TokenType::BangEqual =>    return "Not Equal",
            TokenType::Equal =>        return "Assignment",
            TokenType::EqualEqual =>   return "Equals",
            TokenType::Greater =>      return "Greater",
            TokenType::GreaterEqual => return "Greater Equal",
            TokenType::Less =>         return "Less",
            TokenType::LessEqual =>    return "Less Equal",
            TokenType::Identifier =>   return "Identifier",
            TokenType::String =>       return "String",
            TokenType::Number =>       return "Number",
            TokenType::And =>          return "And",
            TokenType::Class =>        return "Class",
            TokenType::Else =>         return "Else",
            TokenType::False =>        return "False",
            TokenType::Fun =>          return "Fun",
            TokenType::For =>          return "For",
            TokenType::If =>           return "If",
            TokenType::Nil =>          return "Nil",
            TokenType::Or =>           return "Or",
            TokenType::Print =>        return "Print",
            TokenType::Return =>       return "Return",
            TokenType::Super =>        return "Super",
            TokenType::This =>         return "This",
            TokenType::True =>         return "True",
            TokenType::Var =>          return "Var",
            TokenType::While =>        return "While",
            TokenType::Eof =>          return "Eof",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Numeric(f64),
    Str(String),
    Id(String),
    Keyword(String),
    Null
}

impl Literal {
    pub fn to_string(&self) -> &str {
        match self {
            Literal::Numeric(_) => return "Numeric",
            Literal::Str(s)     => return s.as_str(),
            Literal::Id(_)      => return "Identifier",
            Literal::Keyword(s) => return s.as_str(),
            Literal::Null       => return "Null",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self {
        Self { token_type, lexeme: lexeme.to_string(), literal, line}
    }

    pub fn to_string(&self) -> String {
        let s = "Token => Type: ".to_owned() + self.token_type.to_string() + " Lexeme: " +
                        &self.lexeme + " Literal: " + self.literal.to_string() + " at Line: " + &self.line.to_string();
        
        s
    }

    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_literal(&self) ->  Literal {
        self.literal.clone()
    }
}