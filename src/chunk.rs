use crate::value::Value;
pub enum OpCode<'a> {
    OpReturn,
    OpConstant(&'a usize),
    OpAdd,
    OpMultiply,
    OpDivide,
}

pub struct Chunk<'a> {
    pub count: usize,
    pub code: Vec<&'a OpCode<'a>>,
    pub stack: Vec<Value>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl<'a> Chunk<'a> {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
            stack: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: &'a OpCode, line: i32) {
        self.count += 1;
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}