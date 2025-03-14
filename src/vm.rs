use crate::chunk::{Chunk, OpCode};
use crate::value::{Value};

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn read_byte(&mut self) -> u8 {
        let next = self.chunk.code[self.ip];
        self.ip += 1;
        next
    }

    pub fn read_byte_as_constant(&mut self) -> &'a Value {
        &self.chunk.constants[self.read_byte() as usize]
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

pub fn interpret(chunk: &Chunk) {
    let mut vm = VM { chunk, ip: 0, stack: Vec::new() };
    run(&mut vm);
}

pub fn run(vm: &mut VM) {
    loop {
        match vm.read_byte().into() {
            OpCode::Return => {
                let popped = vm.pop();
                println!("Done :) {:?}", popped);
                return;
            }
            OpCode::Constant => {
                let constant = vm.read_byte_as_constant();
                println!("{:?}", constant);
                vm.push(constant.clone());
            }
            OpCode::Unknown => {
                println!("Unknown opcode");
            }
        }
    }
}
