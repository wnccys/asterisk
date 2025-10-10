mod global;
mod local;

#[cfg(test)]
mod variables {
    use asterisk::{vm::Vm};
    use std::{io::Cursor, panic::{catch_unwind, AssertUnwindSafe}, rc::Rc};
    use crate::common::mk_parser;

    #[test]
    fn var_fun_as_callable_value() {
        let mut vm = Vm::default();
        let source = r"
            fn f() { print 'FROM FN!!'; }

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