use crate::opcode::*;

pub struct Vm {
    ip: usize,
    stack: Vec<Object>,
}

enum Object {
    I(i32),
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

    fn do_const(&mut self, bytecode: &Vec<u8>) {
        let operand_width = operand_width(OPCODE_CONST);
        let mut value = 0;
        for i in 0..operand_width {
            value |= (bytecode[self.ip + 1 + i] as i32) << (8 * i);
        }
        self.stack.push(Object::I(value));

        self.ip += 1 + operand_width;
    }

    pub fn run(&mut self, bytecode: Vec<u8>) {
        let len = bytecode.len();
        self.reset();

        while self.ip < len {
            let opcode = bytecode[self.ip];
            match opcode {
                OPCODE_CONST => self.do_const(&bytecode),
                OPCODE_ADD => todo!(),
                _ => unreachable!(),
            }
        }
        todo!()
    }
}
