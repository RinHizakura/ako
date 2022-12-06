use crate::token::Token;

pub struct Lexer {
    program: String,
    pos: usize,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        Lexer {
            program: program,
            pos: 0,
        }
    }

    fn gettoken_int(&mut self, mut s: String) -> Option<Token> {
        while let Some(next_ch) = self.program.chars().nth(self.pos) {
            if next_ch.is_ascii_digit() {
                s.push(next_ch);
                self.pos += 1;
            } else {
                break;
            }
        }
        Some(Token::int(s))
    }

    pub fn gettoken(&mut self) -> Option<Token> {
        if let Some(ch) = self.program.chars().nth(self.pos) {
            self.pos += 1;

            return match ch {
                ' ' => self.gettoken(), // skip the space
                '+' => Some(Token::plus()),
                '-' => Some(Token::minus()),
                '*' => Some(Token::asterisk()),
                '/' => Some(Token::slash()),
                _ => {
                    if ch.is_ascii_digit() {
                        self.gettoken_int(ch.to_string())
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
