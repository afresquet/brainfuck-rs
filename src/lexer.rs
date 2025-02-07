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

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = (Token, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let c = self.iter.next()?;
            self.index += 1;
            if let Ok(token) = c.try_into() {
                // Need to subtract one since `index` now points to the next character.
                return Some((token, self.index - 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tokens() {
        let input = "><+-.,[]";
        let mut lexer = Lexer::new(input.chars());
        assert_eq!(lexer.next(), Some((Token::MoveRight, 0)));
        assert_eq!(lexer.next(), Some((Token::MoveLeft, 1)));
        assert_eq!(lexer.next(), Some((Token::Increment, 2)));
        assert_eq!(lexer.next(), Some((Token::Decrement, 3)));
        assert_eq!(lexer.next(), Some((Token::Output, 4)));
        assert_eq!(lexer.next(), Some((Token::Input, 5)));
        assert_eq!(lexer.next(), Some((Token::LoopStart, 6)));
        assert_eq!(lexer.next(), Some((Token::LoopEnd, 7)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn ignores_non_token_characters() {
        let input = "[1-r2.";
        let mut lexer = Lexer::new(input.chars());
        assert_eq!(lexer.next(), Some((Token::LoopStart, 0)));
        assert_eq!(lexer.next(), Some((Token::Decrement, 2)));
        assert_eq!(lexer.next(), Some((Token::Output, 5)));
        assert_eq!(lexer.next(), None);
    }
}
