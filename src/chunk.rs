use crate::value::Value;
pub enum OpCode<'a> {
    OpReturn,
    OpConstant(&'a usize),
    OpAdd,
    OpMultiply,
    OpDivide,
}

pub struct Chunk<'a> {
    pub code: Vec<&'a OpCode<'a>>,
    pub stack: Vec<Value>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl<'a> Chunk<'a> {
    pub fn new() -> Self {
        Chunk {
            code: Vec::with_capacity(4),
            stack: Vec::with_capacity(4),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: &'a OpCode, line: i32) {
        dynamize_code_vec(&mut self.code);
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

pub fn dynamize_code_vec(code: &mut Vec<&OpCode>) {
    if code.len() == code.capacity() {
        code.reserve(code.capacity() * 2);
    }
}

pub fn dynamize_stack_vec(stack: &mut Vec<Value>){
    if stack.len() == stack.capacity() {
        stack.reserve(stack.capacity() * 2);
    }
}