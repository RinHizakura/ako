#[derive(Debug, Clone)]
pub enum TokenType {
    TokenPlus,
    TokenMinus,
    TokenAsterisk,
    TokenSlash,
    TokenInt,
    TokenUnknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub literal: String,
}

impl Token {
    pub fn plus() -> Self {
        Token {
            t: TokenType::TokenPlus,
            literal: "+".to_string(),
        }
    }

    pub fn minus() -> Self {
        Token {
            t: TokenType::TokenMinus,
            literal: "-".to_string(),
        }
    }

    pub fn asterisk() -> Self {
        Token {
            t: TokenType::TokenAsterisk,
            literal: "*".to_string(),
        }
    }

    pub fn slash() -> Self {
        Token {
            t: TokenType::TokenSlash,
            literal: "/".to_string(),
        }
    }

    pub fn int(n: String) -> Self {
        Token {
            t: TokenType::TokenInt,
            literal: n,
        }
    }

    pub fn unknown() -> Self {
        Token {
            t: TokenType::TokenUnknown,
            literal: "".to_string(),
        }
    }
}
