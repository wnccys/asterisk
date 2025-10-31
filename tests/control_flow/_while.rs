mod _while {
    use std::{io::Cursor, rc::Rc};
    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};
    use crate::common::mk_parser;

    #[test]
    fn _while_basic() {
        let source = r"
            let mut n: Int = 0;

            while (n < 10) {
                n = n + 1;
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var_declaration()
        parser = parser.declaration();
        // while
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 10);
    }
}