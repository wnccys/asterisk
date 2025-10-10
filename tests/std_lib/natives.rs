#[cfg(test)]
mod native {
    use asterisk::vm::Vm;
    use crate::common::mk_parser;
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}, rc::Rc};

    #[test]
    fn call_no_args() {
        let mut vm = Vm::default();
        vm.init_std_lib();

        let source = r"
            print duration();
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser = parser.statement();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let result = catch_unwind(AssertUnwindSafe(||{
            let _ = vm.run();
            // Ensure stack is clean
            assert!(vm.stack.len() == 0);
        }));

        assert!(result.is_ok());
    }

    #[test]
    fn call_with_args() {
        let mut vm = Vm::default();
        vm.init_std_lib();

        let source = r"
            let n = 2;
            print typeof(n);
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();
        parser = parser.statement();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();
            // Only the local variable rests on stack
            assert!(vm.stack.len() == 1);
        }));

        assert!(result.is_ok());
    }

    #[test]
    fn call_with_args_multi() {
        let mut vm = Vm::default();
        vm.init_std_lib();

        let source = r"
            let n = 2;
            print typeof(n);
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.var_declaration();
        parser = parser.statement();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let result = catch_unwind(AssertUnwindSafe(|| {
            let _ = vm.run();
            // Only the local variable rests on stack
            assert!(vm.stack.len() == 1);
        }));

        assert!(result.is_ok());
    }
}