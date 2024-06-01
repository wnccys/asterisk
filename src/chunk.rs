
pub enum OpCode {
    OpReturn,
    // FIXME sets correct OpConstant Structure;
    OpConstant,
}

pub enum Value {
    Float(f32),
}

pub struct Chunk {
    pub count: usize,
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.count += 1;
        self.code.push(byte);
        self.lines.push(line);
    } 
}