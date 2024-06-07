pub enum OpCode<'a> {
    OpReturn,
    OpConstant(&'a usize),
    OpNegate,
}

#[derive(Clone)]
pub enum Value {
    Float(f32),
}

pub struct Chunk<'a> {
    pub count: usize,
    pub code: Vec<&'a OpCode<'a>>,
    pub stack: Vec<Value>,
    pub stack_top: usize,
    pub constant_count: usize,
    pub constants: Vec<Value>,
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

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constant_count += 1;
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    pub fn pop_stack(&mut self) -> Value {
        self.stack_top -= 1;

        return self.stack.pop().unwrap() 
    }

}

impl Copy for Value {}

impl Value {
     pub fn negate(&self) -> Value {
        match self {
            Value::Float(value) => Value::Float(-value),
            _ => panic!("operation not allowed for this variant"),
        }
     }
    // pub fn custom_copy(&self) -> Self {
    //     match self {
    //         Value::Float(value) => Value::Float(*value),
    //     }
    // }
}