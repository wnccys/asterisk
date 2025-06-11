use std::{any::Any, cell::RefCell, env::Args, rc::Rc};

use crate::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct NativeFn {
    pub arity: usize,
    pub _fn: fn(&[Rc<RefCell<Value>>]) -> Value
}

impl NativeFn {
    pub fn call(&mut self, args: &[Rc<RefCell<Value>>]) -> Value {
        if args.len() != self.arity { panic!("Expect {} but got {} arguments.", args.len(), self.arity) }

        (self._fn)(args)
    }
}
