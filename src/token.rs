#[derive(Debug)]
pub enum Type {
    TokenPlus,
    TokenUnknown,
}

pub struct Token {
    pub t: Type,
}

impl Token {
    pub fn plus() -> Self {
        Token { t: Type::TokenPlus }
    }

    pub fn unknown() -> Self {
        Token {
            t: Type::TokenUnknown,
        }
    }
}
