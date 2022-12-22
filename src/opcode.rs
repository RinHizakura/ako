pub const OPCODE_CONST: u8 = 0;
pub const OPCODE_ADD: u8 = 1;

pub fn operand_width(opcode: u8) -> usize {
    match opcode {
        OPCODE_CONST => 4,
        OPCODE_ADD => 0,
        _ => unreachable!(),
    }
}
