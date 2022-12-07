mod compiler;
mod parser;
mod lexer;
mod stmt;
mod token;

use crate::compiler::Compiler;
use crate::parser::Parser;

pub struct Ako {
    program: String,
}

impl Ako {
    pub fn new(program: String) -> Self {
        Ako { program }
    }

    pub fn compile(&mut self) {
        let mut parser = Parser::new();
        let v = parser.parse_program(self.program.clone());

        let mut compiler = Compiler::new();
        compiler.compile(v);
    }
}
