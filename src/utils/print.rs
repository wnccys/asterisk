use crate::chunk::{Chunk, OpCode, Value};

 #[allow(unused)]
pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    let mut i = 0;
    for _ in 0..chunk.count {
        i = disassemble_instruction(&chunk, i);
    }       
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:0>4} ");

    let instruction = chunk.code[offset];

    let new_offset = match instruction {
        OpCode::OpReturn => 
            simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant(index) => 
            constant_instruction("OP_CONSTANT", &chunk, index, offset),
    };
    
    new_offset
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, op_index: &usize, offset: usize) -> usize {
    let spaces: usize = 6;
    print!("{name}{op_index:>spaces$} ");

    match chunk.constants[*op_index] {
        Value::Float(value) => print_value(value),
    }

    offset + 1
}

fn print_value(value: &f32) {
    println!("'{value:.1}'");
}