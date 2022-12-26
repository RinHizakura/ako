use crate::opcode::*;

pub struct Vm {
    ip: usize,
    stack: Vec<Object>,
}

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat));
        }
    }};
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

    fn do_const(&mut self, bytecode: &Vec<u8>) {
        let operand_width = operand_width(OPCODE_CONST);
        let mut value = 0;
        for i in 0..operand_width {
            value |= (bytecode[self.ip + 1 + i] as i32) << (8 * i);
        }
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

    pub fn run(&mut self, bytecode: Vec<u8>) {
        let len = bytecode.len();
        self.reset();

        while self.ip < len {
            let opcode = bytecode[self.ip];
            let operand_width = operand_width(opcode);

            match opcode {
                OPCODE_CONST => self.do_const(&bytecode),
                OPCODE_ADD => self.do_add(),
                _ => unreachable!(),
            }

            self.ip += 1 + operand_width;
        }

        println!("{:?}", self.stack);
    }
}
