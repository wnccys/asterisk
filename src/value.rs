use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
    Int(i32),
    Bool(bool),
}

pub fn values_equal(a: Value, b: Value) -> Value {
    if a != b {
        panic!("comparison are only allowed between 2 equal types.");
    }

    Value::Bool(true)
}

impl Copy for Value {}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            _ => panic!("different type operations are now allowed"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => panic!("operation now allowed."),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            _ => panic!("different type operations are now allowed"),
        }
    }
}
