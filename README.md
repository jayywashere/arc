# ARC

> a small stack-based bytecode virtual machine written in Rust because supposedly writing normal programs was not enough

## About

`ARC` (**Abstract Runtime Core**) is a small stack-based bytecode virtual machine written in `Rust`!

i made this mainly to practice working with:

- bytecode
- virtual machine design
- stack-based execution
- instruction sets
- constant pools
- runtime values
- error handling
- structuring a reusable Rust library

the main idea is for ARC to act as a reusable runtime for different frontends and languages.

for example, a future language such as `TASM` (a frontend i'll be making soon) could compile its instructions into ARC bytecode and let ARC handle the actual execution. (AND `Spark` AS WELL!!!)

## Features

- stack-based bytecode execution
- constant pool
- integer arithmetic
- floating-point arithmetic
- division and modulo
- basic stack operations
- `Print` opcode
- runtime error handling (`runtime::RuntimeError`)
- bytecode builder API
- method chaining because writing everything manually gets old very quickly and that can cause my brain to deteriorate at the speed of light
- `Print` and `DebugPrint` opcodes
- MORE runtime error handling (`runtime::RuntimeError`)
- `.arx` bytecode binaries
- bytecode serialization and deserialization
- bytecode builder API
- runtime values! new one is `Char`: (`Int`, `Float`, `Bool`, `Char`, and `String`)

## Supported Instructions

| **Instruction** | **Operation**                                                     |
| --------------- | ----------------------------------------------------------------- |
| `Halt`          | stops execution                                                   |
| `Ldc`           | loads a constant onto the stack                                   |
| `Pop`           | removes the top value from the stack                              |
| `Dup`           | duplicates the top value                                          |
| `Add`           | addition                                                          |
| `Sub`           | subtraction                                                       |
| `Mul`           | multiplication                                                    |
| `Div`           | division                                                          |
| `Rem`           | modulo                                                            |
| `Print`         | prints and removes the top value using its display representation |
| `DebugPrint`    | prints and removes the top value using its debug representation   |

(wasted most of my dang time centering and formatting this freaking table just to find out that markdown formats the final look perfectly.....)

## How It Works

ARC uses a stack-based execution model!! (it's literally a stack-based bytecode virtual machine...)

instead of using registers (like register-based bytecode virtual machine.. such as Lua VM, BEAM/Erlang, Dalvik VM / ART, and more!), values are pushed onto a stack and instructions operate on the values at the top!

for example:

```text
10
20
Add
```

becomes:

```text
10 and 20
    ↓
   Add
    ↓
   30.
```

so... the VM reads bytecode one instruction at a time, updates the stack, and continues until it reaches `Halt`.

## Example

a small program can be built using the `BytecodeBuilder` (`builder::BytecodeBuilder`):
(do take note that this is a short snippet.. it doesn't include the usual boilerplate you should see, such as `fn main() {}` and `use`)

```rust
// Create your lovely BytecodeBuilder !!
// It acts as an API that makes your life easier by letting you
// call methods and pass random arguments to get it workin'!!!
let mut builder = BytecodeBuilder::new();

// Start calling required methods!!!
// You can use method chaining if you want;
// it's for the convenience and ease of development!
builder
    .ldc(Value::Int(10))
    .ldc(Value::Int(20))
    .op(Opcode::Add)
    .op(Opcode::Print)
    .op(Opcode::Halt);

// Just call `.finish()` once you're done cooking!
// It creates a struct object `Bytecode` behind the scenes...
// It contains:
// `code`, which is of type `Vec<u8>`!
// and
// `constants`, which is of type `Vec<Value>`!
let bytecode = builder.finish();

// Create a `Vm` object, passing the finalized bytecode into the constructor.
let mut vm = Vm::new(bytecode);

// Run the virtual machine!
vm.run().unwrap();
```

the output will show:

```text
10
```

## Why I Made This

tbh, IDRK.... i WOULD say that: i wanted to learn how virtual machines actually work instead of just treating them as some mysterious thing that runs code somewhere....

i also wanted a project that involved more than just:

```text
read input
do something
print result
repeat
```

so naturally, i decided to make a bytecode VM and voluntarily signed up myself for raining bullets into the foot 💝

## Current Scope

hey. look. at. features.
scroll. up.

more features may be added soon, such as:

- comparisons
- control flow
- boolean operations
- variables
- functions
- call frames
- memory
- a frontend language that targets ARC

## Related Projects

ARC is intended to uhm be usable by other projects!!

a future project called `TASM` WILL use ARC as its runtime and compile its own syntax into ARC bytecode.

ARC itself is not tied to `TASM` or to assembly-like languages; i intend to design it to be a general-purpose virtual machine, ready to be used by frontends, of any type.!

## Requirements

in general:

- Rust
- Cargo

including the development launcher:

- Bash

## Running

clone the lovely repository:

```bash
git clone https://github.com/jayywashere/arc
cd arc
```

run the tests (optional):

```bash
cargo test
```

to display printed output from tests:

```bash
cargo test -- --nocapture
```

furthermore, we have `arc.sh`!! which means we can use this to, uhm, do CLI thingies,.. YOU'LL SEE WHAT I MEAN.

available commands rn:

```bash
arc help
arc run <.arx binary>
```

to view all available commands and their usage format:

```bash
arc help
```

to run a `.arx` binary:

```bash
arc run <.arx binary>
```

## License

See [LICENSE](LICENSE)
