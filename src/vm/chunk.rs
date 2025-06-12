use crate::primitives::{types::{Type, Modifier}, primitive::Primitive};

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Return,
    Constant(usize),
    True,
    False,
    Equal,
    /// Same as Equal, but persist the 'b' variable value on stack.
    PartialEqual,
    Pop,
    Greater,
    Less,
    Not,
    Add,
    Multiply,
    Divide,
    Negate,
    Print,
    Nil,
    DefineLocal(usize, Modifier),
    SetLocal(usize, Modifier),
    GetLocal(usize),
    SetRefLocal(usize),
    DefineGlobal(usize, Modifier),
    SetGlobal(usize),
    GetGlobal(usize),
    SetRefGlobal(usize),
    SetType(Type),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Jump(usize),
    Loop(usize),
    Call(usize),
}

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    /// The sequence of Bytecodes used to change the stack state.
    pub code: Vec<OpCode>,
    /// Where values are saved before being used.
    pub constants: Vec<Primitive>,
}

impl Chunk {
    /// Push to code vec.
    ///
    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    /// Push to constants vec.
    ///
    pub fn write_constant(&mut self, value: Primitive) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
