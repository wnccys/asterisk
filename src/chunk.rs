use crate::value::Value;
#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Constant(usize),
    True,
    False,
    Equal,
    Nil,
    Pop,
    Greater,
    Less,
    Not,
    Add,
    Multiply,
    Divide,
    Negate,
    Print,
    DefineGlobal(usize),
    GetGlobal(usize),
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
        dynamize_vec(&mut self.code);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

pub fn dynamize_vec<T: Clone>(vec: &mut Vec<T>) {
    if vec.len() == vec.capacity() {
        vec.reserve(vec.capacity())
    }
}
