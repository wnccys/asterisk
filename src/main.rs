pub enum OpCode {
    OpReturn(u8),
}

struct Chunk {
    count: i32,
    code: Vec<OpCode>,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
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
                OpCode::OpReturn(code) => println!("OpReturn: {}", code),
            }
        }
    }
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn(0));
    chunk.disassemble_chunk("test-chunk".to_string());
}