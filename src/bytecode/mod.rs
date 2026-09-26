pub mod bytecode;
pub mod instruction;
pub mod opcode;

pub use bytecode::Bytecode;
pub use instruction::Instruction;
pub use opcode::Opcode;

pub const MAGIC: &[u8; 4] = b"ARC\0";