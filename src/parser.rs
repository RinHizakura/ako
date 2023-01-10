use crate::lexer::Lexer;
use crate::stmt::*;
use crate::token::{Token, TokenType};
use anyhow::{anyhow, Result};

const PRECEDENCE_LOWEST: i8 = -1;
pub fn get_token_precedence(token_type: &TokenType) -> i8 {
    /* FIXME: Consider if a token could represent two different
     * operation(e.g. '+' can be used for unary plus or addition) */
    match token_type {
        TokenType::TokenAssign => 1,
        TokenType::TokenOr => 2,
        TokenType::TokenXor => 3,
        TokenType::TokenAnd => 4,
        TokenType::TokenPlus | TokenType::TokenMinus => 5,
        TokenType::TokenAsterisk | TokenType::TokenSlash | TokenType::TokenPercent => 6,
        TokenType::TokenLparen => 7,
        _ => PRECEDENCE_LOWEST,
    }
}

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

    fn parse_call_expression_list(&mut self) -> Result<Vec<Option<Expression>>> {
        let mut v = Vec::new();

        /* Handle the case when there's no parameters for the function call */
        if let Some(next_token) = &self.next_token {
            match &next_token.t {
                TokenType::TokenRparen => {
                    self.update_token();
                    return Ok(v);
                }
                _ => {}
            }
        }

        /* Otherwise, each argument should be an individual expression. The expression
         * is expected to end up with ',' or ')' */
        while let Some(token) = &self.cur_token {
            match &token.t {
                TokenType::TokenComma | TokenType::TokenLparen => {
                    v.push(self.parse_expression(PRECEDENCE_LOWEST)?);
                }
                TokenType::TokenRparen => {
                    /* If the current token is rparen, then this is the
                     * end of the call expression. */
                    break;
                }
                _ => {
                    return Err(anyhow!(format!(
                        "Parser error: unexpected token {:?} in the expression list",
                        token
                    )));
                }
            }
        }

        Ok(v)
    }

    fn parse_expression(&mut self, precedence: i8) -> Result<Option<Expression>> {
        let mut left = None;
        self.update_token();
        if let Some(token) = &self.cur_token {
            let token_type = &token.t;
            left = match token_type {
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

        while let Some(next_token) = &self.next_token {
            /* If the next token has lower precedence, the 'left' which we had
             * gotten previously should belong to the left sub-expression. */
            if get_token_precedence(&next_token.t) < precedence {
                break;
            }

            self.update_token();
            // FIXME: Clone for ownership model, any better approach?
            let cur_token = self.cur_token.clone();
            if let Some(token) = cur_token {
                let token_type = &token.t;
                match token_type {
                    TokenType::TokenAssign => {
                        if !matches!(left.as_ref().unwrap(), Expression::Ident(_)) {
                            return Err(anyhow!(
                                "Parser error: Invalid left expression of assign operator"
                            ));
                        }

                        let right = self.parse_expression(get_token_precedence(token_type))?;
                        left = Some(Expression::assign(left, right))
                    }
                    TokenType::TokenAnd
                    | TokenType::TokenOr
                    | TokenType::TokenXor
                    | TokenType::TokenPlus
                    | TokenType::TokenMinus
                    | TokenType::TokenAsterisk
                    | TokenType::TokenSlash
                    | TokenType::TokenPercent => {
                        let right = self.parse_expression(get_token_precedence(token_type))?;
                        left = Some(Expression::infix(Self::token_op(&token), left, right))
                    }
                    TokenType::TokenLparen => {
                        let right = self.parse_call_expression_list()?;
                        left = Some(Expression::call(left, right));
                    }
                    TokenType::TokenSemiColon | TokenType::TokenComma | TokenType::TokenRparen => {
                        /* TODO: Is it always reasonable to end an expression with
                         * these tokens? */
                        break;
                    }
                    _ => {
                        return Err(anyhow!(format!(
                            "Parser error: unexpected token {:?} in the expression",
                            token
                        )))
                    }
                };
            }
        }

        Ok(left)
    }

    fn parse_expr_statement(&mut self) -> Result<Option<Statement>> {
        let expr = self.parse_expression(PRECEDENCE_LOWEST)?;
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

        let expr = self.parse_expression(PRECEDENCE_LOWEST)?;
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
