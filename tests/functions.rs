mod common;
use common::{mk_parser};

#[cfg(test)]
mod functions {
    use asterisk::{primitives::primitive::Primitive, vm::Vm};

    use super::*;
    use std::{io::Cursor, rc::Rc};

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
        parser.fun_declaration();

        // Extract function from current parser's chunk
        let _fn = parser
            .function
            .chunk
            .constants
            .get(1)
            .unwrap_or_else(|| panic!("Could not find function object."));

        let inner_fn = match _fn {
            Primitive::Function(f) => f,
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
        parser.declaration();
        // expression (call)
        parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        let _ = vm.run();
    }

    #[test]
    fn fun_declaration_multi_argument() {
        let source = r"
            fn g(n: Int, m: String, p: &Int, g: &String, b: Bool, c: Float, d: &Float) {}
        ";

        let mut parser = mk_parser(Cursor::new(source));
        parser.advance();
        parser.fun_declaration();

        // Extract function from current parser's chunk
        let _fn = parser
            .function
            .chunk
            .constants
            .get(1).unwrap_or_else(|| panic!("Could not find function object"));

        let f = match _fn { 
            Primitive::Function(f) => f,
            f => panic!("{}", format!("Invalid function object: {:?}", f))
        };

        assert_eq!(f.arity, 7);
        assert_eq!(f.name, "g");
    }
}