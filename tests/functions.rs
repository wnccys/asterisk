mod common;
use common::{mk_parser};

#[cfg(test)]
mod functions {
    use asterisk::primitives::primitive::Primitive;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn fun_declaration_single_argument() {
        let source = r"
            fn f(n: Int) {}
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.fun_declaration();

        // Extract function from current parser's chunk
        let _fn = parser
            .function
            .chunk
            .constants
            .get(1).unwrap_or_else(|| panic!("Could not find function object"));

        let f = match _fn { 
            Primitive::Function(f) => f,
            f => panic!("{}", format!("Invalid function object: {:?}", f))
        };

        assert_eq!(f.arity, 1);
        assert_eq!(f.name, "f");
    }

    #[test]
    fn fun_declaration_multi_argument() {
        let source = r"
            fn g(n: Int, m: String, p: &Int, g: &String, b: Bool, c: Float, d: &Float) {}
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.fun_declaration();

        // Extract function from current parser's chunk
        let _fn = parser
            .function
            .chunk
            .constants
            .get(1).unwrap_or_else(|| panic!("Could not find function object"));

        let f = match _fn { 
            Primitive::Function(f) => f,
            f => panic!("{}", format!("Invalid function object: {:?}", f))
        };

        assert_eq!(f.arity, 7);
        assert_eq!(f.name, "g");
    }
}