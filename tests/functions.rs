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
        let sources: [&'static str; 2] = [
            r"
                fn f(n: Int) { n; } // 1
            ",
            r"
                let n = 2;
                f(n); // 2
            "
        ];
        let mut parser = mk_parser(Cursor::new(sources[0]));
        parser.advance();
        parser = parser.fun_declaration();

        // Extract function from current parser's chunk
        let _fn = parser
            .function
            .chunk
            .constants
            .get(1)
            .unwrap_or_else(|| panic!("Could not find function object."));

        let inner_fn = match _fn {
            Primitive::Closure {_fn, ..} => _fn,
            f => panic!("{}", format!("Invalid function object: {:?}", f))
        }.clone();

        assert_eq!(inner_fn.arity, 1);
        assert_eq!(inner_fn.name, "f");

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        // Verify fn arity and resolved object (match parser)
        match vm.globals.get(&inner_fn.name) {
            Some(f) => {
                match &Rc::clone(&f).borrow().value {
                    Primitive::Function(f) => {
                        if f.arity != inner_fn.arity {
                            panic!("Invalid arity of VM function callable object.") 
                        } 
                    },
                    _ => panic!("Invalid type for inner_fn.")
                }
            },
            None => panic!("Function was not declared.")
        };

        // 2
        let mut parser = mk_parser(Cursor::new(sources[1]));
        // var_declaration
        parser = parser.declaration();
        // expression (call)
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();
    }

    #[test]
    fn fun_declaration_multi_argument() {
        let mut vm = Vm::default();

        let sources: [&'static str; 2] = [
            r"
                fn f(n: Int, m: String, p: &Int, g: &String, b: Bool, c: Float, d: &Float) {
                    n; m; p; g; b; c; d;
                }
            ",
            r"
                let n: Int = 32;
                let m: String = 'str';
                let b: Bool = true;
                let c: Float = 1.0;
                let p: &Int = &n;
                let g: &String = &m;
                let d: &Float = &c;

                f(n, m, p, g, b, c, d);
            "
        ];
        let mut parser = mk_parser(Cursor::new(sources[0]));
        parser.advance();
        parser = parser.fun_declaration();

        // Extract function from current parser's chunk
        let _fn = parser
            .function
            .chunk
            .constants
            .get(1)
            .unwrap_or_else(|| panic!("Could not find function object."));

        let inner_fn = match _fn {
            Primitive::Closure {_fn, ..} => _fn,
            f => panic!("{}", format!("Invalid function object: {:?}", f))
        }.clone();

        assert_eq!(inner_fn.arity, 7);
        assert_eq!(inner_fn.name, "f");

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        // Verify fn arity and resolved object (match parser)
        match vm.globals.get(&inner_fn.name) {
            Some(f) => {
                match &Rc::clone(&f).borrow().value {
                    Primitive::Closure {_fn, ..} => {
                        if _fn.arity != inner_fn.arity {
                            panic!("Invalid arity of VM function callable object.") 
                        } 
                    },
                    _ => panic!("Invalid type for inner_fn.")
                }
            },
            None => panic!("Function was not declared.")
        };

        // 2
        let mut parser = mk_parser(Cursor::new(sources[1]));
        for _ in 0..7 {
            // var_declaration
            parser = parser.declaration();
        }
        // expression (call)
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();
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
        parser.advance();
        parser = parser.fun_declaration();
        parser.advance();
        parser.var_declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();

            let r = match vm.globals.get(&String::from("r")).expect("Invalid variable.").borrow().value {
                Primitive::Int(i) => i,
                _ => panic!("Invalid variable return type.")
            };

            assert_eq!(r, 55);
        }));

        assert!(result.is_ok())
    }

    fn fun_local_inner() {
        let source = "
            let x = \"global\";

            fn outer() {
                let x = \"inner\";
                fn inner() {
                    print x;
                }
                inner();
            }
            outer();
        ";
    }
}