mod common;
use common::mk_parser;

#[cfg(test)]
mod ops {
    use std::{cell::RefCell, io::Cursor, rc::Rc};

    use asterisk::{primitives::{primitive::Primitive, value::Value}, vm::Vm};

    use crate::common::mk_parser;

    #[test]
    fn sum_int_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(2),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(2),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 4);
    }

    #[test]
    fn sum_int_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(2),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(-2),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 0);
    }

    #[test]
    fn sum_float_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 4.0);
    }

    #[test]
    fn sum_float_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(-2.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("+");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 0.0);
    }

    #[test]
    fn mul_int_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 9);
    }

    #[test]
    fn mul_int_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(-3),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -9);
    }

    #[test]
    fn mul_float_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(5.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 10.0);
    }

    #[test]
    fn mul_float_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(2.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(-5.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("*");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -10.0);
    }

    #[test]
    fn div_int_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(12),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(3),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 4);
    }

    #[test]
    fn div_int_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Int(-30),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Int(10),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Int(i) => i,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -3);
    }

    #[test]
    fn div_float_positive() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(15.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(3.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, 5.0);
    }

    #[test]
    fn div_float_negative() {
        let mut vm = Vm::default();
        let a = Value {
            value: Primitive::Float(-12.0),
            ..Default::default()
        };

        let b = Value {
            value: Primitive::Float(3.0),
            ..Default::default()
        };
        vm.stack.push(Rc::new(RefCell::new(a)));
        vm.stack.push(Rc::new(RefCell::new(b)));

        let _ = vm.binary_op("/");

        let result = match vm.stack.pop().unwrap().borrow().value {
            Primitive::Float(f) => f,
            _ => panic!("Invalid output result.")
        };

        assert_eq!(result, -4.0);
    }

    /* Bool conditional ops */

    #[test]
    fn bool_or() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = true or false;
            let mut b = true || false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        let b = vm.globals.get(&"b".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(true));
        assert_eq!(b.borrow().value, Primitive::Bool(true));
    }

    #[test]
    fn bool_and() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = true and false;
            let mut b = true && false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();
        let b = vm.globals.get(&"b".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(false));
        assert_eq!(b.borrow().value, Primitive::Bool(false));
    }

    #[test]
    fn bool_negation_true_to_false() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = !true;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(false));
    }

    #[test]
    fn bool_negation_false_to_true() {
        let mut vm = Vm::default();
        let source = r"
            let mut a = !false;
        ";

        let mut parser = mk_parser(Cursor::new(source));
        // var decl
        parser = parser.declaration();

        vm.call(Rc::new(parser.end_compiler()), 0);
        vm.run().unwrap();

        let a = vm.globals.get(&"a".to_string()).unwrap();

        assert_eq!(a.borrow().value, Primitive::Bool(true));
    }
}