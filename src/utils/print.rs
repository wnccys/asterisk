use crate::vm::chunk::Chunk;
use crate::vm::Stack;

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    for (i, code) in chunk.code.iter().enumerate() {
        print!("{i:0>4} ");
        println!("{code:?}");
    }
}

pub fn print_stack(stack: &Stack) {
    println!("==stack-trace==");
    for value in stack.iter().rev() {
        print!(">");
        println!("{}", &value.borrow().value);
    }
    println!("===end-trace===")
}
