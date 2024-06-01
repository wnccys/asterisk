mod utils;
mod chunk;
use crate::utils::*;
use crate::chunk::{Chunk, OpCode, Value};

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.write_constant(&Value::Float(&2.0));
    let op_constant = OpCode::OpConstant(&constant);
    chunk.write(&op_constant, 123);
    let op_return = OpCode::OpReturn;
    chunk.write(&op_return, 123);

    print::disassemble_chunk(&chunk, String::from("test-constants"));
}