use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// >
    ///
    /// Increment the data pointer by one (to point to the next cell to the right).
    MoveRight,
    /// <
    ///
    /// Decrement the data pointer by one (to point to the next cell to the left).
    MoveLeft,
    /// +
    ///
    /// Increment the byte at the data pointer by one.
    Increment,
    /// -
    ///
    /// Decrement the byte at the data pointer by one.
    Decrement,
    /// .
    ///
    /// Output the byte at the data pointer.
    Output,
    /// ,
    ///
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    Input,
    /// [
    ///
    /// If the byte at the data pointer is zero,
    /// then instead of moving the instruction pointer forward to the next command,
    /// jump it forward to the command after the matching ] command.
    LoopStart,
    /// ]
    ///
    /// If the byte at the data pointer is nonzero,
    /// then instead of moving the instruction pointer forward to the next command,
    /// jump it back to the command after the matching [ command.
    LoopEnd,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {:?}", char::from(*self), self)
    }
}

#[derive(Debug, Error)]
#[error("invalid token {0}")]
pub struct InvalidTokenError(char);

impl TryFrom<char> for Token {
    type Error = InvalidTokenError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::MoveRight),
            '<' => Ok(Self::MoveLeft),
            '+' => Ok(Self::Increment),
            '-' => Ok(Self::Decrement),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            '[' => Ok(Self::LoopStart),
            ']' => Ok(Self::LoopEnd),
            c => Err(InvalidTokenError(c)),
        }
    }
}

impl From<Token> for char {
    fn from(value: Token) -> Self {
        match value {
            Token::MoveRight => '>',
            Token::MoveLeft => '<',
            Token::Increment => '+',
            Token::Decrement => '-',
            Token::Output => '.',
            Token::Input => ',',
            Token::LoopStart => '[',
            Token::LoopEnd => ']',
        }
    }
}
