mod compiler;
mod lexer;
mod opcode;
mod parser;
mod stmt;
mod symtab;
mod token;
mod vm;

#[macro_use]
mod macros;

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

    pub fn compile_and_run(&mut self) -> Result<()> {
        let mut parser = Parser::new();
        let v = parser.parse_program(self.program.clone())?;

        let mut compiler = Compiler::new();
        let bytecode = compiler.compile(v)?;

        let mut vm = Vm::new();
        vm.run(bytecode);

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let() {
        let program = "let a = 1;".to_string();

        let mut ako = Ako::new(program);
        let err = ako.compile_and_run();
        assert!(err.is_ok());
    }

    #[test]
    fn test_add() {
        let program = "let a = 1 + 1;".to_string();

        let mut ako = Ako::new(program);
        let err = ako.compile_and_run();
        assert!(err.is_ok());
    }

    #[test]
    fn test_get_ident() {
        let program = "let a = 1;\n \
                       let b = a;"
            .to_string();

        let mut ako = Ako::new(program);
        let err = ako.compile_and_run();
        assert!(err.is_ok());
    }
}
