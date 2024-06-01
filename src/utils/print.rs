use crate::Chunk;
use crate::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    for (i, c) in chunk.code.iter().enumerate() {
        if i > 0 {
            match chunk.lines[i] == chunk.lines[i-1] {
                true => print!("  | "),
                _ => print!("{} ", chunk.lines[i]),
            }
        } else {
            print!("{} ", chunk.lines[i]);
        }

        match c {
            OpCode::OpReturn(code) => 
                println!("{code:0>4} OpReturn"),
            OpCode::OpConstant(code, index) => 
                println!("{code:0>4} OpConstant {} {:.1}", i, chunk.constants[*index]),
        }
    }
}