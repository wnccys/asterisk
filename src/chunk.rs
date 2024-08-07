use crate::value::Value;
#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Constant(usize),
    True,
    False,
    Equal,
    Greater,
    Less,
    Not,
    Add,
    Multiply,
    Divide,
    Negate,
}

#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub stack: Vec<Value>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
        dynamize_code_vec(&mut self.code);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

pub fn dynamize_code_vec(code: &mut Vec<OpCode>) {
    if code.len() == code.capacity() {
        code.reserve(code.capacity());
    }
}

pub fn dynamize_stack_vec(stack: &mut Vec<Value>) {
    if stack.len() == stack.capacity() {
        stack.reserve(stack.capacity());
    }
}
