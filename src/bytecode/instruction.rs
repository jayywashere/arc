use crate::bytecode::Opcode;

pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Option<u32>,
}