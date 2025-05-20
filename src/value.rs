use core::fmt;
use std::{
    cell::RefCell, fmt::Display, ops::{Add, Div, Mul}, rc::Rc
};

use crate::chunk::Chunk;

/// All Asterisk Values definition.
///
#[derive(Debug, Clone)]
pub struct Value {
    pub value: Primitive,
    pub _type: Type,
    pub modifier: Modifier,
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

#[derive(Debug)]
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

impl Default for Value {
    fn default() -> Self {
        Value {
            value: Primitive::Void(()),
            _type: Type::Void,
            modifier: Modifier::Unassigned,
        }
    }
}

/* Primitives are variable assigned data, Type are the check for this data to be valid throught the runtime */
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    Fn,
    Ref(Rc<Type>),
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Function(Function),
    Ref(Rc<RefCell<Value>>),
    RefMut(*mut Value),
    Void(()),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Int(_) => write!(f, "Int"),
            Primitive::Float(_) => write!(f, "Float"),
            Primitive::Bool(_) => write!(f, "Bool"),
            Primitive::String(_) => write!(f, "String"),
            // Primitive::Ref(primitive) => write!(f, "&({})", unsafe { (**primitive).clone() }),
            // Primitive::RefMut(primitive) => write!(f, "&({})", unsafe { (**primitive).clone() }),
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

impl Default for Modifier {
    fn default() -> Self {
        Modifier::Unassigned
    } 
}

crate::macros::gen_primitives_operations!(Float, Int);
crate::macros::gen_values_operations!(Int, Float);
