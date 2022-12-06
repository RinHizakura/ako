use crate::lexer::Lexer;
use crate::stmt::{Expression, OpType, Statement};
use crate::token::{Token, TokenType};

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
        self.next_token = self.lexer.as_mut().unwrap().gettoken();
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

    pub fn parse_program(&mut self, program: String) -> Vec<Statement> {
        self.lexer = Some(Lexer::new(program));
        // Initialize the token cursor
        self.cur_token = None;
        self.next_token = self.lexer.as_mut().unwrap().gettoken();

        let mut statements = vec![];
        while let Some(s) = self.parse_statement() {
            println!("stmt {:?}", s);
            statements.push(s);
        }

        statements
    }
}

pub struct Evaluator {
    program: String,
}

impl Evaluator {
    pub fn new(program: String) -> Self {
        Evaluator { program }
    }

    pub fn compile(&mut self) {
        let mut parser = Parser::new();
        let v = parser.parse_program(self.program.clone());
    }
}
