mod utils;
mod chunk;
use crate::utils::*;
use crate::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();

    let index = chunk.add_constant(2.0);
    chunk.write(OpCode::OpConstant(1, index), 123);
    chunk.write(OpCode::OpReturn(0), 123);
    let index = chunk.add_constant(3.0);
    chunk.write(OpCode::OpConstant(2, index), 200);

    print::disassemble_chunk(&chunk, String::from("test-constants"));
}