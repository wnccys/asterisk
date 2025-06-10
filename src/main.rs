mod chunk;
mod compiler;
mod errors;
mod macros;
mod parser;
mod object;
mod types;
mod utils;
mod native;
mod value;
mod vm;

use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Write};
use std::{env, fs, io};
use vm::{InterpretResult, Vm};

fn main() {
    let mut vm = Vm::default();
    check_cmd_args(&mut vm);
}

fn check_cmd_args(vm: &mut Vm) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(vm),
        2 => run_file(vm, &args[1]),
        _ => panic!("Usage: cargo run -- [file]"),
    }
}

fn repl(vm: &mut Vm) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut buffer = std::io::Cursor::new(String::new());

        print!("> ");
        io::stdout().flush().unwrap();

        let bytes_read = handle.read_line(&mut buffer.get_mut()).unwrap();

        if bytes_read == 0 {
            println!("exiting...");
            break;
        }

        vm.interpret(buffer);
    }
}

fn run_file(vm: &mut Vm, file_path: &str) {
    let input = File::open(file_path).unwrap();
    let source = BufReader::new(input);

    match vm.interpret(source) {
        InterpretResult::Ok => (),
        InterpretResult::RuntimeError => std::process::exit(2),
        InterpretResult::CompileError => std::process::exit(3),
    }
}
