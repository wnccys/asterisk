#[cfg(test)]
mod local {
    use asterisk::{primitives::primitive::Primitive, vm::Vm};
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}, rc::Rc};

    use crate::common::mk_parser;

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
}