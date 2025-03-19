use std::ops::{Add, Div, Mul};

/// Asterisk types definition.
/// 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Value {
    pub value: Primitive,
    pub modifier: Modifier,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Void(()),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Modifier {
    Const,
    Let,
    LetMut,
}

pub enum RefType {
    Owned,
    Ref,
    MutRef,
}

crate::macros::gen_primitives_equal!(
    Float(f64),
    Int(i32),
    Bool(bool),
    String(String),
    Void(())
);

crate::macros::gen_values_operations!(

);


impl Add for Primitive {
    type Output = PrimitiveValue;

    fn add(self, other: PrimitiveValue) -> PrimitiveValue {
        match (self, other) {
            (PrimitiveValue::Float(a), PrimitiveValue::Float(b)) => PrimitiveValue::Float(a + b),
            (PrimitiveValue::Int(a), PrimitiveValue::Int(b)) => PrimitiveValue::Int(a + b),
            (PrimitiveValue::String(str1), PrimitiveValue::String(str2)) => PrimitiveValue::String(str1.add(&str2[..])),
            _ => panic!("operation add not allowed."),
        }
    }
}

impl Mul for PrimitiveValue {
    type Output = PrimitiveValue;

    fn mul(self, other: PrimitiveValue) -> PrimitiveValue {
        match (self, other) {
            (PrimitiveValue::Float(a), PrimitiveValue::Float(b)) => PrimitiveValue::Float(a * b),
            (PrimitiveValue::Int(a), PrimitiveValue::Int(b)) => PrimitiveValue::Int(a * b),
            _ => panic!("operation mult not allowed."),
        }
    }
}

impl Div for PrimitiveValue {
    type Output = PrimitiveValue;

    fn div(self, other: PrimitiveValue) -> PrimitiveValue {
        match (self, other) {
            (PrimitiveValue::Float(a), PrimitiveValue::Float(b)) => PrimitiveValue::Float(a / b),
            (PrimitiveValue::Int(a), PrimitiveValue::Int(b)) => PrimitiveValue::Int(a / b),
            _ => panic!("operation divide not allowed."),
        }
    }
}
