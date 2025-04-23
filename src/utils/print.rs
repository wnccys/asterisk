use crate::chunk::{Chunk, OpCode};
use crate::value::{Primitive, Value};

#[allow(unused)]
pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("===%=== {} ===%===", name);

    disassemble_instruction(chunk, 0);
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    // Recursion base case
    if offset >= chunk.code.len() {
        return;
    }

    print!("{offset:0>4} ");
    verify_lines(offset, chunk);

    let instruction = &chunk.code[offset];

    let offset = match instruction {
        OpCode::Return => simple_instruction("OP_RETURN", offset),
        OpCode::Constant(index) => constant_instruction("OP_CONSTANT", chunk, index, offset),
        OpCode::Add => simple_instruction("OP_ADD", offset),
        OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
        OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
        OpCode::Negate => simple_instruction("OP_NEGATE", offset),
        OpCode::True => simple_instruction("OP_TRUE", offset),
        OpCode::False => simple_instruction("OP_FALSE", offset),
        OpCode::Not => simple_instruction("OP_NOT", offset),
        OpCode::Equal => simple_instruction("OP_EQUAL", offset),
        OpCode::Greater => simple_instruction("OP_GREATER", offset),
        OpCode::Less => simple_instruction("OP_LESS", offset),
        OpCode::Print => simple_instruction("OP_PRINT", offset),
        OpCode::Pop => simple_instruction("OP_POP", offset),
        OpCode::DefineGlobal(index, _) => {
            constant_instruction("OP_DEFINE_GLOBAL", chunk, index, offset)
        }
        OpCode::GetGlobal(index) => constant_instruction("OP_GET_GLOBAL", chunk, index, offset),
        OpCode::SetGlobal(index) => constant_instruction("OP_SET_GLOBAL", chunk, index, offset),
        OpCode::GetLocal(index) => byte_instruction("OP_GET_LOCAL", chunk, index, offset),
        OpCode::SetLocal(index, _) => byte_instruction("OP_SET_LOCAL", chunk, index, offset),
        OpCode::SetRef(_) => simple_instruction("OP_SET_REF", offset),
        OpCode::SetType(_) => simple_instruction("OP_SET_TYPE", offset)
    };

    disassemble_instruction(chunk, offset);
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, op_index: &usize, offset: usize) -> usize {
    let spaces: usize = 20 - name.len();
    print!("{name}{op_index:>spaces$} ");

    print_value(&chunk.constants[*op_index]);

    offset + 1
}

fn byte_instruction(name: &str, chunk: &Chunk, index: &usize, offset: usize) -> usize {
    let slot = &chunk.code[*index];

    println!("{name}      {slot:?}");
    return offset + 1;
}

pub fn print_value(value: &Primitive) {
    match value {
        Primitive::Float(f) => println!("{f:.1}"),
        Primitive::Int(i) => println!("{i}"),
        Primitive::Bool(b) => println!("{b}"),
        Primitive::String(str) => println!("{str}"),
        Primitive::Void(t) => println!("{t:?}"),
        Primitive::Ref(value_ptr) => {
            let ref_value = &value_ptr.borrow().value;
            print!("&");
            print_value(&ref_value);
        },
        _ => panic!("invalid value."),
    }
}

pub fn print_stack(chunk: &Chunk) {
    println!("==stack-trace==");
    for value in chunk.stack.iter() {
        print!(">");
        print_value(&value.value);
    }
    println!("===end-trace===")
}

fn verify_lines(offset: usize, chunk: &Chunk) {
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("| ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }
}
