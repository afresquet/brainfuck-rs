use std::str::Chars;

use crate::Token;

#[derive(Debug)]
pub struct Lexer<'a, I> {
    program: &'a str,
    iter: I,
    i: usize,
}

impl<'a> Lexer<'a, Chars<'a>> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            iter: program.chars(),
            i: 0,
        }
    }

    pub fn to_peekable(self) -> PeekableLexer<'a> {
        PeekableLexer {
            lexer: self,
            peeked: None,
        }
    }

    pub fn index(&self) -> usize {
        self.i
    }

    pub fn reset(&mut self) {
        self.iter = self.program.chars();
        self.i = 0;
    }
}

impl<'a> Iterator for Lexer<'a, Chars<'a>> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.iter.next()?;
            self.i += 1;
            if let Ok(token) = c.try_into() {
                return Some(token);
            }
        }
    }
}

#[derive(Debug)]
pub struct PeekableLexer<'a> {
    lexer: Lexer<'a, Chars<'a>>,
    peeked: Option<Option<Token>>,
}

impl PeekableLexer<'_> {
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Token> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.lexer.next(),
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.peeked
            .get_or_insert_with(|| self.lexer.next())
            .as_ref()
    }

    pub fn index(&self) -> usize {
        self.lexer.index()
    }

    pub fn reset(&mut self) {
        self.lexer.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tokens() {
        let input = "><+-.,[]";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::MoveRight));
        assert_eq!(lexer.next(), Some(Token::MoveLeft));
        assert_eq!(lexer.next(), Some(Token::Increment));
        assert_eq!(lexer.next(), Some(Token::Decrement));
        assert_eq!(lexer.next(), Some(Token::Output));
        assert_eq!(lexer.next(), Some(Token::Input));
        assert_eq!(lexer.next(), Some(Token::LoopStart));
        assert_eq!(lexer.next(), Some(Token::LoopEnd));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn ignores_non_token_characters() {
        let input = "[1-r2.";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::LoopStart));
        assert_eq!(lexer.next(), Some(Token::Decrement));
        assert_eq!(lexer.next(), Some(Token::Output));
        assert_eq!(lexer.next(), None);
    }
}
