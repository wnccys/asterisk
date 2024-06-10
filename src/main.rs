mod chunk;
mod utils;
mod vm;
mod value;
use crate::chunk::{Chunk, OpCode};
use crate::utils::*;
use crate::vm::{Vm, InterpretResult};
use crate::value::Value;
use std::io::{BufRead, Write};
use std::{env, fs, io};

fn main() {
    let mut vm = Vm::new();
    check_cmd_args(&mut vm);
    let mut chunk = Chunk::new();

    let constant = chunk.write_constant(Value::Float(1.2));
    let constant2 = chunk.write_constant(Value::Float(3.4));

    let op_constant = OpCode::OpConstant(&constant);
    let op_constant2 = OpCode::OpConstant(&constant2);
    let op_add = OpCode::OpAdd;
    let op_return = OpCode::OpReturn;

    chunk.write(&op_constant, 123);
    chunk.write(&op_constant2, 123);
    chunk.write(&op_add, 123);
    chunk.write(&op_return, 124);

    print::disassemble_chunk(&chunk, String::from("test-constants"));

    let result = vm.interpret(&mut chunk);
    println!("{:?}", result);
}

fn check_cmd_args(vm: &mut Vm) {
    let args: Vec<String> = env::args().collect();

    match args {
        args if args.len() == 1 => repl(vm),
        args if args.len() == 2 => run_file(vm, &args[2]),
        _ => panic!("Usage: astr [path]"),
    }
}

fn repl(vm: &mut Vm) {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        buffer.clear();
        let bytes_read =
            handle
                .read_line(&mut buffer).unwrap();
        
        if bytes_read == 0 {
            println!();
            break;
        }

        // TODO fix arg type
        vm.interpret(buffer.trim());
    }
}

fn run_file(vm: &mut Vm, file: &String) {
    let file_code = fs::read_to_string(file);
    // TODO fix arg type
    let result = vm.interpret(file_code);

    match result {
        InterpretResult::Ok => (),
        InterpretResult::RuntimeError => std::process::exit(65),
        InterpretResult::CompileError =>  std::process::exit(75),
    }
}