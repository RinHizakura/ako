use crate::cast;
use crate::opcode::*;
use crate::stmt::*;
use crate::symtab::Symtab;
use anyhow::{anyhow, Result};

pub struct Compiler {
    bytecode: Option<Vec<u8>>,
    symtab: Symtab,
    /* If this flag is set when compiling the assign expression,
     * it means this is a let statement which allow new symbol
     * defined in symbol table. */
    let_flag: bool,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            bytecode: None,
            symtab: Symtab::new(),
            let_flag: false,
        }
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
        let name;
        if let Some(target) = assign.target {
            name = cast!(*target, Expression::Ident);
        } else {
            // This should never happen if we implement everything right
            panic!("Try to compile an invalid assign expression");
        }

        let s;
        if self.let_flag == true {
            s = self.symtab.define_var(name);
        } else {
            s = self.symtab.resolve(&name);
        }
        todo!();

        // Reset the flag to default value
        self.let_flag = false;
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
        match stmt.t {
            StmtType::Let => {
                self.let_flag = true;
                self.compile_expr(stmt.expr)
            }
            StmtType::Expr => self.compile_expr(stmt.expr),
        }
    }

    pub fn compile(&mut self, stmts: Vec<Statement>) -> Result<Vec<u8>> {
        self.bytecode = Some(vec![]);
        self.symtab.reset();

        for stmt in stmts {
            self.compile_statement(stmt)?;
        }

        let bytecode = self.bytecode.take().unwrap();
        Ok(bytecode)
    }
}
