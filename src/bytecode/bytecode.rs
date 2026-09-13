use crate::runtime::Value;

pub struct Bytecode {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Bytecode {
    pub(crate) fn new(code: Vec<u8>, constants: Vec<Value>) -> Self {
        Self { code, constants }
    }
}