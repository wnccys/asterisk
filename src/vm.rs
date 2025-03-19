use crate::chunk::*;
use crate::compiler::compile;
use crate::types::hash_table::HashTable;
use crate::utils::print::{
    print_stack, 
    print_value};
use crate::value::{values_equal, Value};

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
    pub fn interpret(&mut self, source: Vec<char>) -> InterpretResult {
        let (chunk, result) = compile(&mut self.strings, source);

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
                                .expect("Error on return: stack underflow."),
                        );
                    }

                    InterpretResult::Ok
                }
                OpCode::Negate => {
                    {
                        let chunk = self.chunk.as_mut();
                        let to_be_negated = chunk.stack.pop().unwrap();

                        match to_be_negated {
                            Value::Int(value) => chunk.stack.push(Value::Int(-value)),
                            Value::Float(value) => chunk.stack.push(Value::Float(-value)),
                            Value::Bool(value) => chunk.stack.push(Value::Bool(!value)),
                            _ => panic!("Operation not allowed."),
                        }
                    }

                    InterpretResult::Ok
                }
                OpCode::Not => {
                    let chunk = self.chunk.as_mut();
                    let to_be_negated = chunk.stack.pop().unwrap();

                    match to_be_negated {
                        Value::Bool(value) => chunk.stack.push(Value::Bool(!value)),
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
                    chunk.stack.push(Value::Bool(true));

                    InterpretResult::Ok
                }
                OpCode::False => {
                    let chunk = self.chunk.as_mut();
                    chunk.stack.push(Value::Bool(false));

                    InterpretResult::Ok
                }
                OpCode::Equal => {
                    let chunk = self.chunk.as_mut();
                    let a = chunk.stack.pop().unwrap();
                    let b = chunk.stack.pop().unwrap();

                    chunk.stack.push(values_equal(a, b));

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

                    print_value(chunk);

                    InterpretResult::Ok
                }
                OpCode::Pop => {
                    let chunk = self.chunk.as_mut();

                    chunk.stack.pop().expect("Error on pop: stack underflow.");

                    InterpretResult::Ok
                }
                // TODO Add correct nil value handling (not permitted)
                OpCode::Nil => {
                    self.chunk.stack.push(Value(Void(())));

                    InterpretResult::Ok
                },
                // Bring value from constants vector to stack
                OpCode::Constant(var_index) => {
                    let chunk = self.chunk.as_mut();
                    let constant = chunk.constants[var_index].clone();
                    chunk.stack.push(constant);

                    InterpretResult::Ok
                }
                // NOTE Check for duplicated variable
                // Get value from value position and load it into the top of stack,
                // this way other operations can interact with the value.
                OpCode::GetLocal(var_index) => {
                    let value = self.chunk.stack[var_index].clone();

                    self.chunk.stack.push(value);

                    InterpretResult::Ok
                }
                // Set new value to local variable.
                OpCode::SetLocal(var_index) => {
                    let temp_index = var_index;
                    let value = self.chunk.stack.last().unwrap().clone();

                    self.chunk.stack[temp_index] = value;

                    InterpretResult::Ok
                }
                // Get variable name from constants and assign it to globals vec
                OpCode::DefineGlobal(var_index) => {
                    let chunk = self.chunk.as_mut();
                    let var_name = chunk.constants[var_index].clone(); // (Value, Modifier)

                    match var_name {
                        Value::String(name) => {
                            self.globals.insert(&name, chunk.stack.pop().unwrap());
                        }
                        _ => panic!("Invalid global variable name."),
                    }

                    InterpretResult::Ok
                }
                // TODO Implement better global var get (No extra-const register)
                OpCode::GetGlobal(var_index) => {
                    let temp_index = var_index;
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[temp_index] {
                        Value::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

                    let value = match self.globals.get(name) {
                        Some(value) => value,
                        None => panic!("Use of undeclared variable '{}'", &name),
                    };

                    chunk.stack.push(value);

                    InterpretResult::Ok
                }
                // Re-assign to already set global variable.
                OpCode::SetGlobal(index) => {
                    let chunk = self.chunk.as_mut();

                    let name = match &chunk.constants[index] {
                        Value::String(name) => name,
                        _ => panic!("Invalid global variable name."),
                    };

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
                }
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
            ">" => self.chunk.stack.push(Value::Bool(a > b)),
            "<" => self.chunk.stack.push(Value::Bool(a < b)),
            _ => panic!("invalid operation."),
        }

        InterpretResult::Ok
    }
}
