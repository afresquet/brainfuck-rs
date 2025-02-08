#![allow(clippy::char_lit_as_u8)]

use thiserror::Error;

use crate::Lexer;

/// Representation of the characters as language commands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// >
    ///
    /// Increment the data pointer by one (to point to the next cell to the right).
    MoveRight = '>' as u8,
    /// <
    ///
    /// Decrement the data pointer by one (to point to the next cell to the left).
    MoveLeft = '<' as u8,
    /// +
    ///
    /// Increment the byte at the data pointer by one.
    Increment = '+' as u8,
    /// -
    ///
    /// Decrement the byte at the data pointer by one.
    Decrement = '-' as u8,
    /// .
    ///
    /// Output the byte at the data pointer.
    Output = '.' as u8,
    /// ,
    ///
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    Input = ',' as u8,
    /// [
    ///
    /// If the byte at the data pointer is zero,
    /// then instead of moving the instruction pointer forward to the next command,
    /// jump it forward to the command after the matching ] command.
    LoopStart = '[' as u8,
    /// ]
    ///
    /// If the byte at the data pointer is nonzero,
    /// then instead of moving the instruction pointer forward to the next command,
    /// jump it back to the command after the matching [ command.
    LoopEnd = ']' as u8,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} | {:?}", char::from(*self), self)
    }
}

#[derive(Debug, Error)]
#[error("invalid token {0}")]
pub struct InvalidTokenError<T>(T);

impl TryFrom<u8> for Token {
    type Error = InvalidTokenError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'>' => Ok(Self::MoveRight),
            b'<' => Ok(Self::MoveLeft),
            b'+' => Ok(Self::Increment),
            b'-' => Ok(Self::Decrement),
            b'.' => Ok(Self::Output),
            b',' => Ok(Self::Input),
            b'[' => Ok(Self::LoopStart),
            b']' => Ok(Self::LoopEnd),
            c => Err(InvalidTokenError(c)),
        }
    }
}

impl TryFrom<&u8> for Token {
    type Error = InvalidTokenError<u8>;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        TryFrom::try_from(*value).map_err(|_| InvalidTokenError(*value))
    }
}

impl From<Token> for u8 {
    fn from(value: Token) -> Self {
        value as u8
    }
}

impl TryFrom<char> for Token {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        TryFrom::try_from(value as u8).map_err(|_| InvalidTokenError(value))
    }
}

impl From<Token> for char {
    fn from(value: Token) -> Self {
        value as u8 as char
    }
}

pub trait TokenIterator<'a> {
    type IntoIter: Iterator<Item = (Token, usize)>;

    fn iter_token(&'a self) -> Self::IntoIter;
}

impl<'a> TokenIterator<'a> for str {
    type IntoIter = Lexer<core::str::Chars<'a>>;

    fn iter_token(&'a self) -> Self::IntoIter {
        Lexer::new(self.chars())
    }
}

impl<'a> TokenIterator<'a> for [u8] {
    type IntoIter = Lexer<core::slice::Iter<'a, u8>>;

    fn iter_token(&'a self) -> Self::IntoIter {
        Lexer::new(self.iter())
    }
}
