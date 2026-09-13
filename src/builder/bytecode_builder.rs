use crate::bytecode::{Bytecode, Instruction, Opcode};
use crate::runtime::Value;

pub struct BytecodeBuilder {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) -> &mut Self {
        self.code.push(instruction.opcode as u8);

        if let Some(operand) = instruction.operand {
            self.code.extend_from_slice(&operand.to_le_bytes());
        }

        self
    }

    pub fn const_val(&mut self, value: Value) -> u32 {
        let index = self.constants.len() as u32;
        self.constants.push(value);
        index
    }

    pub fn op(&mut self, opcode: Opcode) -> &mut Self {
        self.emit(Instruction {
            opcode,
            operand: None
        });

        self
    }

    pub fn ldc(&mut self, value: Value) -> &mut Self {
        let idx = self.const_val(value);
        self.emit(Instruction {
            opcode: Opcode::Ldc,
            operand: Some(idx)
        });

        self
    }

    pub fn finish(self) -> Bytecode {
        Bytecode::new(self.code, self.constants)
    }

    pub fn clear(&mut self) {
        self.code.clear();
        self.constants.clear();
    }

    pub fn finish_and_reset(&mut self) -> Bytecode {
        let bytecode = Bytecode::new(self.code.clone(), self.constants.clone());
        self.clear();

        bytecode
    }
}