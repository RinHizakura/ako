pub const OPCODE_CONST: u8 = 0;
pub const OPCODE_ADD: u8 = 1;
pub const OPCODE_SET_LOCAL: u8 = 2;
pub const OPCODE_SET_GLOBAL: u8 = 3;
pub const OPCODE_GET_LOCAL: u8 = 4;
pub const OPCODE_GET_GLOBAL: u8 = 5;

pub fn operand_width(opcode: u8) -> usize {
    match opcode {
        OPCODE_CONST => 4,
        OPCODE_ADD => 0,
        OPCODE_SET_LOCAL => 2,
        OPCODE_SET_GLOBAL => 2,
        OPCODE_GET_LOCAL => 2,
        OPCODE_GET_GLOBAL => 2,
        _ => unreachable!(),
    }
}
