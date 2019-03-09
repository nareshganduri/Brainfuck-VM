use crate::compile::{OpCode, Program};
use std::io::{self, Read, Write};

type VmResult = Result<(), String>;

pub struct Vm {
    array: [u8; 30_000],
    idx: usize,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            array: [0; 30_000],
            idx: 0,
            ip: 0,
        }
    }

    pub fn run(&mut self, program: &Program) -> VmResult {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        self.ip = 0;

        loop {
            let inst = unsafe { program.code.get_unchecked(self.ip) };
            self.ip += 1;

            match inst {
                OpCode::Return => return Ok(()),
                OpCode::Nop => return Err("Hit no-op in compiled bytecode.".to_string()),
                OpCode::Dec => {
                    self.array[self.idx] = self.array[self.idx].wrapping_sub(1);
                }
                OpCode::DecDp => {
                    if self.idx == 0 {
                        return Err("Moved past end of array.".to_string());
                    } else {
                        self.idx -= 1;
                    }
                }
                OpCode::Inc => {
                    self.array[self.idx] = self.array[self.idx].wrapping_add(1);
                }
                OpCode::IncDp => {
                    if self.idx == 29_999 {
                        return Err("Moved past end of array.".to_string());
                    } else {
                        self.idx += 1;
                    }
                }
                OpCode::Output => {
                    let c = self.array[self.idx] as char;
                    print!("{}", c);
                    stdout.flush().unwrap();
                }
                OpCode::Input => {
                    let mut c = [0u8];
                    stdin.read_exact(&mut c).unwrap();

                    self.array[self.idx] = c[0];
                }
                OpCode::Branch(dist) => {
                    if self.array[self.idx] == 0 {
                        self.ip += *dist as usize;
                    }
                }
                OpCode::Jump(dist) => {
                    let dist = -*dist as isize;
                    self.ip -= dist as usize;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vm;
    use crate::compile::Compiler;

    #[test]
    fn test_vm_no_branch() {
        let code = "++++++";
        let compiler = Compiler::new(code);
        let program = compiler.compile().unwrap();

        let mut vm = Vm::new();
        vm.run(&program).unwrap();

        assert_eq!(vm.array[0], 6);
    }

    #[test]
    fn test_vm_simple() {
        let code = "
        ++
        > +++++
        [< + > -]
        ++++ ++++
        [
        < +++ +++
        > -
        ]
        <";

        let compiler = Compiler::new(code);
        let program = compiler.compile().unwrap();

        let mut vm = Vm::new();
        vm.run(&program).unwrap();

        assert_eq!(vm.array[0], 55);
    }
}
