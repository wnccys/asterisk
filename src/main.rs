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

    let constant = chunk.write_constant(Value::Float(1.2));
    let constant2 = chunk.write_constant(Value::Float(3.4));
    let constant3 = chunk.write_constant(Value::Float(5.6));

    let op_constant = OpCode::OpConstant(&constant);
    let op_constant2 = OpCode::OpConstant(&constant2);
    let op_constant3 = OpCode::OpConstant(&constant3);
    let op_add = OpCode::OpAdd;
    let op_return = OpCode::OpReturn;

    chunk.write(&op_constant, 123);
    chunk.write(&op_constant2, 123);
    chunk.write(&op_add, 123);
    chunk.write(&op_constant3, 124);
    chunk.write(&OpCode::OpDivide, 124);
    chunk.write(&op_return, 128);

    print::disassemble_chunk(&chunk, String::from("test-constants"));

    let result = vm.interpret(&mut chunk);
    println!("{:?}", result);
}

#[cfg(test)]
#[test]
fn simple_add_op() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let const1 = chunk.write_constant(Value::Float(2.0));
    let const2 = chunk.write_constant(Value::Float(2.0));
    let op_const1 = OpCode::OpConstant(&const1);
    let op_const2 = OpCode::OpConstant(&const2);

    chunk.write(&op_const1, 0);
    chunk.write(&op_const2, 0);
    chunk.write(&OpCode::OpAdd, 0);
    chunk.write(&OpCode::OpReturn, 1);

    assert_eq!(InterpretResult::Ok, vm.interpret(&mut chunk));
}

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