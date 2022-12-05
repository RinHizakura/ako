use crate::lexer::Lexer;

pub struct Evaluator {
    expr: String,
}

impl Evaluator {
    pub fn new(expr: String) -> Self {
        Evaluator { expr: expr }
    }

    pub fn compile(&mut self) {
        let mut lexer = Lexer::new(self.expr.clone());
        let mut cur_token = lexer.gettoken();

        while let Some(token) = cur_token {
            println!("{:?}", token);
            cur_token = lexer.gettoken();
        }
    }
}
