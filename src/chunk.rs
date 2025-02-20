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
    SetGlobal(usize),
    GetGlobal(usize),
}

#[derive(Debug, Default)]
pub struct Chunk {
    /// The sequence of Bytecodes' used to change the stack state.
    pub code: Vec<OpCode>,
    /// Where the Bytecodes' operations itself are executed.
    pub stack: Vec<Value>,
    /// Where values are saved before being used.
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    /// Write OpCode to code vec 
    /// 
    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Write to constants vec
    /// 
    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
