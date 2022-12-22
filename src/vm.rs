use crate::opcode::*;

pub struct Vm {
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Vm { ip: 0 }
    }

    pub fn run(&mut self, bytecode: Vec<u8>) {
        let len = bytecode.len();

        while self.ip < len {
            let opcode = bytecode[self.ip];
            match opcode {
                OPCODE_CONST => todo!(),
                OPCODE_ADD => todo!(),
                _ => unreachable!(),
            }
        }
        todo!()
    }
}
