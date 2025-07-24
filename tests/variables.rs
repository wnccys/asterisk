mod common;
use common::mk_parser;

#[cfg(test)]
pub mod variables {
    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use super::*;
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}, rc::Rc};

    #[test]
    fn var_declaration_immut() {
        let mut vm = Vm::default();
        let sources: [&'static str; 2] = [
            r"
                let a = 32; // 1
            ",
            r"
                a = 2; // 2
            "
        ];

        let mut parser = mk_parser(Cursor::new(sources[0]));
        parser.advance();
        parser.var_declaration();

        // "a" and "32"
        assert_eq!(parser.function.chunk.constants.len(), 2);
        // No locals were added
        assert_eq!(parser.scopes.len(), 0);

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        let var_value = match &vm.globals
            .get(&"a".to_string())
            .expect("Variable is not available on VM.")
            .borrow().value {
                Primitive::Int(i) => *i,
                _ => panic!("Could not find variable by name.")
            };

        assert_eq!(var_value, 32);

        let mut parser = mk_parser(Cursor::new(sources[1]));
        parser = parser.statement();

        vm.call(Rc::new(parser.end_compiler()), 0);

        let r = catch_unwind(
            AssertUnwindSafe(|| {
                let _ = vm.run();
            }
        ));

        assert!(r.is_err());
    }

    #[test]
    fn var_declaration_mut() {
                let mut vm = Vm::default();
        let sources: [&'static str; 2] = [
            r"
                let mut a = 32; // 1
            ",
            r"
                a = 2; // 2
            "
        ];

        // 1
        let mut parser = mk_parser(Cursor::new(sources[0]));
        parser.advance();
        parser.var_declaration();

        // "a" and "32"
        assert_eq!(parser.function.chunk.constants.len(), 2);
        // No locals were added
        assert_eq!(parser.scopes.len(), 0);

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        let var_value = match &vm.globals
            .get(&"a".to_string())
            .expect("Variable is not available on VM.")
            .borrow().value {
                Primitive::Int(i) => *i,
                _ => panic!("Could not find variable by name.")
            };

        assert_eq!(var_value, 32);

        // 2
        let mut parser = mk_parser(Cursor::new(sources[1]));
        parser = parser.statement();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();

        let var_value = match &vm.globals
            .get(&"a".to_string())
            .expect("Variable is not available on VM.")
            .borrow().value {
                Primitive::Int(i) => *i,
                _ => panic!("Could not find variable by name.")
            };

        assert_eq!(var_value, 2);
    }

    #[test]
    fn var_declaration_immut_local() {
        let mut vm = Vm::default();
        let source = r"
            {
                let a = 32;
                a = 2;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.begin_scope();
        parser = parser.block();
        parser.end_scope();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let result = catch_unwind(AssertUnwindSafe(|| {
            match vm.run() {
                Ok(_) => (),
                Err(e) => { panic!("{:?}", e)}
            }
        }));

        assert!(result.is_err());
    }

    #[test]
    fn var_declaration_mut_local() {
        let mut vm = Vm::default();
        let source = r"
            {
                let mut a = 32;
                a = 2;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.begin_scope();
        parser = parser.block();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();
        }));
        let var_value = match vm.stack[0].borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid value on local slot.")
        };

        assert!(var_value == 2);
        assert!(result.is_ok());
    }

    #[test]
    fn var_fun_as_callable_value() {
        let mut vm = Vm::default();
        let source = r"
            fn f(){ print 'FROM FN!!'; }

            let a = f;

            print a;
            a();
        ";

        let mut parser = mk_parser(Cursor::new(source));
        vm.call(Rc::new(parser.end_compiler()), 0);

        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();
        }));

        assert!(result.is_ok());
    }
}