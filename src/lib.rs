//! A Brainfuck virtual machine implemented in Rust

mod compile;
mod lexer;
mod parser;
mod token;
mod vm;

use crate::compile::Compiler;
use crate::vm::Vm;
use std::fs::OpenOptions;
use std::io::Read;

/// Runs a program with its source code stored as a string
pub fn run_source(source: &str) -> Result<(), String> {
    let compiler = Compiler::new(&source);
    let program = compiler.compile()?;

    let mut vm = Vm::new();
    vm.run(&program)
}

/// Runs a program from a file
pub fn run_file(filename: &str) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .read(true)
        .open(filename)
        .map_err(|e| e.to_string())?;

    let mut src = String::new();
    f.read_to_string(&mut src).map_err(|e| e.to_string())?;

    run_source(&src)
}
