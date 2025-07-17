mod common;
use common::{mk_parser};

#[cfg(test)]
mod scopes {
    use super::*;
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}};

    #[test]
    fn scopes_single() {
        let source: [&str; 1] = [
            r"
                {
                    let a = 2;
                }
            "
        ];

        let mut parser = mk_parser(Cursor::new(source[0]));
        let result = catch_unwind(AssertUnwindSafe(|| {
                parser.advance();
                parser.begin_scope();
                    parser.advance();
                    parser.var_declaration();

                    assert!(parser.scopes.last().unwrap().local_count == 1);
                    assert!(parser.scopes.len() == 1);
                parser.end_scope();
        }));

        assert!(result.is_ok());
    }

    #[test]
    fn scopes_multi() {
        let source: [&str; 1] = [
            r"
            {
                {
                    {
                        let a = 2;
                    }
                }
            }
            "
        ];

        let mut parser = mk_parser(Cursor::new(source[0]));
        let result = catch_unwind(AssertUnwindSafe(|| {
                parser.advance();
                parser.begin_scope();
                    parser.advance();
                    parser.begin_scope();
                        parser.advance();
                        parser.begin_scope();
                            parser.advance();
                            parser.var_declaration();

                            assert!(parser.scopes.last().unwrap().local_count == 1);
                            assert!(parser.scopes.len() == 3);
                        parser.end_scope();
                    parser.end_scope();
                parser.end_scope();
        }));

        assert!(result.is_ok());
    }
}