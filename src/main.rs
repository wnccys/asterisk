mod chunk;
mod utils;
mod vm;
mod value;
use crate::chunk::{Chunk, OpCode};
use crate::utils::*;
use crate::vm::{Vm, InterpretResult};
use crate::value::Value;

fn main() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let constant = chunk.write_constant(Value::Float(2.0));
    let op_constant = OpCode::OpConstant(&constant);
    let op_return = OpCode::OpReturn;

    chunk.write(&op_constant, 123);
    chunk.write(&op_return, 128);

    print::disassemble_chunk(&chunk, String::from("test-constants"));

    let result = vm.interpret(&mut chunk);
    println!("{:?}", result);
}

#[cfg(test)]
#[test]
fn multiple_add_op() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let constant = chunk.write_constant(Value::Float(2.0));
    let op_constant = OpCode::OpConstant(&constant);
    let op_add = OpCode::OpAdd;
    let op_return = OpCode::OpReturn;

    chunk.write(&op_constant, 123);
    chunk.write(&op_constant, 123);
    chunk.write(&op_add, 124);
    chunk.write(&op_constant, 125);
    chunk.write(&op_constant, 125);
    chunk.write(&op_add, 126);
    chunk.write(&op_add, 127);
    chunk.write(&op_return, 128);
    let result = vm.interpret(&mut chunk);

    assert_eq!(InterpretResult::Ok, result)
}