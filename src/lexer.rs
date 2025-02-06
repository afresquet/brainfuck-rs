use crate::Token;

#[derive(Debug)]
pub struct Lexer<I> {
    iter: I,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(iter: I) -> Self {
        Self { iter }
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
            if let Ok(token) = c.try_into() {
                return Some(token);
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
