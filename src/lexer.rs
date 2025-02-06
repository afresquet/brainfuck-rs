use std::str::Chars;

use crate::Token;

/// Lexical analyzer that transforms a program to Tokens.
#[derive(Debug)]
pub struct Lexer<I> {
    iter: I,
    index: usize,
}

impl<'a> Lexer<Chars<'a>> {
    pub fn new(program: &'a str) -> Self {
        Self {
            iter: program.chars(),
            index: 0,
        }
    }

    pub fn to_peekable(self) -> PeekableLexer<'a> {
        PeekableLexer {
            lexer: self,
            peeked: None,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl Iterator for Lexer<Chars<'_>> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.iter.next()?;
            self.index += 1;
            if let Ok(token) = c.try_into() {
                return Some(token);
            }
        }
    }
}

/// Lexical analyzer that transforms a program to Tokens.
/// Can be peeked.
#[derive(Debug)]
pub struct PeekableLexer<'a> {
    lexer: Lexer<Chars<'a>>,
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
