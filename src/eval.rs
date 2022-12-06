use crate::lexer::Lexer;
use crate::stmt::{Expression, OpType, Statement};
use crate::token::{Token, TokenType};

pub struct Evaluator {
    statement: String,
    lexer: Lexer,
    cur_token: Option<Token>,
    next_token: Option<Token>,
}

impl Evaluator {
    pub fn new(statement: String) -> Self {
        Evaluator {
            statement: statement.clone(),
            lexer: Lexer::new(statement),
            cur_token: None,
            next_token: None,
        }
    }

    fn token_op(token: &Token) -> OpType {
        match token.t {
            TokenType::TokenPlus => OpType::OpPlus,
            TokenType::TokenMinus => OpType::OpMinus,
            TokenType::TokenAsterisk => OpType::OpMul,
            TokenType::TokenSlash => OpType::OpDiv,
            _ => OpType::OpUnknown,
        }
    }

    fn token_int(token: &Token) -> usize {
        let chars = token.literal.chars();
        let mut n = 0;
        for c in chars {
            if let Some(digit) = c.to_digit(10) {
                n *= 10;
                n += digit as usize;
            } else {
                break;
            }
        }
        n
    }

    fn update_token(&mut self) {
        self.cur_token = self.next_token.take();
        self.next_token = self.lexer.gettoken();
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let mut left = None;
        self.update_token();
        if let Some(token) = &self.cur_token {
            left = match token.t {
                TokenType::TokenInt => Some(Expression::int(Self::token_int(token))),
                _ => todo!(),
            };
        }

        self.update_token();
        // FIXME: Clone for ownership model, any better approach?
        let cur_token = self.cur_token.clone();
        if let Some(token) = cur_token {
            left = match token.t {
                TokenType::TokenPlus => {
                    let right = self.parse_expression();
                    Some(Expression::infix(Self::token_op(&token), left, right))
                }
                _ => todo!(),
            };
        }

        left
    }

    fn parse_expr_statement(&mut self) -> Option<Statement> {
        if let Some(expr) = self.parse_expression() {
            Some(Statement::new(expr))
        } else {
            None
        }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        // TODO: support more different statement type
        self.parse_expr_statement()
    }

    pub fn compile(&mut self) {
        // Initialize the token cursor
        self.next_token = self.lexer.gettoken();

        while let Some(s) = self.parse_statement() {
            println!("stmt {:?}", s);
        }
    }
}
