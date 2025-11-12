use std::{cell::RefCell, rc::Rc};

use crate::{primitives::value::Value, vm::chunk::Chunk};

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
    // Upvalue count
    pub upv_count: usize,
}

#[derive(Debug, PartialEq)]
pub enum FunctionType {
    // Main script ctx
    Script,
    // Scoped fn
    Fn,
}

impl Function {
    pub fn new(name: String) -> Self {
        Function {
            arity: 0,
            chunk: Chunk::default(),
            upv_count: 0,
            name,
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == self.arity
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFn {
    pub arity: usize,
    pub _fn: fn(&[Rc<RefCell<Value>>]) -> Value,
}

impl NativeFn {
    pub fn call(&mut self, args: &[Rc<RefCell<Value>>]) -> Value {
        if args.len() != self.arity {
            panic!("Expect {} but got {} arguments.", self.arity, args.len())
        }

        (self._fn)(args)
    }
}