use core::iter::Peekable;

use crate::{IRError, IntermediateRepresentation, Ranged, TokenIterator};

/// Instruction of the language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction<T> {
    /// >
    ///
    /// Increment the data pointer by an amount (to point to the right).
    MoveRight(usize),
    /// <
    ///
    /// Decrement the data pointer by an amount (to point to the left).
    MoveLeft(usize),
    /// +
    ///
    /// Increment the byte at the data pointer by an amount.
    Increment(u8),
    /// -
    ///
    /// Decrement the byte at the data pointer by an amount.
    Decrement(u8),
    /// .
    ///
    /// Output the byte at the data pointer.
    Output,
    /// ,
    ///
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    Input,
    /// []
    ///
    /// Loop if the data pointer is not zero.
    Loop(InstructionLoop<T>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionLoop<T>(T);

impl<T> InstructionLoop<T> {
    pub fn new(program: T) -> Self {
        Self(program)
    }
}

impl<'a, T> InstructionLoop<&'a T>
where
    T: ?Sized,
{
    pub fn program(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> IntoIterator for &InstructionLoop<&'a T>
where
    T: TokenIterator<'a> + Ranged + ?Sized,
{
    type Item = Result<Instruction<&'a T>, IRError>;

    type IntoIter = IntermediateRepresentation<&'a T, Peekable<T::TokenIter>>;

    fn into_iter(self) -> Self::IntoIter {
        IntermediateRepresentation::new(self.program())
    }
}

impl<'a, T> core::fmt::Display for Instruction<&'a T>
where
    T: TokenIterator<'a> + Ranged + ?Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Instruction::MoveRight(amount) => write!(f, "Increment pointer {amount} times"),
            Instruction::MoveLeft(amount) => write!(f, "Decrement pointer {amount} times"),
            Instruction::Increment(amount) => write!(f, "Increment {amount} times"),
            Instruction::Decrement(amount) => write!(f, "Decrement {amount} times"),
            Instruction::Output => write!(f, "Output"),
            Instruction::Input => write!(f, "Input"),
            Instruction::Loop(instructions) => {
                writeln!(f, "Start Loop:")?;
                for instruction in instructions {
                    match instruction {
                        Ok(instruction) => writeln!(f, "{instruction}")?,
                        Err(error) => panic!("ERROR: {error}"),
                    }
                }
                write!(f, "End Loop")
            }
        }
    }
}

pub trait InstructionIterator<'a, E, T = Self>
where
    T: TokenIterator<'a> + Ranged + ?Sized + 'a,
{
    type InstructionIter: Iterator<Item = Result<Instruction<&'a T>, E>>;

    fn iter_instruction(&'a self) -> Self::InstructionIter;
}

impl<'a, T> InstructionIterator<'a, IRError> for T
where
    T: TokenIterator<'a> + Ranged + ?Sized + 'a,
{
    type InstructionIter = IntermediateRepresentation<&'a Self, Peekable<T::TokenIter>>;

    fn iter_instruction(&'a self) -> Self::InstructionIter {
        IntermediateRepresentation::new(self)
    }
}
