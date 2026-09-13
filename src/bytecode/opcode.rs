#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Control / VM: 0x00–0x0F
    Halt = 0x00,

    // Constants / literals: 0x10–0x1F
    Ldc = 0x10,

    // Stack: 0x20–0x2F
    Pop = 0x20,
    Dup = 0x21,

    // Arithmetic: 0x30–0x3F
    Add = 0x30,
    Sub = 0x31,
    Mul = 0x32,
    Div = 0x33,
    Rem = 0x34,

    // I/O: 0x40–0x4F
    Print = 0x40,
}

impl TryFrom<u8> for Opcode {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::Halt),

            0x10 => Ok(Self::Ldc),

            0x20 => Ok(Self::Pop),
            0x21 => Ok(Self::Dup),
            
            0x30 => Ok(Self::Add),
            0x31 => Ok(Self::Sub),
            0x32 => Ok(Self::Mul),
            0x33 => Ok(Self::Div),
            0x34 => Ok(Self::Rem),

            0x40 => Ok(Self::Print),
            
            _ => Err(byte),
        }
    }
}