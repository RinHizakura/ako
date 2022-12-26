mod compiler;
mod lexer;
mod opcode;
mod parser;
mod stmt;
mod symtab;
mod token;
mod vm;

use crate::compiler::Compiler;
use crate::parser::Parser;
use crate::vm::Vm;
use anyhow::Result;

pub struct Ako {
    program: String,
}

impl Ako {
    pub fn new(program: String) -> Self {
        Ako { program }
    }

    pub fn compile(&mut self) -> Result<()> {
        let mut parser = Parser::new();
        let v = parser.parse_program(self.program.clone())?;

        let mut compiler = Compiler::new();
        let bytecode = compiler.compile(v)?;

        let mut vm = Vm::new();
        vm.run(bytecode);

        Ok(())
    }
}
