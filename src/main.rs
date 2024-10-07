mod chunk;
mod compiler;
mod parser;
mod utils;
mod value;
mod vm;
use std::io::{BufRead, Write};
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
        _ => panic!("Usage: ask [path]"),
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

        let bytes_read = handle.read_line(&mut buffer).unwrap();

        if bytes_read == 0 {
            println!("exiting!!");
            break;
        }

        let trimmed_buffer = buffer.trim().to_string();
        let chars: Vec<char> = trimmed_buffer.to_owned().chars().collect();
        vm.interpret(chars);
    }
}

fn run_file(vm: &mut Vm, file_path: &str) {
    let source_code = fs::read_to_string(file_path);
    if source_code.is_err() {
        panic!("could not read bytes from file.")
    }

    let source_chars: Vec<char> = source_code.unwrap().chars().collect();
    let result = vm.interpret(source_chars);

    match result {
        InterpretResult::Ok => (),
        InterpretResult::RuntimeError => std::process::exit(65),
        InterpretResult::CompileError => std::process::exit(75),
    }
}
