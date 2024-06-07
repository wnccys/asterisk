pub enum OpCode<'a> {
    OpReturn,
    OpConstant(&'a usize),
    OpNegate,
}

pub enum Value<'a> {
    Float(&'a f32),
}

pub struct Chunk<'a> {
    pub count: usize,
    pub code: Vec<&'a OpCode<'a>>,
    pub stack: Vec<&'a Value<'a>>,
    pub stack_top: usize,
    pub constant_count: usize,
    pub constants: Vec<&'a Value<'a>>,
    pub lines: Vec<i32>,
}

impl<'a> Chunk<'a> {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
            stack: Vec::new(),
            stack_top: 0,
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

    pub fn write_constant(&mut self, value: &'a Value) -> usize {
        self.constant_count += 1;
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn push(&mut self, value: &'a Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> &Value {
        self.stack_top -= 1;

        return self.stack.pop().unwrap() 
    }

}

pub trait ValueOperations {
    fn negate(&self) -> Value<'_>;
}

impl<'a> ValueOperations for Value<'a> {
    fn negate(&self) -> Self {
        match self {
            Value::Float(value) => {
                let result = (*value * -1.0) as f32;
                Value::Float(Box::leak(Box::new(result)))
            },
            _ => panic!("operation not allowed for this variant"),
        }
    }
}
