use std::ops::{Add, Div, Mul};

/// Asterisk types definition.
///
#[derive(Debug, Clone)]
pub struct Value {
    pub value: Primitive,
    pub ref_type: RefType,
    pub modifier: Modifier,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Ref(Box<Primitive>),
    Void(()),
}

impl Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Int(_) => write!(f, "Int"),
            Primitive::Float(_) => write!(f, "Float"),
            Primitive::Bool(_) => write!(f, "Bool"),
            Primitive::String(_) => write!(f, "String"),
            Primitive::Ref(primitive) => write!(f, "&({})", primitive),
            _ => panic!("This type does not implement the format trait.")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Unassigned,
    Const,
    Mut,
}

#[derive(Debug, Clone)]
pub enum RefType {
    Owned,
    Ref,
    MutRef,
}

crate::macros::gen_primitives_operations!(Float(f64), Int(i32));

crate::macros::gen_values_operations!(Int(i32), Float(f64));
