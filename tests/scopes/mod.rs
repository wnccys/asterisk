#[cfg(test)]
mod scopes {
    use asterisk::vm::Vm;
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}, rc::Rc};

    use crate::common::mk_parser;

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

        assert_eq!(parser.scopes.len(), 0);
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

        assert_eq!(parser.scopes.len(), 0);
        assert!(result.is_ok());
    }

    #[test]
    fn scopes_closures_single() {
        let mut vm = Vm::default();
        let source = "
            fn outer() {
                let mut x = 2;

                fn inner() {
                    x = x + 1;

                    print x;
                }
                
                inner();
            }

            outer();
        ";
        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser = parser.fun_declaration();
        assert_eq!(parser.scopes.len(), 0);

        parser = parser.statement();
        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();
    }

    // TODO
    fn scopes_closures_multi() {
        let source = "
            fn outer() {
                let mut x = 2;

                fn inner() {
                    x = x + 1;

                    print x;
                }

                fn f() {
                    x = x + 1;

                    print x;
                }
                
                inner();
                f();
            }

            outer();
        ";

        let parser = mk_parser(Cursor::new(source));
    }
}