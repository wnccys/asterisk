mod common;
use common::mk_parser;

#[cfg(test)]
pub mod variables {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn var_declaration_immut() {
        let source = r"
            let a = 32;
            a = 2;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        // "a" and "32"
        assert_eq!(parser.function.chunk.constants.len(), 2);
        // No locals were added
        assert_eq!(parser.scopes.len(), 0);

        parser.statement();
    }

    #[test]
    fn var_declaration_mut() {
        let source = r"
            let a = 32;
            a = 2;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();

        // "a" and "32"
        assert_eq!(parser.function.chunk.constants.len(), 2);
        // No locals was added
        assert_eq!(parser.scopes.len(), 0);
    }

    #[test]
    fn var_declaration_immut_local() {
        let source = r"
            {
                let a = 32;
                a = 2;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.begin_scope();
        parser.block();

        // "a" and "32"
        assert_eq!(parser.function.chunk.constants.len(), 2);
        assert_eq!(parser.scopes.len(), 1);

        // parser.statement();
    }

    #[test]
    fn var_declaration_mut_local() {
        let source = r"
            {
                let mut a = 32;
                a = 2;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.begin_scope();
        parser.block();
        // Block are not uninitialized

        // "a" and "32"
        assert_eq!(parser.function.chunk.constants.len(), 2);
        assert_eq!(parser.scopes.len(), 1);
    }
}