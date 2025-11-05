use std::{cell::RefCell, rc::Rc};

use crate::primitives::value::Value;

/* Primitives are variable assigned data, Type is the contract for this data to be valid throught the runtime */
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    Struct,
    Tuple,
    Fn,
    // Dyn is resolved dynamically entirelly at VM's bytecode execution phase
    Dyn(Dyn),
    NativeFn,
    Closure,
    Ref(Rc<Type>),
    Void,
    #[default]
    UnInit,
}

#[derive(Default, Debug, Clone, PartialEq)]
// type struct's (name, type)
pub struct Dyn(pub Rc<RefCell<Value>>);

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
