mod chunk;
mod utils;
mod vm;
use crate::chunk::{Chunk, OpCode, Value};
use crate::utils::*;
use crate::vm::vm::Vm;

fn main() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let constant = chunk.write_constant(&Value::Float(&2.0));
    let op_constant = OpCode::OpConstant(&constant);
    let op_return = OpCode::OpReturn;

    chunk.write(&op_constant, 123);
    chunk.write(&op_return, 123);

    print::disassemble_chunk(&chunk, String::from("test-constants"));

    let result = vm.interpret(&mut chunk);
    println!("{:?}", result);
}
