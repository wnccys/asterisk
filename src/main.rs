mod utils;
mod chunk;
use crate::utils::*;
use crate::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn, 123);

    print::disassemble_chunk(&chunk, String::from("test-constants"));
}