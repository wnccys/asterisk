use std::rc::Rc;

use crate::chunk::*;
use crate::compiler::compile;
use crate::types::hash_table::HashTable;
use crate::utils::parse_type;
use crate::utils::print::{print_stack, print_value};
use crate::value::{Modifier, Primitive, Type, Value};

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm {
    chunk: Box<Chunk>,
    globals: HashTable<String>,
    strings: HashTable<String>,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            chunk: Box::default(),
            globals: HashTable::default(),
            strings: HashTable::default(),
        }
    }
}

impl Vm {
    /// Parse a Vec<char> into valid asterisk state.
    ///
    /// This function is the compiler itself, compile the source code into chunks and run it's emitted Bytecodes.
    ///
    pub fn interpret(&mut self, source_code: String) -> InterpretResult {
        let source_code = format!(
            "{}
        EOF",
            source_code
        );

        let (chunk, result) = compile(&mut self.strings, source_code);

        if result != InterpretResult::Ok {
            panic!("{:?}", result);
        }

        self.chunk = Box::new(chunk);
        self.run()
    }

    /// Loop throught returned Bytecode vector (code vec) handling it's behavior.
    ///
    fn run(&mut self) -> InterpretResult {
        let mut op_status = InterpretResult::CompileError;

        #[cfg(feature = "debug")]
        println!("Constants Vec: {:?}", self.chunk.constants);

        for i in 0..self.chunk.code.len() {
            #[cfg(feature = "debug")]
            {
                print_stack(&self.chunk);
                println!("current code: {:?}", self.chunk.code[i]);
            }

            op_status = match self.chunk.code[i].clone() {
                OpCode::Return => {
                    {
                        let chunk = self.chunk.as_mut();
                        print_value(
                            &chunk
                                .stack
                                .pop()
                                .expect("Error on return: stack underflow.")
                                .value,
                        );
                    }

                    InterpretResult::Ok
                }
                OpCode::Negate => {
                    {
                        let chunk = self.chunk.as_mut();
                        let to_be_negated = chunk.stack.pop().unwrap();

                        match to_be_negated {
                            Value {
                                value: Primitive::Int(value),
                                modifier,
                                _type,
                            } => chunk.stack.push(Value {
                                value: Primitive::Int(-value),
                                modifier,
                                _type,
                            }),
                            Value {
                                value: Primitive::Float(value),
                                modifier,
                                _type,
                            } => chunk.stack.push(Value {
                                value: Primitive::Float(-value),
                                modifier,
                                _type,
                            }),
                            Value {
                                value: Primitive::Bool(value),
                                modifier,
                                _type,
                            } => chunk.stack.push(Value {
                                value: Primitive::Bool(!value),
                                modifier,
                                _type,
                            }),
                            _ => panic!("Operation not allowed."),
                        }
                    }

                    InterpretResult::Ok
                }
                OpCode::Not => {
                    let chunk = self.chunk.as_mut();
                    let to_be_negated = chunk.stack.pop().unwrap();

                    match to_be_negated {
                        Value {
                            value: Primitive::Bool(value),
                            modifier,
                            _type,
                        } => chunk.stack.push(Value {
                            value: Primitive::Bool(!value),
                            modifier,
                            _type,
                        }),
                        _ => panic!("Value should be a boolean."),
                    }

                    InterpretResult::Ok
                }
                OpCode::Add => {
                    self.binary_op("+");

                    InterpretResult::Ok
                }
                OpCode::Multiply => {
                    self.binary_op("*");

                    InterpretResult::Ok
                }
                OpCode::Divide => {
                    self.binary_op("/");

                    InterpretResult::Ok
                }
                OpCode::True => {
                    let chunk = self.chunk.as_mut();
                    chunk.stack.push(Value {
                        value: Primitive::Bool(true),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    });

                    InterpretResult::Ok
                }
                OpCode::False => {
                    let chunk = self.chunk.as_mut();
                    chunk.stack.push(Value {
                        value: Primitive::Bool(false),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    });

                    InterpretResult::Ok
                }
                OpCode::Equal => {
                    let chunk = self.chunk.as_mut();
                    let a = chunk.stack.pop().unwrap();
                    let b = chunk.stack.pop().unwrap();

                    chunk.stack.push(Value {
                        value: Primitive::Bool(a == b),
                        modifier: Modifier::Unassigned,
                        _type: Type::Bool,
                    });

                    InterpretResult::Ok
                }
                OpCode::Greater => {
                    self.binary_op(">");

                    InterpretResult::Ok
                }
                OpCode::Less => {
                    self.binary_op("<");

                    InterpretResult::Ok
                }
                OpCode::Print => {
                    let chunk = &self
                        .chunk
                        .as_mut()
                        .stack
                        .pop()
                        .expect("Could not find value to print.");

                    print_value(&chunk.value);

                    InterpretResult::Ok
                }
                OpCode::Pop => {
                    let chunk = self.chunk.as_mut();

                    chunk.stack.pop().expect("Error on pop: stack underflow.");

                    InterpretResult::Ok
                }
                // Bring value from constants vector to stack
                OpCode::Constant(var_index) => {
                    let chunk = self.chunk.as_mut();
                    let constant = chunk.constants[var_index].clone();
                    let _type = parse_type(&constant);

                    chunk.stack.push(Value {
                        value: constant,
                        modifier: Modifier::Unassigned,
                        _type,
                    });

                    InterpretResult::Ok
                }
                /*
                    // NOTE Check for duplicated variable
                    Get value from value position and load it into the top of stack,
                    this way other operations can interact with the value.
                */
                OpCode::GetLocal(var_index) => {
                    let variable = self.chunk.stack[var_index].clone();

                    self.chunk.stack.push(variable);

                    InterpretResult::Ok
                }
                /*
                    Set new value to local variable.
                */
                OpCode::SetLocal(var_index, modifier) => {
                    let mut value = self.chunk.stack.last().unwrap().clone();
                    let variable = &mut self.chunk.stack[var_index];

                    if variable._type != value._type {
                        panic!("Cannot assign {:?} to {:?}", value._type, variable._type)
                    };
                    if modifier != Modifier::Mut {
                        panic!("Cannot assign to immutable variable.")
                    }

                    value.modifier = variable.modifier;

                    *variable = value;

                    InterpretResult::Ok
                }
                /*
                    Get variable name from constants and value from top of stack assigning it to globals HashMap
                */
                OpCode::DefineGlobal(var_name_index, modifier) => {
                    let chunk = self.chunk.as_mut();
                    let var_name = &chunk.constants[var_name_index];

                    let mut variable = match chunk.stack.pop().unwrap() {
                        /* This match only a dummy type specifier */
                        Value { value: Primitive::Void(..), _type, .. } => {
                            let value = chunk.stack.pop().unwrap();

                            if value._type != _type { panic!("Cannot assign {:?} to {:?}", value._type, _type) }

                            value
                        },
                        /* Inferred Type, so no match is needed */
                        v => v,
                    };
                    variable.modifier = modifier;

                    /*  Only strings are allowed to be var names */
                    match var_name {
                        Primitive::String(name) => {
                            self.globals.insert(name, variable);
                        }
                        _ => panic!("Invalid global variable name."),
                    }

                    InterpretResult::Ok
                }
                /*
                    TODO Implement better global var get (No extra-const register)
                */
                OpCode::GetGlobal(var_index) => {
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[var_index] {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let value = match self.globals.get(name) {
                        Some(value) => value,
                        None => panic!("Use of undeclared variable '{}'", &name),
                    };

                    chunk.stack.push(value.borrow().clone());

                    InterpretResult::Ok
                }
                /*
                    Re-assign to already set global variable.
                */
                OpCode::SetGlobal(name_index) => {
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[name_index] {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let variable = self.globals.get(name).unwrap();
                    if variable.borrow().modifier != Modifier::Mut {
                        panic!("Cannot assign to a immutable variable.")
                    }

                    let mut to_be_inserted = chunk.stack.last().unwrap().to_owned();

                    /* Check if type of dangling value are equal the to-be-assigned variable */
                    if variable.borrow()._type != to_be_inserted._type {
                        panic!(
                            "Error: Cannot assign {:?} to {:?} ",
                            to_be_inserted._type, variable.borrow()._type
                        );
                    }

                    to_be_inserted.modifier = variable.borrow().modifier;

                    if self.globals.insert(name, to_be_inserted) {
                        let _ = self.globals.delete(name);
                        panic!("Global variable is used before it's initialization.");
                    }

                    InterpretResult::Ok
                }
                /* Let var type information available on stack, this is used in explicit variable declaration */
                OpCode::SetType(t) => {
                    let dummy_value = Value { _type: t, ..Default::default() };
                    self.chunk.stack.push(dummy_value);

                    InterpretResult::Ok
                }
                /* Get var name from constants and craft a ref value based on globals' referenced Value */
                OpCode::SetRef(var_index) => {
                    // let referenced_value = self.chunk.stack.pop().unwrap().value;
                    let referenced_name = match self.chunk.constants[var_index].clone() {
                        Primitive::String(str) => str,
                        _ => panic!("Invalid var name reference."),
                    };

                    /* Get value to be referenced */
                    let referenced_value = self.globals.get(&referenced_name).unwrap_or_else(|| panic!("Invalid referenced value."));
                    let referenced_type = referenced_value.borrow()._type.clone();

                    let _ref = Value {
                        value: Primitive::Ref(referenced_value),
                        _type: Type::Ref(Rc::new(referenced_type)),
                        modifier: Modifier::Const
                    };

                    match self.chunk.stack.pop() {
                        Some(value) => {
                            match value {
                                Value { _type, .. } => { 
                                    if _type != _ref._type {
                                        dbg!(&_type, &_ref._type);
                                        panic!("Cannot assign {:?} to {:?}", _ref._type, _type)
                                    };
                                },
                            }
                        },
                        None => (),
                    }

                    self.chunk.stack.push(_ref);

                    InterpretResult::Ok
                }
            };
        }

        op_status
    }

    fn binary_op(&mut self, op: &str) -> InterpretResult {
        let b = self.chunk.stack.pop().expect("value b not loaded.");

        let a = self.chunk.stack.pop().expect("value a not loaded.");

        match op {
            "+" => self.chunk.stack.push(a + b),
            "*" => self.chunk.stack.push(a * b),
            "/" => self.chunk.stack.push(a / b),
            ">" => self.chunk.stack.push(Value {
                value: Primitive::Bool(a > b),
                modifier: a.modifier,
                _type: a._type,
            }),
            "<" => self.chunk.stack.push(Value {
                value: Primitive::Bool(a < b),
                modifier: a.modifier,
                _type: a._type,
            }),
            _ => panic!("invalid operation."),
        }

        InterpretResult::Ok
    }
}