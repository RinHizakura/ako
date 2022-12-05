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
                '-' => Some(Token::minus()),
                '*' => Some(Token::asterisk()),
                '/' => Some(Token::slash()),
                _ => {
                    if let Some(n) = ch.to_digit(10) {
                        let mut n = n as usize;
                        while let Some(next_ch) = self.expr.chars().nth(self.pos) {
                            if let Some(digit) = next_ch.to_digit(10) {
                                n *= 10;
                                n += digit as usize;
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }
                        Some(Token::int(n))
                    } else {
                        // TODO: support variable name token
                        Some(Token::unknown())
                    }
                }
            };
        }

        None
    }
}
