use crate::chunk::*;
use crate::utils::print::print_value;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm<'a> {
    chunk: Option<&'a mut Chunk<'a>>,
    ip: Option<&'a OpCode<'a>>,
}

// REVIEW set correct VM structure
// static mut VM: Vm = Vm::new();

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Vm {
            chunk: None,
            ip: None,
        }
    }

    pub fn interpret(&mut self, chunk: &'a mut Chunk<'a>) -> InterpretResult {
        self.chunk = Some(chunk);
        // test hipotesis for use immutable refences for opcode so
        // it doesn't needs to be mutable, it will copy any value
        // inside it if needed to perform the instruction actions
        // (supposelly) freeing memory after that.
        // carries a &mut &mut reference yay......
        self.ip = Some(&mut self.chunk.as_mut().unwrap()
            .code.first()
            .unwrap()
        );

        self.run()
    }

    fn run(&self) -> InterpretResult {
        let mut result = InterpretResult::CompileError;

        result
    }
}
