use crate::chunk::*;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm<'a> {
    chunk: Option<&'a Chunk<'a>>,
    ip: Option<&'a Vec<&'a OpCode<'a>>>,
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

    pub fn interpret(&mut self, chunk: &'a Chunk) {
        self.chunk = Some(chunk);
        self.ip = Some(&chunk.code);
        // self.run()
    }

    //    fn run(&mut self) -> InterpretResult {}
}
