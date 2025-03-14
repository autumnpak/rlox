use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("==== {} ====", name);
    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{number:0>4} ", number=offset);
    let instruction: OpCode = chunk.code[offset].into();
    match instruction {
        OpCode::Return => {
            simple_instruction("RETURN", offset)
        }
        OpCode::Constant => {
            constant_instruction("CONSTANT", chunk, offset)
        }
        _ => {
            println!("Unknown opcode {:?}", instruction);
            512
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constant_instruction (name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant: usize = chunk.code[offset + 1].into();
    println!("{blah:<16} {number:<4} '{:?}'", chunk.constants[constant], blah=name, number=constant);
    return offset + 2;
}
