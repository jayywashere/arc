use std::{fs, path};

use crate::{bytecode::MAGIC, runtime::Value};

pub struct Bytecode {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Bytecode {
    pub(crate) fn new(code: Vec<u8>, constants: Vec<Value>) -> Self {
        Self { code, constants }
    }

    pub fn read_from<P: AsRef<path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let bytes = fs::read(path)?;

        if bytes.len() < MAGIC.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "File is too small to be a valid '.arx' binary."
            ));
        }

        if &bytes[..MAGIC.len()] != MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Magic bytes do not match. Is this a valid '.arx' binary?"
            ));
        }

        let mut cursor = MAGIC.len();
        let code_len = read_u32(&bytes, &mut cursor)? as usize;

        if bytes.len() < cursor + code_len {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid binary: Code section is truncated."
            ));
        }

        let code = bytes[cursor..cursor + code_len].to_vec();
        cursor += code_len;

        let constant_count = read_u32(&bytes, &mut cursor)? as usize;
        let mut constants = Vec::with_capacity(constant_count);

        for _ in 0..constant_count {
            constants.push(Self::read_value(&bytes, &mut cursor)?);
        }

        Ok(Self { code, constants })
    }

    fn read_value(bytes: &[u8], cursor: &mut usize) -> Result<Value, std::io::Error> {
        let tag = *bytes.get(*cursor).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid binary: Missing constant type."
            )
        })?;

        *cursor += 1;

        match tag {
            0x00 => {
                let val = read_i64(bytes, cursor)?;
                Ok(Value::Int(val))
            }
            
            0x01 => {
                let val = read_f64(bytes, cursor)?;
                Ok(Value::Float(val))
            }

            0x02 => {
                let val = *bytes.get(*cursor).ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid binary: Missing boolean value."
                    )
                })?;

                *cursor += 1;

                match val {
                    0 => Ok(Value::Bool(false)),
                    1 => Ok(Value::Bool(true)),
                    _ => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid binary: Invalid boolean value."
                    )),
                }
            }

            0x03 => {
                let val = read_u32(bytes, cursor)?;

                let val = char::from_u32(val) .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid binary: Invalid character."
                    )
                })?;

                Ok(Value::Char(val))
            }

            0x04 => {
                let len = read_u32(bytes, cursor)? as usize;
    
                if bytes.len() < *cursor + len {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid binary: String is truncated.",
                    ));
                }
    
                let value = String::from_utf8(bytes[*cursor..*cursor + len].to_vec())
                    .map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid binary: String contains invalid UTF-8.",
                        )
                    })?;
    
                *cursor += len;
    
                Ok(Value::String(value))
            }
    
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid binary: Unknown constant type.",
            )),
        }
    }

    pub fn write_to<P: AsRef<path::Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // * Insert MAGIC Header
        bytes.extend_from_slice(MAGIC);

        // * Code length + Code
        bytes.extend_from_slice(&(self.code.len()as u32).to_le_bytes());
        bytes.extend_from_slice(&self.code);

        // * Constant count + Constants
        bytes.extend_from_slice(&(self.constants.len() as u32).to_le_bytes());
        for constant in &self.constants {
            Self::write_value(&mut bytes, constant);
        }

        fs::write(path, bytes)
    }

    fn write_value(bytes: &mut Vec<u8>, value: &Value) {
        match value {
            Value::Int(val) => {
                bytes.push(0x00);
                bytes.extend_from_slice(&val.to_le_bytes());
            }

            Value::Float(val) => {
                bytes.push(0x01);
                bytes.extend_from_slice(&val.to_le_bytes());
            }

            Value::Bool(val) => {
                bytes.push(0x02);
                bytes.push(*val as u8);
            }

            Value::Char(val) => {
                bytes.push(0x03);
                bytes.extend_from_slice(&(*val as u32).to_le_bytes());
            }

            Value::String(val) => {
                bytes.push(0x04);

                let val_bytes = val.as_bytes();
                let len = u32::try_from(val_bytes.len())
                    .expect("String literal size exceeds maximum u32 limit (4.29 GB).");

                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(val_bytes);
            }
        }
    }
}

fn read_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32, std::io::Error> {
    if bytes.len() < *cursor + 4 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid binary: Unexpected end of file."
        ));
    }

    let value = u32::from_le_bytes(
        bytes[*cursor..*cursor + 4]
            .try_into()
            .unwrap()
    );

    *cursor += 4;

    Ok(value)
}

fn read_i64(bytes: &[u8], cursor: &mut usize) -> Result<i64, std::io::Error> {
    if bytes.len() < *cursor + 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid binary: Unexpected end of file.",
        ));
    }

    let value = i64::from_le_bytes(
        bytes[*cursor..*cursor + 8]
            .try_into()
            .unwrap(),
    );

    *cursor += 8;

    Ok(value)
}

fn read_f64(bytes: &[u8], cursor: &mut usize) -> Result<f64, std::io::Error> {
    if bytes.len() < *cursor + 8 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid binary: Unexpected end of file.",
        ));
    }

    let value = f64::from_le_bytes(
        bytes[*cursor..*cursor + 8]
            .try_into()
            .unwrap(),
    );

    *cursor += 8;

    Ok(value)
}