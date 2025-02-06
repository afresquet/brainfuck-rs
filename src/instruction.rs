use std::fmt::Display;

use crate::{IntermediateRepresentation, PeekableLexer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction<'a> {
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
    Loop(InstructionLoop<'a>),
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                        Err(error) => writeln!(f, "ERROR: {error}")?,
                    }
                }
                write!(f, "End Loop")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionLoop<'a> {
    program: &'a str,
}

impl<'a> InstructionLoop<'a> {
    pub fn new(program: &'a str) -> Self {
        Self { program }
    }
}

impl<'a> IntoIterator for &InstructionLoop<'a> {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = IntermediateRepresentation<'a, PeekableLexer<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        IntermediateRepresentation::new(self.program)
    }
}
