use std::{cell::RefCell, rc::Rc};
use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
    Function(Rc<Function>),
    NativeFunction(NativeFn),
    Ref(Rc<RefCell<Value>>),
    Void(()),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Int(_) => write!(f, "Int"),
            Primitive::Float(_) => write!(f, "Float"),
            Primitive::Bool(_) => write!(f, "Bool"),
            Primitive::String(_) => write!(f, "String"),
            _ => panic!("This type does not implement the format trait."),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum FunctionType {
    Fn,
    Script
}

impl Function {
    pub fn new(name: String) -> Self {
        Function {
            arity: 0,
            chunk: Chunk::default(),
            name,
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == self.arity
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct NativeFn {
    pub arity: usize,
    pub _fn: fn(&[Rc<RefCell<Value>>]) -> Value
}

impl NativeFn {
    pub fn call(&mut self, args: &[Rc<RefCell<Value>>]) -> Value {
        if args.len() != self.arity { panic!("Expect {} but got {} arguments.", args.len(), self.arity) }

        (self._fn)(args)
    }
}
