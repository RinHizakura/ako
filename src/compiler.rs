use crate::opcode::*;
use crate::stmt::*;
use anyhow::{anyhow, Result};

pub struct Compiler {
    bytecode: Option<Vec<u8>>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { bytecode: None }
    }

    fn emit(&mut self, opcode: u8, operands: &[i32]) {
        let operand_width = operand_width(opcode);

        if let Some(ref mut bytecode) = self.bytecode {
            bytecode.push(opcode);
            for operand in operands {
                for i in 0..operand_width {
                    bytecode.push((operand >> (8 * i)) as u32 as u8);
                }
            }
        }
    }

    fn compile_infix_expr(&mut self, infix: InfixExpression) -> Result<()> {
        if let Some(left) = infix.left {
            self.compile_expr(*left)?;
        }
        if let Some(right) = infix.right {
            self.compile_expr(*right)?;
        }

        match infix.op {
            OpType::OpAdd => self.emit(OPCODE_ADD, &[]),
            _ => {
                return Err(anyhow!(
                    "Compile error: Unknown operator in the infix expression"
                ))
            }
        }

        Ok(())
    }

    fn compile_assign_expr(&mut self, assign: AssignExpression) -> Result<()> {
        /* Note: We had make sure the target is an ident expression at parser. */
        todo!();

        Ok(())
    }

    fn compile_int_expr(&mut self, i: i32) -> Result<()> {
        // The operand will be constant value itself
        self.emit(OPCODE_CONST, &[i]);
        Ok(())
    }

    fn compile_expr(&mut self, expr: Expression) -> Result<()> {
        match expr {
            Expression::Int(i) => self.compile_int_expr(i),
            Expression::Infix(infix) => self.compile_infix_expr(infix),
            Expression::Ident(i) => todo!(),
            Expression::Assign(assign) => self.compile_assign_expr(assign),
        }
    }

    fn compile_statement(&mut self, stmt: Statement) -> Result<()> {
        // TODO: support more different statement type
        self.compile_expr(stmt.expr)
    }

    pub fn compile(&mut self, stmts: Vec<Statement>) -> Result<Vec<u8>> {
        self.bytecode = Some(vec![]);
        for stmt in stmts {
            self.compile_statement(stmt)?;
        }

        let bytecode = self.bytecode.take().unwrap();
        Ok(bytecode)
    }
}
