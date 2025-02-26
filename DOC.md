# The compiler shades
This document exposes some shades the compiler/language itself has as well as other general topics as the language grammar and the compiler architecture.

## Topics
- The Language Itself
- The Language Grammar
- Basic Var Assign Flow
- Basic Mathematical Operation Flow


### The Language Itself
asterisk is a lightweight scripting language; It's code is compiled to a bytecode representation which the availables are:

- Return,
- Constant(usize),
- True,
- False,
- Equal,
- Nil,
- Pop,
- Greater,
- Less,
- Not,
- Add,
- Multiply,
- Divide,
- Negate,
- Print,
- DefineGlobal(usize),
- SetGlobal(usize),
- GetGlobal(usize),
- GetLocal(usize),
- SetLocal(usize),

To execute asterisk code a .ask file is required or the language REPL can be used.
Usage: 

    Read From File: cargo run -- [path-to-file].ask

    REPL: cargo run

### The Language Grammar
asterisk uses a basic

```rust
let sum = 32 + 4;
```

### The Compiler 
Currently, asterisk can compile mathematical expresions with order operation compatibility so 
```rust
(-1 + 2) * 3 - -4
```
evaluates to **7** following the correct mathematical order.

### Basic Var Assign Flow
```rust
let a = 32;
print a;

let b = 50;
print b;

{
    let x = "x here!!!!";
    x = "x is NOT HERE ANYMORE!!";
    print x;
}
```

Next, we present a basic workflow of the function calls as well as how the stack, the code array, and the constants array behaviors when the above code is executed; Below are the Bytecodes emmited:

NOTE: OpCodes disassemble formatting as seen below in constants print is {bytecode}{bytecode_index}{value}.

0000 0 OP_CONSTANT        1 32
0001 | OP_DEFINE_GLOBAL   0 a

    Var identifier is reached by **advance()**, them, in **parse_variable()**, the identifier (variable name) is consumed and the global variable is set, getting the Token's name and setting in in **Constants** vector. After that the **Compiler** check for equal sign which in this case match, consuming it and calling **expression()** which execute the recursive ruler which evaluate the expressions and set them in **Stack**, which in our specific case do the following: call **parse_precedence(Precedence::Assignment)** which advance Token (previous is Number and current is Semicolon) executing the prefix of previous which set a value (Int or Float) to the **Constants** vector returning to our **parse_precedence** call, which also set a **can_assign** variable that check if the precedence we passed to function is equal or not to Assignment which in our case is true, so variable can be assigned. Next we enter a loop, where while the precendece we passed firstly to **parse_precedence** (Assignment) is lower than the current token (Semicolon which is none) the condition is false, so we don't execute nothing. Now we are back in **var_declaration** function, we consume Semicolon Token and define the global variable (in this case 'a') by passing the index of the value already read and set to constants to the **DefineGlobal(usize)** which take the value in the specified index on constants and set it to globals HashTable.

0002 | OP_GET_GLOBAL      2 a
0003 | OP_PRINT
0004 | OP_CONSTANT        4 50
0005 | OP_DEFINE_GLOBAL   3 b
0006 | OP_GET_GLOBAL      5 b
0007 | OP_PRINT
0008 | OP_CONSTANT        6 x here!!!!
0009 | OP_CONSTANT        7 x is NOT HERE ANYMORE!!
0010 | OP_SET_LOCAL      Constant(1)
0011 | OP_POP
0012 | OP_GET_LOCAL      Constant(1)
0013 | OP_PRINT
0014 | OP_POP

Wow, that's a lot. Take your time to think a little about all the code architecture which is involved in parsing this "simple code".


### Basic Mathematical Operation Flow