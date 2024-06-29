use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

#[allow(unused)]
pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    let mut i = 0;
    for _ in 0..chunk.code.len() {
        i = disassemble_instruction(chunk, i);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:0>4} ");
    verify_lines(offset, &chunk);

    let instruction = &chunk.code[offset];

    match instruction {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant(index) => constant_instruction("OP_CONSTANT", chunk, &index, offset),
        OpCode::OpAdd => simple_instruction("OP_ADD", offset),
        OpCode::OpMultiply => simple_instruction("OP_MULTIPLY", offset),
        OpCode::OpDivide => simple_instruction("OP_DIVIDE", offset),
        OpCode::OpNegate => simple_instruction("OP_NEGATE", offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, op_index: &usize, offset: usize) -> usize {
    let spaces: usize = 6;
    print!("{name}{op_index:>spaces$} ");

   print_value(&chunk.constants[*op_index]);

    offset + 1
}

pub fn print_value(value: &Value) {
    match value {
        Value::Float(f) => println!("'{f:.1}'"),
        Value::Int(i) => println!("'{i}'"),
    }
}

pub fn print_stack(chunk: &Chunk) {
    println!("=====stack-trace=====");
    for value in chunk.stack.iter() {
        print!(">");
        print_value(value);
    }
}

fn verify_lines(offset: usize, chunk: &Chunk) {
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("| ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }
}
