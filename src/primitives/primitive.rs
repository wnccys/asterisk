use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

use crate::primitives::{functions::{Closure, Function, NativeFn}, structs::{Instance, Struct}, tuple::Tuple};

use super::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
    Struct(Struct),
    Instance(Instance),
    Tuple(Tuple),
    NativeFunction(NativeFn),
    Function(Rc<Function>),
    Closure(Closure),
    Ref(Rc<RefCell<Value>>),
    Void(()),
}

impl<'a> From<&'a Primitive> for &'a String {
    fn from(_p: &'a Primitive) -> &'a String {
        match _p {
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
            Primitive::Closure(c) => write!(fmt, "&closure<{:?}, {}>", c._fn.arity, c._fn.name),
            Primitive::Struct(_struct) => {
                write!(fmt, "{} {{ ", _struct.name)?;

                for (k, v) in _struct.field_indices.iter() {
                    write!(fmt, "{}: {:?} ", k, v.0)?;
                }

                write!(fmt, "}}")
            },
            Primitive::Instance(inst) => write!(fmt, "instance_of({})", inst._struct.borrow().value),
            Primitive::Tuple(t) => write!(fmt, "{:?}", t.items)
        }
    }
}

#[derive(Debug)]
pub struct UpValue {
    pub index: usize,
    pub is_local: bool,
}