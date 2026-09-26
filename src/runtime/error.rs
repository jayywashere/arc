use std::fmt;

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

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode(opcode) => {
                write!(
                    f,
                    "[RuntimeError] InvalidOpcode: Unknown opcode {opcode}."
                )
            }

            Self::UnexpectedEndOfBytecode => {
                write!(
                    f,
                    "[RuntimeError] UnexpectedEndOfBytecode: Bytecode ended before the instruction was complete."
                )
            }

            Self::StackUnderflow => {
                write!(
                    f,
                    "[RuntimeError] StackUnderflow: Attempted to access/pop a value from an empty stack."
                )
            }

            Self::StackOverflow => {
                write!(
                    f,
                    "[RuntimeError] StackOverflow: The VM stack cannot hold any more values."
                )
            }

            Self::InvalidConstantIndex(index) => {
                write!(
                    f,
                    "[RuntimeError] InvalidConstantIndex: Constant index '{index}' is out of bounds."
                )
            }

            Self::TypeMismatch => {
                write!(
                    f,
                    "[RuntimeError] TypeMismatch: This operation cannot be performed on the given value types."
                )
            }

            Self::DivisionByZero => {
                write!(
                    f,
                    "[RuntimeError] DivisionByZero: Cannot divide by zero."
                )
            }

            Self::UnknownOperator(operator) => {
                write!(
                    f,
                    "[RuntimeError] UnknownOperator: '{operator}' is not a recognized operator."
                )
            }
        }
    }
}