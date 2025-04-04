# The compiler's shades
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

Below is present a basic workflow of the function calls as well as how the stack, the code array, and the constants array behavior when the above code is executed:

NOTE: OpCodes disassemble formatting as seen below in constants print is {bytecode}{index of variable received for}{value}.

``` rust

0000 0 OP_CONSTANT        1 32
0001 | OP_DEFINE_GLOBAL   0 a

    Var identifier is reached by **advance()**, then, in **parse_variable()** the identifier (variable name) is consumed getting the Token's name and set it up in **Constants** vector.
    After that the **Compiler** check for equal sign which in this case match, consuming it and calling **expression()** which execute the recursive ruler which evaluate the expressions and set them in **Stack**,
    which in our specific case do the following: call **parse_precedence(Precedence::Assignment)** which advance Token (now previous is Number and current is Semicolon),
    executing the prefix of previous which set a value (Int or Float) to the **Constants** vector returning to our **parse_precedence** call, which also set a **can_assign** variable that check if the precedence
    we passed to function is equal or not to Assignment which in our case is true, so variable can be assigned. Next we enter a loop, where while the precedence we passed firstly to **parse_precedence** (Assignment)
    is lower than the current Token (Semicolon which is none) the condition is false, so we don't execute nothing. Now we are back in **var_declaration** function,
    we consume Semicolon Token and define the global variable (in this case 'a') by passing the index of the value already read and set to constants to the **DefineGlobal(usize)**
    which take the value in the specified index (variable name) on constants and set it to globals HashTable using the element in the top of stack. The general order is finally: 
    The name of variable are load into **Constants** vector, number rule is found, executed and a **OpConstant(index)** is emitted carrying the index of the new variable pushed to **Constants**
    the **OpDefineGlobal(index)** Bytecode is set carrying the index of the variable's name, the Bytecode takes the name of variable and set the globals HashTable<variable name, **stack.pop()**> 
    as the value is already set in the **Stack** by the previous **OpCode::Constant(_)**.

0002 | OP_GET_GLOBAL      2 a
0003 | OP_PRINT

    After that we are back in **compile()** initial loop which calls **declaration()** which matches **statement()** call, which matches our current Token **Print** that parses the expression in front of the Token, 
    calling **expression()** that advances the Token once more (Print match advanced too), now we got Identifier as previous and ';' as current, so we execute the prefix of Identifier which calls **variable** rule.
    This rule check for local variables, if not local (our case now), **identifier_constant()** is called, getting the variable name from Token, emmiting a **Constant(var_index)** which set our variable name in **Constants**
    and set it to **Stack**. After that it emit a **GetGlobal(usize)** Bytecode, which will get this name as we passed its index, and get it from globals HashTable and set it to the stack, next we are back to 
    **parse_precedence()** where our current Token is still ';' Semicolon, we check for it's precedence, if it's higher of equal **Assignment**, what is false, then we don't execute nothing and return to our **Print** 
    statement which consume our actual Token ';' correctly and emit the **Print** Bytecode, which **pop()** a value from stack, and print it to STDOUT.

0004 | OP_CONSTANT        4 50
0005 | OP_DEFINE_GLOBAL   3 b
0006 | OP_GET_GLOBAL      5 b
0007 | OP_PRINT

    These one's are identical behaviorally to the 4 first Bytecodes!

0008 | OP_CONSTANT        6 x here!!!!
0009 | OP_CONSTANT        7 x is NOT HERE ANYMORE!!
0010 | OP_SET_LOCAL      Constant(1)
0011 | OP_POP

    Next, we're back to the compiler loop, where our previous are Semicolon and current is now LeftBrace, which match one of the declaration statements, advance Token (previous: LeftBrace, current: Var) 
    and calls **begin_scope()**, **block()** and **end_scope()**; **begin_scope()** increment compiler's scope_depth and **block()** is called, which while "}" or EOF isn't reached, 
    iit keeps calling the **declaration()** function which match the **declaration()** Var branch as our Token advanced on match previous LeftBrace match, advance the Token once more (previous: Var, current: Identifier),
    **var_declaration()** are called consuming variable's identifier, returning early (no Constants are generated) as we are doing with a local variable, consuming Identifier (our current Token) and 
    advancing the Token (previous: Identifier, current: Equal); 
    After that we enter the local_variable declaration conditional after we check if Compiler scope_depth == 0 (check if variable is global, which in our case is false) so we go into add_local,
    where we set a variable local Token to Compiler.locals, set it's correct depth and increase it's locals count; Next Equal Token is match which advance our Token (previous: Equal, current: String),
    **expression()** is called, advancing once more (previous: String, current: Semicolon), and our Token are parsed, Number prefix are executed, can_assign is true so it's value lies on Stack,
    Semicolon is consumed so now we got (previous: Semicolon, current: Identifier); Now we're back at Compiler's **declaration()** where we match the statement fallback and further **expression_statement()** are called
    which calls **expression()**, advancing our Token (previous: Identifier, current: Equal), can_assign is true and executing the previous identifier (variable which calls named_variable),
    we call **resolve_local()** which reverselly search for local variable, comparing the previous Token name with all of the tokens present in compiler.locals returning it's index,
    next we check if the index is -1 (fallback for local not found), set a tuple of proper methods, and in our case we match Equal (true) advancing Token (previous: Equal, current: String),
    parsing our expression (previous: String, current: Semicolon), which set our string at the top of the Stack and a emit **OpCode::SetLocal(var_index)**; Next, we're finally back at **expression_statement()**, which consume our Semicolon and emit a **OpCode::Pop** bytecode. (previous: Semicolon, current: Print);

0012 | OP_GET_LOCAL      Constant(1)
0013 | OP_PRINT
0014 | OP_POP
```

Wow, that's a lot. Take your time to think a little about all the code architecture which is involved in parsing this "simple code".


### Basic Mathematical Operation Flow