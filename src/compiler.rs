use crate::stmt::*;

pub type Opcode = u8;
pub const OpcodeNop: Opcode = 0;
pub const OpcodeAdd: Opcode = 1;

pub struct Compiler {
    dummy: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { dummy: 0 }
    }

    fn emit(&mut self, opcode: Opcode) {

    }

    fn compile_infix_expr(&mut self) {
        todo!();
    }

    fn compile_int_expr(&mut self) {
        todo!();
    }

    fn compile_expr(&mut self, expr: Expression) {
        match expr {
            Expression::Int(i) => self.compile_int_expr(),
            Expression::Infix(expr) => self.compile_infix_expr(),
            _ => todo!(),
        }
    }

    fn compile_statement(&mut self, stmt: Statement) {
        // TODO: support more different statement type
        self.compile_expr(stmt.expr);
    }

    pub fn compile(&mut self, stmts: Vec<Statement>) {
        for stmt in stmts {
            self.compile_statement(stmt);
        }
    }
}
