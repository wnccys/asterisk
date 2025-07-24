mod common;
use common::{mk_parser};

#[cfg(test)]
mod functions {
    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use super::*;
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}, rc::Rc};

    #[test]
    fn fun_declaration_single_argument() {
        let mut vm = Vm::default();
        let sources = r"
            fn f(n: Int) { n; }
            let n = 2;
            f(n);
        ";
        let mut parser = mk_parser(Cursor::new(sources));
        // fun_declaratio
        parser = parser.declaration();
        // var_declaration
        parser = parser.declaration();
        // stmt
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        // Verify fn arity and resolved object (match parser)
        let target_fn = match vm.globals.get(&String::from("f")) {
            Some(f) => {
                match &f.borrow().value {
                    Primitive::Function(f) => Rc::clone(f),
                    _ => panic!("Invalid type for fn object.")
                }
            },
            None => panic!("Function was not declared.")
        };

        assert_eq!(target_fn.arity, 1);
        assert_eq!(target_fn.name, "f");
    }

    #[test]
    fn fun_declaration_multi_argument() {
        let mut vm = Vm::default();

        let sources = r"
            fn f(n: Int, m: String, p: &Int, g: &String, b: Bool, c: Float, d: &Float) {
                n; m; p; // g; b; c; d;
            }

            let n: Int = 32;
            let m: String = 'str';
            let p: &Int = &n;
            let c: Float = 1.0;
            let b: Bool = true;
            let g: &String = &m;
            let d: &Float = &c;

            f(n, m, p, g, b, c, d);
        ";
        let mut parser = mk_parser(Cursor::new(sources));
        // fun declaration
        parser = parser.declaration();

        for _ in 0..3 {
            // var_declaration
            parser = parser.declaration();
        }

        // expression (call)
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        let val_fn= vm.globals.get(&String::from("f")).unwrap();

        let _fn = match &val_fn.borrow().value {
            Primitive::Function(_fn) => Rc::clone(_fn),
            _ => panic!("Invalid type for inner_fn.")
        };

        assert_eq!(_fn.arity, 7);
        assert_eq!(_fn.name, "f");
    }

    #[test]
    fn fun_recursive() {
        let mut vm = Vm::default();
        let source = r"
            fn fib(n: Int) {
                if (n < 2) { return n; }

                return fib(n - 1) + fib(n - 2);
            }

            let r = fib(10);
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // fun declaration
        parser = parser.declaration();
        // var declaration
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);

        // Check for correct fn return
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();

            let r = match vm
                .globals
                .get(&String::from("r"))
                .expect("Invalid variable.")
                .borrow().value {
                    Primitive::Int(i) => i,
                    _ => panic!("Invalid variable return type.")
                };

            assert_eq!(r, 55);
        }));

        assert!(result.is_ok())
    }

    fn fun_local_inner() {
        let mut vm = Vm::default();
        let source = "
            let x = 'global';

            fn outer() {
                let x = 'outside';

                fn inner() {
                    x = 'inner';
                }

                inner();
            }

            outer();
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // fn
        parser = parser.declaration();
        // stmt
        parser.declaration();


    }
}