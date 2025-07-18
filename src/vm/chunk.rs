use crate::primitives::{
    primitive::Primitive,
    types::{Modifier, Type},
};

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
    DefineLocal(usize, Modifier, Type),
    SetLocal(usize, Modifier),
    GetLocal(usize),
    SetRefLocal(usize),
    DefineGlobal(usize, Modifier, Type),
    SetGlobal(usize),
    GetGlobal(usize),
    SetRefGlobal(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Jump(usize),
    Loop(usize),
    Call(usize),
    Closure(usize),
    GetUpValue(usize),
    SetUpValue(usize),
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
