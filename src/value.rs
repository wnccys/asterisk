use std::ops::{Add, Mul, Div};

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i32),
}

impl Copy for Value {}

impl Value {}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match(self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a+b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a+b),
            _ => panic!("different type operations are now allowed"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a*b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a*b),
            _ => panic!("different type operations are now allowed"),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a/b),
            (Value::Int(a), Value::Int(b)) => Value::Int(a/b),
            _ => panic!("different type operations are now allowed"),
        }
    }
}