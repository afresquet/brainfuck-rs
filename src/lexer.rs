use crate::Token;

/// Lexical analyzer that transforms a program to [`Token`]s.
#[derive(Debug)]
pub struct Lexer<I> {
    iter: I,
    index: usize,
}

impl<I> Lexer<I> {
    pub fn new(iter: I) -> Self {
        Self { iter, index: 0 }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<I, T> Iterator for Lexer<I>
where
    I: Iterator<Item = T>,
    T: TryInto<Token>,
{
    type Item = (usize, Token);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.iter.next()?;
            self.index += 1;
            if let Ok(token) = c.try_into() {
                // Need to subtract one since `index` now points to the next character.
                return Some((self.index - 1, token));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenIterator;

    use super::*;

    #[test]
    fn parses_tokens() {
        let program = "><+-.,[]";
        let mut lexer = program.iter_token();
        assert_eq!(lexer.next(), Some((0, Token::MoveRight,)));
        assert_eq!(lexer.next(), Some((1, Token::MoveLeft,)));
        assert_eq!(lexer.next(), Some((2, Token::Increment,)));
        assert_eq!(lexer.next(), Some((3, Token::Decrement,)));
        assert_eq!(lexer.next(), Some((4, Token::Output,)));
        assert_eq!(lexer.next(), Some((5, Token::Input,)));
        assert_eq!(lexer.next(), Some((6, Token::LoopStart,)));
        assert_eq!(lexer.next(), Some((7, Token::LoopEnd,)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn ignores_non_token_characters() {
        let program = "[1-r2.";
        let mut lexer = program.iter_token();
        assert_eq!(lexer.next(), Some((0, Token::LoopStart,)));
        assert_eq!(lexer.next(), Some((2, Token::Decrement,)));
        assert_eq!(lexer.next(), Some((5, Token::Output,)));
        assert_eq!(lexer.next(), None);
    }
}
