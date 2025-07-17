use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::vm::chunk::Chunk;

use super::value::Value;

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

impl<'a> From<&'a Primitive> for &'a String {
    fn from(_p: &'a Primitive) -> &'a String {
        match _p  {
            Primitive::String(s) => s,
            _ => panic!("Invalid Cast to &Primitive for String")
        }
    }
} 

impl Display for Primitive {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Float(f) => write!(fmt, "{f:.1}"),
            Primitive::Int(i) => write!(fmt, "{i}"),
            Primitive::Bool(b) => write!(fmt, "{b}"),
            Primitive::String(str) => write!(fmt, "{}", str),
            Primitive::Void(t) => write!(fmt, "{t:?}"),
            Primitive::Ref(value_ptr) => write!(fmt, "&{}", value_ptr.borrow().value),
            Primitive::Function(f) => write!(fmt, "&fn<{}, {}>", f.arity, f.name),
            Primitive::NativeFunction(f) => write!(fmt, "&native_fn<{:?}>", f),
            _ => panic!("invalid value."),
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
    Script,
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
