use crate::value::{Value};

#[derive(Debug)]
pub enum OpCode {
    Constant = 0,
    Return,
    Unknown
}

impl From<u8> for OpCode {
    fn from(value: u8) -> OpCode {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            _ => OpCode::Unknown
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn write_chunk(&mut self, chunk: u8) {
        self.code.push(chunk);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
