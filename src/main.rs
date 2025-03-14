mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = chunk::Chunk { code: Vec::new(), constants: Vec::new() };
    let constant = chunk.add_constant(value::Value::Double(1.2));
    let constant2 = chunk.add_constant(value::Value::Double(7.7));
    chunk.write_chunk(chunk::OpCode::Return as u8);
    chunk.write_chunk(chunk::OpCode::Constant as u8);
    chunk.write_chunk(constant as u8);
    chunk.write_chunk(chunk::OpCode::Constant as u8);
    chunk.write_chunk(constant2 as u8);
    chunk.write_chunk(chunk::OpCode::Return as u8);
    chunk.write_chunk(chunk::OpCode::Return as u8);
    println!("{:?}", chunk);
    debug::disassemble_chunk(&chunk, "test");
}
