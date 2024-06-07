use crate::chunk::{Chunk, OpCode, Value};

#[allow(unused)]
pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    let mut i = 0;
    for _ in 0..chunk.count {
        i = disassemble_instruction(chunk, i);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:0>4} ");
    verify_lines(offset, chunk);

    let instruction = chunk.code[offset];

    match instruction {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant(index) => constant_instruction("OP_CONSTANT", chunk, index, offset),
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
        Value::Float(value) => println!("'{value:.1}'"),
    }
}

pub fn print_stack(chunk: &Chunk) {
    print!("         ");
    for &slot in chunk.stack.iter() {
        print!(">");
        print_value(&slot);
    }
}

fn verify_lines(offset: usize, chunk: &Chunk) {
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("  | ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }
}
