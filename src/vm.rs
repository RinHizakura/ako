use crate::cast;
use crate::opcode::*;

pub struct Vm {
    ip: usize,
    stack: Vec<Object>,
}

#[derive(Debug)]
enum Object {
    I(i32),
    Bool(bool),
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
    }

    fn get_operand(&self, bytecode: &Vec<u8>, opcode: u8) -> i32 {
        let operand_width = operand_width(opcode);
        let mut value = 0;
        for i in 0..operand_width {
            value |= (bytecode[self.ip + 1 + i] as i32) << (8 * i);
        }
        value
    }

    fn do_const(&mut self, bytecode: &Vec<u8>) {
        let value = self.get_operand(bytecode, OPCODE_CONST);
        self.stack.push(Object::I(value));
    }

    fn do_add(&mut self) {
        let right = self
            .stack
            .pop()
            .expect("Fail to pop right operand from stack");
        let left = self
            .stack
            .pop()
            .expect("Fail to pop left operand from stack");

        let right = cast!(right, Object::I);
        let left = cast!(left, Object::I);
        self.stack.push(Object::I(left + right));
    }

    fn do_set_local(&mut self, bytecode: &Vec<u8>) {
        let index = self.get_operand(bytecode, OPCODE_SET_LOCAL) as u32 as usize;
        todo!();
    }

    pub fn run(&mut self, bytecode: Vec<u8>) {
        let len = bytecode.len();
        self.reset();

        while self.ip < len {
            let opcode = bytecode[self.ip];
            let operand_width = operand_width(opcode);

            match opcode {
                OPCODE_CONST => self.do_const(&bytecode),
                OPCODE_ADD => self.do_add(),
                OPCODE_SET_LOCAL => self.do_set_local(&bytecode),
                _ => unreachable!(),
            }

            self.ip += 1 + operand_width;
        }

        println!("{:?}", self.stack);
    }
}
