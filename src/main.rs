mod chunk;
mod utils;
mod vm;
mod value;
mod scanner;
mod compiler;
use crate::chunk::{Chunk, OpCode};
use crate::utils::*;
use crate::vm::{Vm, InterpretResult};
use crate::value::Value;
use std::io::{BufRead, Write};
use std::{env, fs, io};

fn main() {
    let mut vm = Vm::new();
    check_cmd_args(&mut vm);
}

fn check_cmd_args(vm: &mut Vm) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(vm),
        2 => run_file(vm, &args[1]),
        _ => panic!("Usage: astr [path]"),
    }
}

fn repl(vm: &mut Vm) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
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

        let trimmed_buffer = buffer.trim().to_string();

        {
            vm.interpret(&trimmed_buffer);
        }
    }
}

fn run_file(vm: &mut Vm, file: &String) {
    let file_code = fs::read_to_string(file);
    if file_code.is_err() { panic!("could not read bytes from file.") }
    
    let result = vm.interpret(&file_code.unwrap());

    match result {
        InterpretResult::Ok => (),
        InterpretResult::RuntimeError => std::process::exit(65),
        InterpretResult::CompileError =>  std::process::exit(75),
    }
}