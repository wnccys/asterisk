use std::{cell::RefCell, rc::Rc};

use crate::value::{Modifier, Primitive, Type, Value};

#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Constant(usize),
    True,
    False,
    Equal,
    Pop,
    Greater,
    Less,
    Not,
    Add,
    Multiply,
    Divide,
    Negate,
    Print,
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
    Jump(usize),
}

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    /// The sequence of Bytecodes used to change the stack state.
    pub code: Vec<OpCode>,
    /// Where the Bytecodes' operations itself are executed.
    pub stack: Vec<Rc<RefCell<Value>>>,
    /// Where values are saved before being used.
    pub constants: Vec<Primitive>,
    pub lines: Vec<i32>,
}

impl Chunk {
    /// Push to code vec.
    ///
    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Push to constants vec.
    ///
    pub fn write_constant(&mut self, value: Primitive) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
