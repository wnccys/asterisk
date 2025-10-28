use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::primitives::{types::{Modifier, Type}, value::Value};

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub field_indices: HashMap<String, (Type, usize)>,
    pub field_count: usize,
}

// Suitable for clone inspec
impl Clone for Struct {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            field_indices: self.field_indices.clone(),
            field_count: self.field_count,
        }
    }
}

impl Into<Value> for Struct {
    fn into(self) -> Value {
        Value {
            value: crate::primitives::primitive::Primitive::Struct(self),
            _type: Type::Struct,
            modifier: Modifier::Const
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub _struct: Rc<RefCell<Value>>,
    pub values: Vec<Value>
}

impl Into<Value> for Instance {
    fn into(self) -> Value {
        Value {
            value: crate::primitives::primitive::Primitive::Instance(self),
            _type: Type::Struct,
            modifier: Modifier::Const
        }
    }
}