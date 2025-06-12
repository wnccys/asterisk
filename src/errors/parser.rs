use crate::primitives::primitive::Function;

pub type ParserResult = std::result::Result<Function, Vec<(u32, &'static str)>>;