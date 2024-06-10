use asterisk::vm::{Vm, InterpretResult};
use asterisk::chunk::{Chunk, OpCode};
use asterisk::value::Value;

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
fn simple_sub_op(){
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();

    let const1 = chunk.write_constant(Value::Float(2.0));
    let const2 = chunk.write_constant(Value::Float(-6.0));
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

    assert_eq!(vm.interpret(&mut chunk), InterpretResult::Ok); 
}