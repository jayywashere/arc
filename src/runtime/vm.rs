use crate::bytecode::{Opcode, Bytecode};
use crate::runtime::Value;
use crate::runtime::error::RuntimeError;

pub struct Vm {
    bytecode: Bytecode,
    stack: Vec<Value>,
    pc: usize,
}

impl Vm {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            stack: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            let opcode = Opcode::try_from(self.read_byte())
                .expect("Invalid Opcode.");

            match opcode {
                Opcode::Halt => break,

                Opcode::Ldc => {
                    let idx = self.read_u32() as usize;

                    let val = self.bytecode.constants[idx].clone();
                    self.stack.push(val);
                }

                Opcode::Pop | Opcode::Dup
                    => {
                    if opcode == Opcode::Pop {
                        self.stack.pop().expect("Stack underflow.");
                    } else {
                        self.stack.push(
                            self.stack.last()
                                .cloned()
                                .expect("Stack underflow.")
                        );
                    }
                }

                Opcode::Add | Opcode::Sub |
                Opcode::Mul | Opcode::Div |
                Opcode::Rem => {
                    self.exec_math(
                        match &opcode {
                            Opcode::Add => '+',
                            Opcode::Sub => '-',
                            Opcode::Mul => '*',
                            Opcode::Div => '/',
                            Opcode::Rem => '%',

                            _ => unreachable!(),
                        }
                    )?
                }

                Opcode::Print => {
                    let value = self.stack.pop()
                        .ok_or(RuntimeError::StackUnderflow);

                    println!("{value:?}");
                }
            }
        }

        Ok(())
    }

    fn exec_math(&mut self, op: char) -> Result<(), RuntimeError> {
        let (a, b) = self.pop_nums()?;
    
        let result = match (a, b) {
            // * Int + Int: Perform
            (Value::Int(x), Value::Int(y)) => {
                let res = match op {
                    '+' => x + y,
                    '-' => x - y,
                    '*' => x * y,
                    '/' => {
                        if y == 0 { return Err(RuntimeError::DivisionByZero); }
                        x / y
                    }
                    '%' => {
                        if y == 0 { return Err(RuntimeError::DivisionByZero); }
                        x % y
                    }
                    _ => return Err(
                        RuntimeError::UnknownOperator(op)
                    ),
                };
                Value::Int(res)
            }
    
            // * Float + Int: Cast Int to Float and Perform
            // * Float + Float: Perform
            (val_a, val_b) => {
                let x = match val_a {
                    Value::Int(n) => n as f64,
                    Value::Float(f) => f,

                    _ => unreachable!()
                };

                let y = match val_b {
                    Value::Int(n) => n as f64, 
                    Value::Float(f) => f,

                    _ => unreachable!()
                };
    
                let res = match op {
                    '+' => x + y,
                    '-' => x - y,
                    '*' => x * y,
                    '/' => {
                        if y == 0.0 {
                            return Err(RuntimeError::DivisionByZero);
                        }

                        x / y
                    }
                    '%' => {
                        if y == 0.0 {
                            return Err(RuntimeError::DivisionByZero);
                        }

                        x % y
                    }
                    _ => return Err(
                        RuntimeError::UnknownOperator(op)
                    ),
                };

                Value::Float(res)
            }
        };
    
        self.stack.push(result);
        Ok(())
    }    

    fn pop_nums(&mut self) -> Result<(Value, Value), RuntimeError> {
        let b = self.stack.pop()
            .ok_or(RuntimeError::StackUnderflow)?;

        let a = self.stack.pop()
            .ok_or(RuntimeError::StackUnderflow)?;

        match (&a, &b) {
            (Value::Int(_), Value::Int(_))
            | (Value::Float(_), Value::Float(_)) => {},

            _ => return Err(RuntimeError::TypeMismatch)
        }

        Ok((a, b))
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytecode.code[self.pc];
        self.pc += 1;
        byte
    }

    fn read_u32(&mut self) -> u32 {
        let bytes = [
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
        ];

        u32::from_le_bytes(bytes)
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }
}