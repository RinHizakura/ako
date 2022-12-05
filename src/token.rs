#[derive(Debug)]
pub enum Type {
    TokenPlus,
    TokenMinus,
    TokenAsterisk,
    TokenSlash,
    TokenInt,
    TokenUnknown,
}

#[derive(Debug)]
pub enum Literal {
    Operator(String),
    Int(usize),
    Unknown(()),
}

#[derive(Debug)]
pub struct Token {
    pub t: Type,
    pub literal: Literal,
}

impl Token {
    pub fn to_int(&self) -> usize {
        match self.literal {
            Literal::Int(u) => u,
            _ => panic!("A non-Int type Token cannot be transformed to integer"),
        }
    }

    pub fn plus() -> Self {
        Token {
            t: Type::TokenPlus,
            literal: Literal::Operator("+".to_string()),
        }
    }

    pub fn minus() -> Self {
        Token {
            t: Type::TokenMinus,
            literal: Literal::Operator("-".to_string()),
        }
    }

    pub fn asterisk() -> Self {
        Token {
            t: Type::TokenAsterisk,
            literal: Literal::Operator("*".to_string()),
        }
    }

    pub fn slash() -> Self {
        Token {
            t: Type::TokenSlash,
            literal: Literal::Operator("/".to_string()),
        }
    }

    pub fn int(n: usize) -> Self {
        Token {
            t: Type::TokenInt,
            literal: Literal::Int(n),
        }
    }

    pub fn unknown() -> Self {
        Token {
            t: Type::TokenUnknown,
            literal: Literal::Unknown(()),
        }
    }
}
