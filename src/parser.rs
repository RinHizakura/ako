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
            TokenType::TokenAnd => OpType::OpAnd,
            TokenType::TokenOr => OpType::OpOr,
            TokenType::TokenXor => OpType::OpXor,
            TokenType::TokenPlus => OpType::OpAdd,
            TokenType::TokenMinus => OpType::OpSub,
            TokenType::TokenAsterisk => OpType::OpMul,
            TokenType::TokenSlash => OpType::OpDiv,
            TokenType::TokenPercent => OpType::OpModulo,
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
                TokenType::TokenIdent => Some(Expression::ident(token.literal.clone())),
                _ => {
                    return Err(anyhow!(format!(
                        "Parser error: unexpected token {:?} in the expression",
                        token
                    )))
                }
            };
        }

        self.update_token();
        // FIXME: Clone for ownership model, any better approach?
        let cur_token = self.cur_token.clone();
        if let Some(token) = cur_token {
            left = match token.t {
                TokenType::TokenAssign => {
                    match left.as_ref().unwrap() {
                        Expression::Ident(_) => (),
                        _ => {
                            return Err(anyhow!(
                                "Parser error: Invalid left expression of assign operator"
                            ))
                        }
                    };

                    let right = self.parse_expression()?;
                    Some(Expression::assign(left, right))
                }
                TokenType::TokenAnd
                | TokenType::TokenOr
                | TokenType::TokenXor
                | TokenType::TokenPlus
                | TokenType::TokenMinus
                | TokenType::TokenAsterisk
                | TokenType::TokenSlash
                | TokenType::TokenPercent => {
                    let right = self.parse_expression()?;
                    Some(Expression::infix(Self::token_op(&token), left, right))
                }
                TokenType::TokenSemiColon => left,
                _ => {
                    return Err(anyhow!(format!(
                        "Parser error: unexpected token {:?} in the expression",
                        token
                    )))
                }
            };
        }

        Ok(left)
    }

    fn parse_expr_statement(&mut self) -> Result<Option<Statement>> {
        let expr = self.parse_expression()?;
        Ok(expr.map(|e| Statement::new(StmtType::Expr, e)))
    }

    fn parse_let_statement(&mut self) -> Result<Option<Statement>> {
        self.update_token();
        /* Peek the next token to see if is a valid let statement */
        if let Some(token) = &self.next_token {
            if !matches!(token.t, TokenType::TokenIdent) {
                return Err(anyhow!("Parser error: Invalid let statement"));
            }
        }

        let expr = self.parse_expression()?;
        Ok(expr.map(|e| Statement::new(StmtType::Let, e)))
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        /* Peek the next token */
        if let Some(token) = &self.next_token {
            return match token.t {
                TokenType::TokenLet => self.parse_let_statement(),
                _ => self.parse_expr_statement(),
            };
        }

        // There's no token has to be parsed
        Ok(None)
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
