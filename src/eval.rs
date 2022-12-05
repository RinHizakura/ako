use crate::lexer::Lexer;
use crate::token::{Token, Type};

enum Expression {
    Int(usize),
}

struct Statement {
    expression: Expression,
}

impl Statement {
    pub fn new(expression: Expression) -> Self {
        Statement { expression }
    }
}

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

    fn update_token(&mut self) {
        self.cur_token = self.next_token.take();
        self.next_token = self.lexer.gettoken();
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        let left;
        if let Some(token) = &self.cur_token {
            left = match token.t {
                Type::TokenInt => Some(Expression::Int(token.to_int())),
                _ => todo!(),
            };
        }

        if let Some(token) = &self.next_token {
            left = match token.t {
                Type::TokenPlus => todo!(),
                _ => unreachable!(),
            };
        }

        None
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
        self.next_token = self.lexer.gettoken();
        self.update_token();

        while let Some(s) = self.parse_statement() {}
    }
}
