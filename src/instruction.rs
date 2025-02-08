use core::{
    iter::Peekable,
    ops::{Index, Range},
    slice::SliceIndex,
};

use crate::{IRError, IntermediateRepresentation, Lexer, TokenIterator};

/// Instruction of the language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction<P> {
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
    Loop(InstructionLoop<P>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionLoop<P>(P);

impl<P> InstructionLoop<P> {
    pub fn new(program: P) -> Self {
        Self(program)
    }
}

impl<'a, P> InstructionLoop<&'a P>
where
    P: ?Sized,
{
    pub fn program(&self) -> &'a P {
        self.0
    }
}

impl<'a, P> IntoIterator for &InstructionLoop<&'a P>
where
    P: TokenIterator<'a> + Index<Range<usize>, Output = P> + ?Sized,
    Range<usize>: SliceIndex<P>,
{
    type Item = Result<Instruction<&'a <P as Index<Range<usize>>>::Output>, IRError>;

    type IntoIter = IntermediateRepresentation<&'a P, Peekable<P::IntoIter>>;

    fn into_iter(self) -> Self::IntoIter {
        IntermediateRepresentation::new(self.program())
    }
}

impl<'a, P> core::fmt::Display for Instruction<&'a P>
where
    P: TokenIterator<'a> + Index<Range<usize>, Output = P> + ?Sized,
    Range<usize>: SliceIndex<P>,
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

pub trait InstructionIterator<'a, P = Self>
where
    P: TokenIterator<'a> + Index<Range<usize>, Output = P> + ?Sized + 'a,
    Range<usize>: SliceIndex<P>,
{
    type IntoIter: Iterator<
        Item = Result<Instruction<&'a <P as Index<Range<usize>>>::Output>, IRError>,
    >;

    fn iter_instruction(&'a self) -> Self::IntoIter;
}

impl<'a> InstructionIterator<'a> for str {
    type IntoIter = IntermediateRepresentation<&'a Self, Peekable<Lexer<core::str::Chars<'a>>>>;

    fn iter_instruction(&'a self) -> Self::IntoIter {
        IntermediateRepresentation::new(self)
    }
}

impl<'a> InstructionIterator<'a> for [u8] {
    type IntoIter =
        IntermediateRepresentation<&'a Self, Peekable<Lexer<core::slice::Iter<'a, u8>>>>;

    fn iter_instruction(&'a self) -> Self::IntoIter {
        IntermediateRepresentation::new(self)
    }
}
