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

    pub fn to_peekable(self) -> PeekableLexer<Self> {
        PeekableLexer::new(self)
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
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

/// Lexical analyzer that transforms a program to [`Token`]s.
/// Can be peeked.
#[derive(Debug)]
pub struct PeekableLexer<L> {
    lexer: L,
    peeked: Option<Option<Token>>,
}

impl<L> PeekableLexer<L> {
    pub fn new(lexer: L) -> Self {
        Self {
            lexer,
            peeked: None,
        }
    }
}

impl<I> PeekableLexer<Lexer<I>>
where
    I: Iterator<Item = char>,
{
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
        let mut lexer = Lexer::new(input.chars());
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
        let mut lexer = Lexer::new(input.chars());
        assert_eq!(lexer.next(), Some(Token::LoopStart));
        assert_eq!(lexer.next(), Some(Token::Decrement));
        assert_eq!(lexer.next(), Some(Token::Output));
        assert_eq!(lexer.next(), None);
    }
}
