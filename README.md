# Brainfuck VM
A simple [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter implemented in Rust.

The program reads a source file, compiles the input to bytecode, and executes the bytecode in a virtual machine.

## But Why?
Who knows. Fun, I guess? Because Brainfuck is such a simple language, implementing an interpreter for it is a popular exercise.

Besides, there are full-fledged JIT compilers out there for it, so this is *nothing* in comparison.

With that in mind, there is an interesting problem to be solved in regards to compiling jump targets to bytecode. See [compile.rs](./src/compile.rs) for an example.

## Running
Cloning the repository and running the following should do the trick:
```
cargo run -- FILENAME
```
See the [examples](./examples) folder for sample code to run.