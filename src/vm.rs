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
        write!(f, "StackFrame [")?;
        for i in 0..self.stack_ptr {
            write!(f, "{:?}, ", self.mem[i])?;
        }
        write!(f, "]")
    }
}

impl StackFrame {
    pub fn new() -> Self {
        let cap = 128;
        let mut vec = Vec::new();
        vec.resize(cap, None);
        StackFrame {
            mem: vec,
            cap: cap,
            stack_ptr: 0,
        }
    }

    pub fn reset(&mut self) {
        self.stack_ptr = 0;
    }

    pub fn push_stack(&mut self, obj: Object) {
        if self.stack_ptr >= self.cap {
            assert!(self.cap < (1 << 31));
            self.cap <<= 1;
            /* FIXME: Will this cost too much with clone behind the function? */
            self.mem.resize(self.cap, None);
        }
        self.mem[self.stack_ptr] = Some(obj);
        self.stack_ptr += 1;
    }

    pub fn pop_stack(&mut self) -> Object {
        let obj = self.mem[self.stack_ptr - 1].take();
        self.stack_ptr -= 1;
        obj.expect("Fail to pop out object from the stack frame")
    }
}

#[derive(Debug)]
struct ObjectList {
    mem: Vec<Option<Object>>,
    cap: usize,
}

impl ObjectList {
    pub fn new() -> Self {
        let cap = 8;
        let mut vec = Vec::new();
        vec.resize(cap, None);
        ObjectList { mem: vec, cap: cap }
    }

    pub fn reset(&mut self) {
        self.cap = 8;
        self.mem.clear();
        self.mem.resize(self.cap, None);
    }

    pub fn set(&mut self, idx: usize, obj: Object) {
        if idx >= self.cap {
            while idx >= self.cap {
                assert!(self.cap < (1 << 31));
                self.cap <<= 1;
            }
            self.mem.resize(self.cap, None);
        }
        self.mem[idx] = Some(obj);
    }

    pub fn get(&mut self, idx: usize) -> Object {
        self.mem[idx].as_ref().unwrap().clone()
    }
}

pub struct Vm {
    ip: usize,
    stack_frame: StackFrame,
    globals: ObjectList,
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
            stack_frame: StackFrame::new(),
            globals: ObjectList::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.stack_frame.reset();
        self.globals.reset();
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
        self.stack_frame.push_stack(Object::I(value));
    }

    fn do_add(&mut self) {
        let right = self.stack_frame.pop_stack();
        let left = self.stack_frame.pop_stack();

        let right = cast!(right, Object::I);
        let left = cast!(left, Object::I);
        self.stack_frame.push_stack(Object::I(left + right));
    }

    fn do_set_local(&mut self, bytecode: &Vec<u8>) {
        todo!();
    }

    fn do_set_global(&mut self, bytecode: &Vec<u8>) {
        let idx = self.get_operand(bytecode, OPCODE_SET_GLOBAL) as usize;
        let obj = self.stack_frame.pop_stack();
        self.globals.set(idx, obj);
    }

    fn do_get_global(&mut self, bytecode: &Vec<u8>) {
        let idx = self.get_operand(bytecode, OPCODE_GET_GLOBAL) as usize;
        let obj = self.globals.get(idx);
        self.stack_frame.push_stack(obj);
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
                OPCODE_SET_GLOBAL => self.do_set_global(&bytecode),
                OPCODE_GET_LOCAL => todo!(),
                OPCODE_GET_GLOBAL => self.do_get_global(&bytecode),
                _ => unreachable!(),
            }

            self.ip += 1 + operand_width;
        }

        println!("{:?}", self.stack_frame);
        println!("{:?}", self.globals);
    }
}
