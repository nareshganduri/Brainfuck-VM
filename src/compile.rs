use crate::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum Ast {
    IncDp,
    DecDp,
    Inc,
    Dec,
    Output,
    Input,
    Code { codes: Vec<Ast> },
    Branch { inner: Vec<Ast> },
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    IncDp,
    DecDp,
    Inc,
    Dec,
    Output,
    Input,
    Branch(u16),
    Jump(i16),
    Return,
    Nop,
}

pub struct Program {
    pub code: Vec<OpCode>,
}

type CompileResult<T> = Result<T, String>;

pub struct Compiler<'a> {
    parser: Parser<'a>,
    program: Program,
}

impl<'a> Compiler<'a> {
    pub fn new(src: &'a str) -> Self {
        let parser = Parser::new(src);
        let program = Program { code: vec![] };

        Self { parser, program }
    }

    fn compile_ast(&mut self, ast: &Ast) {
        match ast {
            Ast::Code { codes } => {
                for code in codes {
                    self.compile_ast(code);
                }
            }
            Ast::Branch { inner } => {
                let start = self.program.code.len();
                self.program.code.push(OpCode::Nop);

                for code in inner {
                    self.compile_ast(code);
                }

                let end = self.program.code.len();
                let dist = start as i16 - end as i16 - 1;

                self.program.code.push(OpCode::Jump(dist));

                let branch_dist = end as u16 - start as u16;
                self.program.code[start] = OpCode::Branch(branch_dist);
            }
            Ast::Dec => self.program.code.push(OpCode::Dec),
            Ast::DecDp => self.program.code.push(OpCode::DecDp),
            Ast::Inc => self.program.code.push(OpCode::Inc),
            Ast::IncDp => self.program.code.push(OpCode::IncDp),
            Ast::Input => self.program.code.push(OpCode::Input),
            Ast::Output => self.program.code.push(OpCode::Output),
        }
    }

    pub fn compile(mut self) -> CompileResult<Program> {
        let ast = self.parser.parse()?;
        self.compile_ast(&ast);

        self.program.code.push(OpCode::Return);

        Ok(self.program)
    }
}

#[cfg(test)]
mod tests {
    use super::{Compiler, OpCode};
    #[test]
    fn test_compiler_simple() {
        let code = "+->><[++.<,]+.";
        let compiler = Compiler::new(code);
        let program = compiler.compile().unwrap();

        let expected = vec![
            OpCode::Inc,
            OpCode::Dec,
            OpCode::IncDp,
            OpCode::IncDp,
            OpCode::DecDp,
            OpCode::Branch(6),
            OpCode::Inc,
            OpCode::Inc,
            OpCode::Output,
            OpCode::DecDp,
            OpCode::Input,
            OpCode::Jump(-7),
            OpCode::Inc,
            OpCode::Output,
            OpCode::Return,
        ];

        assert_eq!(program.code, expected);
    }
}
