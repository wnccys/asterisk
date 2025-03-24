use crate::chunk::*;
use crate::compiler::compile;
use crate::types::hash_table::HashTable;
use crate::utils::print::{
    print_stack, 
    print_value};
use crate::value::{Modifier, Primitive, Value};

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
        let source_code = format!("{}
        EOF", source_code);

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
                println!("current code: {:?}", opcode);
                print_stack(self.chunk.as_ref());
            }

            op_status = match self.chunk.code[i] {
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
                            Value { value: Primitive::Int(value), modifier } => chunk.stack.push(Value { value: Primitive::Int(-value), modifier }),
                            Value { value: Primitive::Float(value), modifier } => chunk.stack.push(Value { value: Primitive::Float(-value), modifier }),
                            Value { value: Primitive::Bool(value), modifier } => chunk.stack.push(Value { value: Primitive::Bool(!value), modifier }),
                            _ => panic!("Operation not allowed."),
                        }
                    }

                    InterpretResult::Ok
                }
                OpCode::Not => {
                    let chunk = self.chunk.as_mut();
                    let to_be_negated = chunk.stack.pop().unwrap();

                    match to_be_negated {
                        Value { value: Primitive::Bool(value), modifier } => chunk.stack.push(Value { value: Primitive::Bool(!value), modifier }),
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
                    chunk.stack.push(Value { value: Primitive::Bool(true), modifier: Modifier::Unassigned });

                    InterpretResult::Ok
                }
                OpCode::False => {
                    let chunk = self.chunk.as_mut();
                    chunk.stack.push(Value { value: Primitive::Bool(false), modifier: Modifier::Unassigned } );

                    InterpretResult::Ok
                }
                OpCode::Equal => {
                    let chunk = self.chunk.as_mut();
                    let a = chunk.stack.pop().unwrap();
                    let b = chunk.stack.pop().unwrap();

                    chunk.stack.push(Value { value: Primitive::Bool(a == b), modifier: Modifier::Unassigned });

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
                // TODO Add correct nil value handling (not permitted)
                OpCode::Nil => {
                    self.chunk.stack.push(Value { value: Primitive::Void(()), modifier: Modifier::Unassigned });

                    InterpretResult::Ok
                },
                // Bring value from constants vector to stack
                OpCode::Constant(var_index) => {
                    let chunk = self.chunk.as_mut();
                    let constant = chunk.constants[var_index].clone();
                    chunk.stack.push( Value { value: constant, modifier: Modifier::Unassigned });

                    InterpretResult::Ok
                }
                /* 
                    // NOTE Check for duplicated variable
                    Get value from value position and load it into the top of stack,
                    this way other operations can interact with the value.
                */
                OpCode::GetLocal(var_index) => {
                    let value = self.chunk.stack[var_index].clone();

                    self.chunk.stack.push(value);

                    InterpretResult::Ok
                }
                /*
                    Set new value to local variable.
                */
                OpCode::SetLocal(var_index) => {
                    let temp_index = var_index;
                    let value = self.chunk.stack.last().unwrap().clone();

                    self.chunk.stack[temp_index] = value;

                    InterpretResult::Ok
                }
                /* 
                    Get variable name from constants and assign it to globals vec
                    Check for variable assignment
                */
                OpCode::DefineGlobal(var_index, modifier) => {
                    let chunk = self.chunk.as_mut();
                    let var_name = chunk.constants[var_index].clone();
                    let mut var_value = chunk.stack.pop().unwrap();
                    var_value.modifier = modifier.clone();

                    match var_name {
                        Primitive::String(name) => {
                            self.globals.insert(&name, var_value);
                        }
                        _ => panic!("Invalid global variable name."),
                    }

                    InterpretResult::Ok
                }
                /*
                    TODO Implement better global var get (No extra-const register)
                */
                OpCode::GetGlobal(var_index) => {
                    let temp_index = var_index;
                    let chunk = self.chunk.as_mut();


                    let name = match &chunk.constants[temp_index] {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let value = match self.globals.get(name) {
                        Some(value) => value,
                        None => panic!("Use of undeclared variable '{}'", &name),
                    };

                    chunk.stack.push(value);

                    InterpretResult::Ok
                }
                /*
                    Re-assign to already set global variable.
                */
                OpCode::SetGlobal(index) => {
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[index] {
                        Primitive::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let is_mut = self.globals.get(name).unwrap().modifier == Modifier::Mut;
                    if !is_mut { panic!("Cannot assign to a immutable variable.") }

                    if self
                        .globals
                        .insert(
                                name, 
                                chunk
                                    .stack
                                    .iter()
                                    .last()
                                    .unwrap()
                                    .to_owned()
                                )
                    {
                        let _ = self.globals.delete(name);
                        panic!("Global variable is used before it's initialization.");
                    }

                    InterpretResult::Ok
                },
            };
        }

        op_status
    }

    fn binary_op(&mut self, op: &str) -> InterpretResult {
        let b = self
            .chunk
            .stack
            .pop()
            .expect("value b not loaded.");

        let a = self
            .chunk
            .stack
            .pop()
            .expect("value a not loaded.");

        match op {
            "+" => self.chunk.stack.push(a + b),
            "*" => self.chunk.stack.push(a * b),
            "/" => self.chunk.stack.push(a / b),
            ">" => self.chunk.stack.push(Value { value: Primitive::Bool(a > b), modifier: a.modifier }),
            "<" => self.chunk.stack.push(Value { value: Primitive::Bool(a < b), modifier: a.modifier }),
            _ => panic!("invalid operation."),
        }

        InterpretResult::Ok
    }
}