use asterisk::{parser::{lexer::Lexer, Parser}, primitives::functions::{Function, FunctionType}};

/// Crafts a default parser given a source
/// 
pub fn mk_parser<R: std::io::Read>(source: R) -> Parser<R> {
    let mut p = Parser::new(
        Function::default(),
        FunctionType::Script,
        Lexer::new(source)
    );
    p.advance();
    p
}