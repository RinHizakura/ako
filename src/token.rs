#[derive(Debug, Clone)]
pub enum TokenType {
    TokenAnd,
    TokenOr,
    TokenXor,
    TokenPlus,
    TokenMinus,
    TokenAsterisk,
    TokenSlash,
    TokenPercent,
    TokenAssign,
    TokenEq,
    TokenSemiColon,
    TokenLet,
    TokenInt,
    TokenIdent,
    TokenUnknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub literal: String,
}

impl Token {
    pub fn and() -> Self {
        Token {
            t: TokenType::TokenAnd,
            literal: "&".to_string(),
        }
    }

    pub fn or() -> Self {
        Token {
            t: TokenType::TokenPlus,
            literal: "|".to_string(),
        }
    }

    pub fn xor() -> Self {
        Token {
            t: TokenType::TokenPlus,
            literal: "^".to_string(),
        }
    }

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

    pub fn percent() -> Self {
        Token {
            t: TokenType::TokenPercent,
            literal: "%".to_string(),
        }
    }

    pub fn assign() -> Self {
        Token {
            t: TokenType::TokenAssign,
            literal: "=".to_string(),
        }
    }

    pub fn eq() -> Self {
        Token {
            t: TokenType::TokenEq,
            literal: "==".to_string(),
        }
    }

    pub fn semicolon() -> Self {
        Token {
            t: TokenType::TokenSemiColon,
            literal: ";".to_string(),
        }
    }

    pub fn int(n: String) -> Self {
        Token {
            t: TokenType::TokenInt,
            literal: n,
        }
    }

    pub fn ident(n: String) -> Self {
        Token {
            t: TokenType::TokenIdent,
            literal: n,
        }
    }

    pub fn keyword_let() -> Self {
        Token {
            t: TokenType::TokenLet,
            literal: "let".to_string(),
        }
    }

    pub fn unknown(s: String) -> Self {
        Token {
            t: TokenType::TokenUnknown,
            literal: s,
        }
    }
}
