use core::fmt;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
    sync::Arc,
};

/// Asterisk types definition.
///
#[derive(Debug, Clone)]
pub struct Value {
    pub value: Primitive,
    pub _type: Type,
    pub modifier: Modifier,
}

/* Primitives are variable assigned data, Type are the check for this data to be valid throught the runtime */
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    /* Types need to be thread-safe, that's why Arc<..> is here */
    Ref(Arc<Type>),
    Void,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Ref(*const Primitive),
    RefMut(*mut Primitive),
    Void(()),
}

impl<'a> Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Int(_) => write!(f, "Int"),
            Primitive::Float(_) => write!(f, "Float"),
            Primitive::Bool(_) => write!(f, "Bool"),
            Primitive::String(_) => write!(f, "String"),
            Primitive::Ref(primitive) => write!(f, "&({})", unsafe { (**primitive).clone() }),
            Primitive::RefMut(primitive) => write!(f, "&({})", unsafe { (**primitive).clone() }),
            _ => panic!("This type does not implement the format trait."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Unassigned,
    Const,
    Mut,
}

crate::macros::gen_primitives_operations!(Float, Int);
crate::macros::gen_values_operations!(Int, Float);
