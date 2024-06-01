
pub enum OpCode<'a> {
    OpReturn,
    // REVIEW sets correct OpConstant Structure;
    // stores index of chosen constant;
    OpConstant(&'a usize),
}

// REVIEW check correct struct to store values inside;
pub enum Value<'a> {
    Float(&'a f32),
}

pub struct Chunk<'a> {
    pub count: usize,
    pub code: Vec<&'a OpCode<'a>>,
    pub constant_count: usize,
    pub constants: Vec<&'a Value<'a>>,
    pub lines: Vec<i32>,
}

impl<'a> Chunk<'a> {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
            constant_count: 0,
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: &'a OpCode, line: i32) {
        self.count += 1;
        self.code.push(byte);
        self.lines.push(line);
    } 

    // FIXME fix lifetime parameters
    pub fn write_constant(&mut self, value: &'a Value) -> usize {
        self.constant_count += 1;
        self.constants.push(value);
        return self.constants.len() -1
    }
}