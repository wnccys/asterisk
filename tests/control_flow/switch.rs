mod switch {
    use std::{io::Cursor, rc::Rc};

    use asterisk::{primitives::{primitive::Primitive}, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn switch_no_default() {
        let source = r"
            let mut n: Int = 0;

            switch (n) {
                case (1) => {
                    n = 1;
                }
                // No parens needed
                case 0 => {
                    n = 2;
                }
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var_declaration()
        parser = parser.declaration();
        // switch
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 2);
    }

    #[test]
    fn switch_with_default() {
        let source = r"
            let mut n: Int = 10;

            switch (n) {
                case (1) => {
                    n = 1;
                }
                // No parens needed
                case 0 => {
                    n = 2;
                }
                default => {
                    n = 3;
                }
            }
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var_declaration()
        parser = parser.declaration();
        // switch
        parser = parser.declaration();

        let mut vm = Vm::default();
        vm.call(Rc::new(parser.end_compiler()), 0);

        vm.run().unwrap();

        let n = vm.globals.get(&"n".to_string()).unwrap().take();
        let n_value = match n.value {
            Primitive::Int(int) => int,
            _ => panic!("Invalid type")
        };

        assert_eq!(n_value, 3);
    }
}
