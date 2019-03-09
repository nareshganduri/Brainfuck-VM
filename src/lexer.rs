use std::str::Chars;

use crate::token::Token;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    curr: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let curr = chars.next();

        Self { chars, curr }
    }

    fn peek(&self) -> Option<char> {
        self.curr
    }

    fn cont(&mut self) {
        self.curr = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => return,
                _ => self.cont(),
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = self.peek();
        self.cont();

        if let Some(c) = c {
            match c {
                '>' => Token::IncDp,
                '<' => Token::DecDp,
                '+' => Token::Inc,
                '-' => Token::Dec,
                '.' => Token::Output,
                ',' => Token::Input,
                '[' => Token::Branch,
                ']' => Token::Jump,
                _ => unreachable!(),
            }
        } else {
            Token::Eof
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("asdf  ewrw +-[ ] , ff . >< sdfs");

        assert_eq!(lexer.next_token(), Token::Inc);
        assert_eq!(lexer.next_token(), Token::Dec);
        assert_eq!(lexer.next_token(), Token::Branch);
        assert_eq!(lexer.next_token(), Token::Jump);
        assert_eq!(lexer.next_token(), Token::Input);
        assert_eq!(lexer.next_token(), Token::Output);
        assert_eq!(lexer.next_token(), Token::IncDp);
        assert_eq!(lexer.next_token(), Token::DecDp);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
