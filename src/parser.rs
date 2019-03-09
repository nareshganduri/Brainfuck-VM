use crate::compile::Ast;
use crate::lexer::Lexer;
use crate::token::Token;

type ParseResult<T> = Result<T, String>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Token,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer::new(src);
        let curr = lexer.next_token();

        Self { lexer, curr }
    }

    fn cont(&mut self) {
        self.curr = self.lexer.next_token();
    }

    fn matches(&mut self, token: Token) -> bool {
        if self.curr == token {
            self.cont();
            true
        } else {
            false
        }
    }

    fn parse_code(&mut self) -> ParseResult<Ast> {
        if self.matches(Token::Dec) {
            Ok(Ast::Dec)
        } else if self.matches(Token::DecDp) {
            Ok(Ast::DecDp)
        } else if self.matches(Token::Inc) {
            Ok(Ast::Inc)
        } else if self.matches(Token::IncDp) {
            Ok(Ast::IncDp)
        } else if self.matches(Token::Input) {
            Ok(Ast::Input)
        } else if self.matches(Token::Output) {
            Ok(Ast::Output)
        } else {
            Err("Unexpected token.".to_string())
        }
    }

    fn parse_branch(&mut self) -> ParseResult<Ast> {
        let mut v = vec![];

        while !self.matches(Token::Jump) {
            let inner = self.parse_code_or_branch()?;
            v.push(inner);
        }

        let result = Ast::Branch { inner: v };
        Ok(result)
    }

    fn parse_code_or_branch(&mut self) -> ParseResult<Ast> {
        if self.matches(Token::Branch) {
            self.parse_branch()
        } else {
            self.parse_code()
        }
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        let mut v = vec![];

        while !self.matches(Token::Eof) {
            let inner = self.parse_code_or_branch()?;
            v.push(inner);
        }

        let result = Ast::Code { codes: v };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::compile::Ast;

    #[test]
    fn test_simple_parse() {
        let mut parser = Parser::new("><+[,.[+.-]<.]>>");
        let actual = parser.parse().unwrap();

        let expected = Ast::Code {
            codes: vec![
                Ast::IncDp,
                Ast::DecDp,
                Ast::Inc,
                Ast::Branch {
                    inner: vec![
                        Ast::Input,
                        Ast::Output,
                        Ast::Branch {
                            inner: vec![Ast::Inc, Ast::Output, Ast::Dec],
                        },
                        Ast::DecDp,
                        Ast::Output,
                    ],
                },
                Ast::IncDp,
                Ast::IncDp,
            ],
        };

        assert_eq!(actual, expected);
    }
}
