use crate::chunk::{Chunk, OpCode, Value};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    for mut i in 0..chunk.count {
        i = disassemble_instruction(&chunk, i);
    }       
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:0>4} ");

    let instruction = &chunk.code[offset];
    let new_offset = match instruction {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant => constant_instruction("OP_CONSTANT", offset),
    };
    
    new_offset
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset+1
}

fn constant_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset+2
}