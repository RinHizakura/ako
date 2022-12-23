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

    fn gettoken_ident(&mut self, mut s: String) -> Option<Token> {
        while let Some(next_ch) = self.program.chars().nth(self.pos) {
            if next_ch.is_ascii_alphanumeric() {
                s.push(next_ch);
                self.pos += 1;
            } else {
                break;
            }
        }

        // TODO: check specific keyword for the ident
        match s {
            _ => Some(Token::ident(s)),
        }
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
                '=' => {
                    if let Some(ch_next) = self.program.chars().nth(self.pos) {
                        if ch_next == '=' {
                            self.pos += 1;
                            return Some(Token::eq());
                        }
                    }

                    Some(Token::assign())
                }
                _ => {
                    if ch.is_ascii_digit() {
                        self.gettoken_int(ch.to_string())
                    } else if ch.is_ascii_alphabetic() {
                        self.gettoken_ident(ch.to_string())
                    } else {
                        Some(Token::unknown())
                    }
                }
            };
        }

        None
    }
}
