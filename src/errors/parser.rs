use crate::primitives::functions::Function;

// ::<func | (word on source, error msg)>
pub type ParserResult = std::result::Result<Function, (&'static str, &'static str)>;
