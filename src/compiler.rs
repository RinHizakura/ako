use crate::stmt::*;
use anyhow::{anyhow, Result};

pub const OPCODE_CONST: u8 = 0;
pub const OPCODE_ADD: u8 = 1;

struct Instruction(Vec<u8>);
impl Instruction {
    fn operand_width(opcode: u8) -> usize {
        match opcode {
            OPCODE_CONST => 4,
            OPCODE_ADD => 0,
            _ => unreachable!(),
        }
    }

    pub fn new(opcode: u8, operands: &[i32]) -> Self {
        let operand_width = Self::operand_width(opcode);

        let mut inst_bytes = vec![opcode];
        for operand in operands {
            for i in 0..operand_width {
                inst_bytes.push((operand >> (8 * i)) as u32 as u8);
            }
        }

        Instruction(inst_bytes)
    }
}

pub struct Compiler {
    insts: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { insts: vec![] }
    }

    fn emit(&mut self, opcode: u8, operands: &[i32]) {
        let inst = Instruction::new(opcode, operands);
        self.insts.push(inst);
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

    fn compile_int_expr(&mut self, i: i32) -> Result<()> {
        // The operand will be constant value itself
        self.emit(OPCODE_CONST, &[i]);
        Ok(())
    }

    fn compile_expr(&mut self, expr: Expression) -> Result<()> {
        match expr {
            Expression::Int(i) => self.compile_int_expr(i),
            Expression::Infix(infix) => self.compile_infix_expr(infix),
        }
    }

    fn compile_statement(&mut self, stmt: Statement) -> Result<()> {
        // TODO: support more different statement type
        self.compile_expr(stmt.expr)
    }

    pub fn compile(&mut self, stmts: Vec<Statement>) -> Result<()> {
        for stmt in stmts {
            self.compile_statement(stmt)?;
        }

        Ok(())
    }
}
