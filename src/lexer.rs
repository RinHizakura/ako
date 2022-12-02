use crate::token::Token;

pub struct Lexer {
    expr: String,
    pos: usize,
}

impl Lexer {
    pub fn new(expr: String) -> Self {
        Lexer { expr: expr, pos: 0 }
    }

    pub fn gettoken(&mut self) -> Option<Token> {
        if let Some(ch) = self.expr.chars().nth(self.pos) {
            self.pos += 1;
            return match ch {
                '+' => Some(Token::plus()),
                _ => Some(Token::unknown()),
            };
        }

        None
    }
}
