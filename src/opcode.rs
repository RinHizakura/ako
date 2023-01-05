pub const OPCODE_CONST: u8 = 0;
pub const OPCODE_AND: u8 = 1;
pub const OPCODE_OR: u8 = 2;
pub const OPCODE_XOR: u8 = 3;
pub const OPCODE_ADD: u8 = 4;
pub const OPCODE_SUB: u8 = 5;
pub const OPCODE_MUL: u8 = 6;
pub const OPCODE_DIV: u8 = 7;
pub const OPCODE_MODULO: u8 = 8;
pub const OPCODE_SET_LOCAL: u8 = 9;
pub const OPCODE_SET_GLOBAL: u8 = 10;
pub const OPCODE_GET_LOCAL: u8 = 11;
pub const OPCODE_GET_GLOBAL: u8 = 12;

pub fn operand_width(opcode: u8) -> usize {
    match opcode {
        OPCODE_CONST => 4,
        OPCODE_AND | OPCODE_OR | OPCODE_XOR | OPCODE_ADD | OPCODE_SUB | OPCODE_MUL | OPCODE_DIV
        | OPCODE_MODULO => 0,
        OPCODE_SET_LOCAL | OPCODE_SET_GLOBAL | OPCODE_GET_LOCAL | OPCODE_GET_GLOBAL => 2,
        _ => unreachable!(),
    }
}
