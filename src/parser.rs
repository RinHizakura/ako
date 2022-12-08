use crate::lexer::Lexer;
use crate::stmt::*;
use crate::token::{Token, TokenType};
use anyhow::{anyhow, Result};

pub struct Parser {
    lexer: Option<Lexer>,
    cur_token: Option<Token>,
    next_token: Option<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            lexer: None,
            cur_token: None,
            next_token: None,
        }
    }

    fn token_op(token: &Token) -> OpType {
        match token.t {
            TokenType::TokenPlus => OpType::OpAdd,
            TokenType::TokenMinus => OpType::OpSubtract,
            TokenType::TokenAsterisk => OpType::OpMul,
            TokenType::TokenSlash => OpType::OpDiv,
            _ => OpType::OpUnknown,
        }
    }

    fn token_int(token: &Token) -> i32 {
        let chars = token.literal.chars();
        let mut n = 0;
        for c in chars {
            if let Some(digit) = c.to_digit(10) {
                n *= 10;
                n += digit as i32;
            } else {
                break;
            }
        }
        n
    }

    fn update_token(&mut self) {
        self.cur_token = self.next_token.take();
        self.next_token = self.lexer.as_mut().unwrap().gettoken();
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>> {
        let mut left = None;
        self.update_token();
        if let Some(token) = &self.cur_token {
            left = match token.t {
                TokenType::TokenInt => Some(Expression::int(Self::token_int(token))),
                _ => return Err(anyhow!("Parser error: unexpected token in the expression")),
            };
        }

        self.update_token();
        // FIXME: Clone for ownership model, any better approach?
        let cur_token = self.cur_token.clone();
        if let Some(token) = cur_token {
            left = match token.t {
                TokenType::TokenPlus => {
                    let right = self.parse_expression()?;
                    Some(Expression::infix(Self::token_op(&token), left, right))
                }
                _ => return Err(anyhow!("Parser error: unexpected token in the expression")),
            };
        }

        Ok(left)
    }

    fn parse_expr_statement(&mut self) -> Result<Option<Statement>> {
        let expr = self.parse_expression()?;
        Ok(expr.map(|e| Statement::new(e)))
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        // TODO: support more different statement type
        self.parse_expr_statement()
    }

    pub fn parse_program(&mut self, program: String) -> Result<Vec<Statement>> {
        self.lexer = Some(Lexer::new(program));
        // Initialize the token cursor
        self.cur_token = None;
        self.next_token = self.lexer.as_mut().unwrap().gettoken();

        let mut stmts = vec![];
        while let Some(s) = self.parse_statement()? {
            println!("stmt {:?}", s);
            stmts.push(s);
        }

        Ok(stmts)
    }
}
