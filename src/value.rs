use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Float(f64),
    Int(i32),
    Bool(bool),
}

// REVIEW probably resolvable with macro
pub fn values_equal(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Bool(value_a), Value::Bool(value_b)) => Value::Bool(value_a == value_b),
        (Value::Int(value_a), Value::Int(value_b)) => Value::Bool(value_a == value_b),
        (Value::Float(value_a), Value::Float(value_b)) => Value::Bool(value_a == value_b),
        _ => panic!("only equal-types are allowed to be compared."),
    }
}

impl Copy for Value {}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
            _ => panic!("different type operations are not allowed"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
            _ => panic!("operation not allowed."),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
            _ => panic!("different type operations are not allowed"),
        }
    }
}
