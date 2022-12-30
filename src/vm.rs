use crate::cast;
use crate::opcode::*;
use std::fmt;

struct StackFrame {
    mem: Vec<Option<Object>>,
    cap: usize,
    stack_ptr: usize,
}

impl fmt::Debug for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /* TODO: maybe reimplement this to prettier format */
        write!(f, "StackFrame [");
        for i in 0..self.stack_ptr {
            write!(f, "{:?}, ", self.mem[i]);
        }
        write!(f, "]")
    }
}

impl StackFrame {
    pub fn new() -> Self {
        let cap = 128;
        let mut vec = Vec::with_capacity(cap);
        unsafe { vec.set_len(cap) }
        StackFrame {
            mem: vec,
            cap: cap,
            stack_ptr: 0,
        }
    }

    pub fn resize(&mut self) {
        assert!(self.cap < (1 << 31));
        self.cap = self.cap << 1;
        self.mem.resize(self.cap, None);
    }

    pub fn push_stack(&mut self, obj: Object) {
        self.mem[self.stack_ptr] = Some(obj);
        self.stack_ptr += 1;
    }

    pub fn pop_stack(&mut self) -> Object {
        let obj = self.mem[self.stack_ptr - 1].take();
        self.stack_ptr -= 1;
        obj.expect("Fail to pop out object from the stack frame")
    }
}

pub struct Vm {
    ip: usize,
    mem: StackFrame,
}

#[derive(Debug, Clone)]
enum Object {
    I(i32),
    Bool(bool),
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            ip: 0,
            mem: StackFrame::new(),
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
        self.mem.push_stack(Object::I(value));
    }

    fn do_add(&mut self) {
        let right = self.mem.pop_stack();
        let left = self.mem.pop_stack();

        let right = cast!(right, Object::I);
        let left = cast!(left, Object::I);
        self.mem.push_stack(Object::I(left + right));
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

        println!("{:?}", self.mem);
    }
}
