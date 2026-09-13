#[derive(Debug)]
pub enum RuntimeError {
    InvalidOpcode(u8),
    UnexpectedEndOfBytecode,
    StackUnderflow,
    StackOverflow,
    InvalidConstantIndex(u32),
    TypeMismatch,
    DivisionByZero,
    UnknownOperator(char),
}