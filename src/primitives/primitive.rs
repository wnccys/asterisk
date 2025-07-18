use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::primitives::functions::{Function, NativeFn};

use super::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
    Function(Rc<Function>),
    NativeFunction(NativeFn),
    Closure {_s: Option<()>, _fn: Rc<Function>},
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
            Primitive::Closure { _fn, _s } => write!(fmt, "&closure<{:?}, {}>", _fn.arity, _fn.name),
            _ => panic!("invalid value."),
        }
    }
}