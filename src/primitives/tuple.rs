use crate::primitives::value::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct Tuple {
    pub items: Box<[Value]>
}