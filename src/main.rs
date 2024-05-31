enum OpCode {
    OpReturn(u8),
    OpConstant(u8, usize),
}

struct Chunk {
    count: usize,
    code: Vec<OpCode>,
    constants: Vec<f32>,
    lines: Vec<i32>,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    fn write(&mut self, byte: OpCode, line: i32) {
        self.code.push(byte);
        self.count += 1;
        self.lines.push(line)
    } 

    fn disassemble_chunk(&self, name: String) {
        println!("===%=== {} ===%===", name);

        for (i, c) in self.code.iter().enumerate() {
            if i > 0 {
                match self.lines[i] == self.lines[i-1] {
                    true => print!("  | "),
                    _ => print!("{} ", self.lines[i]),
                }
            } else {
                print!("{} ", self.lines[i]);
            }

            match c {
                OpCode::OpReturn(code) => 
                    println!("{code:0>4} OpReturn"),
                OpCode::OpConstant(code, index) => 
                    println!("{code:0>4} OpConstant {} {:.1}", i, self.constants[*index]),
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

    let index = chunk.add_constant(2.0);
    chunk.write(OpCode::OpConstant(1, index), 123);
    chunk.write(OpCode::OpReturn(0), 123);
    let index = chunk.add_constant(3.0);
    chunk.write(OpCode::OpConstant(2, index), 200);

    chunk.disassemble_chunk(String::from("test-constants"));
}