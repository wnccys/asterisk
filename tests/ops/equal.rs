#[cfg(test)]
mod equal {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn equal() {
        let mut vm = Vm::default();
        let source = r"
            let a1 = 'str' == 'str';
            let a2 = 'str' == 'strx';
            let b1 = 2 == 2;
            let b2 = 2 == 1;
            let c1 = 2.0 == 2.0;
            let c2 = 2.0 == 3.0;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        parser = parser.declaration();
        parser = parser.declaration();
        parser = parser.declaration();
        parser = parser.declaration();
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a1 = vm.globals.get(&"a1".to_string()).unwrap().take();
        let a2 = vm.globals.get(&"a2".to_string()).unwrap().take();
        let b1 = vm.globals.get(&"b1".to_string()).unwrap().take();
        let b2 = vm.globals.get(&"b2".to_string()).unwrap().take();
        let c1 = vm.globals.get(&"c1".to_string()).unwrap().take();
        let c2 = vm.globals.get(&"c2".to_string()).unwrap().take();

        assert_eq!(a1.value, Primitive::Bool(true));
        assert_eq!(a2.value, Primitive::Bool(false));
        assert_eq!(b1.value, Primitive::Bool(true));
        assert_eq!(b2.value, Primitive::Bool(false));
        assert_eq!(c1.value, Primitive::Bool(true));
        assert_eq!(c2.value, Primitive::Bool(false));
    }
}