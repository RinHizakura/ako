use crate::token::Token;

pub struct Lexer {
    statement: String,
    pos: usize,
}

impl Lexer {
    pub fn new(statement: String) -> Self {
        Lexer {
            statement: statement,
            pos: 0,
        }
    }

    fn gettoken_int(&mut self, mut n: usize) -> Option<Token> {
        while let Some(next_ch) = self.statement.chars().nth(self.pos) {
            if let Some(digit) = next_ch.to_digit(10) {
                n *= 10;
                n += digit as usize;
                self.pos += 1;
            } else {
                break;
            }
        }
        Some(Token::int(n))
    }

    pub fn gettoken(&mut self) -> Option<Token> {
        if let Some(ch) = self.statement.chars().nth(self.pos) {
            self.pos += 1;

            return match ch {
                ' ' => self.gettoken(), // skip the space
                '+' => Some(Token::plus()),
                '-' => Some(Token::minus()),
                '*' => Some(Token::asterisk()),
                '/' => Some(Token::slash()),
                _ => {
                    if let Some(n) = ch.to_digit(10) {
                        self.gettoken_int(n as usize)
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
