 enum OpCode {
    OpReturn(u8),
    OpConstant(u8, usize),
}

struct Chunk {
    count: i32,
    code: Vec<OpCode>,
    constants: Vec<f32>,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
        self.count += 1;
    } 

    fn disassemble_chunk(&self, name: String) {
        println!("===%=== {} ===%===", name);

        for i in self.code.iter() {
            match i {
                OpCode::OpReturn(code) => 
                    println!("OpReturn: {}", code),
                OpCode::OpConstant(code, index) => 
                    println!("OpConstant: {}, index: {}", code, self.constants[*index]),
            }
        }
    }

    fn add_constant(&mut self, value: f32) -> usize {
        self.constants.push(value);

        self.constants.len() -1
    }
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn(0));

    let index = chunk.add_constant(2.0);
    chunk.write(OpCode::OpConstant(1, index));
    chunk.disassemble_chunk(String::from("test-constants"));
}