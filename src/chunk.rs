pub enum OpCode {
    OpReturn(u8),
    OpConstant(u8, usize),
}

pub struct Chunk {
    pub count: usize,
    pub code: Vec<OpCode>,
    pub constants: Vec<f32>,
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
        self.code.push(byte);
        self.count += 1;
        self.lines.push(line)
    } 

    pub fn add_constant(&mut self, value: f32) -> usize {
        self.constants.push(value);

        self.constants.len() -1
    }
}