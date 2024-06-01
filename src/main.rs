mod utils;
mod chunk;
use crate::utils::*;
use crate::chunk::{Chunk, OpCode, Value};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.write_constant(&Value::Float(&2.0));
    chunk.write(OpCode::OpConstant(constant), 123);

    print::disassemble_chunk(&chunk, String::from("test-constants"));
}