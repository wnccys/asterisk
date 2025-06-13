mod errors;
mod macros;
mod objects;
mod parser;
mod primitives;
mod utils;
mod vm;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{env, io};
use vm::Vm;

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

    vm.interpret(source);
}
