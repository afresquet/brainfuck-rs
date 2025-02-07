use core::fmt::Display;

use crate::IntermediateRepresentation;

/// Instruction of the language.
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
    Loop { program: &'a str },
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Instruction::MoveRight(amount) => write!(f, "Increment pointer {amount} times"),
            Instruction::MoveLeft(amount) => write!(f, "Decrement pointer {amount} times"),
            Instruction::Increment(amount) => write!(f, "Increment {amount} times"),
            Instruction::Decrement(amount) => write!(f, "Decrement {amount} times"),
            Instruction::Output => write!(f, "Output"),
            Instruction::Input => write!(f, "Input"),
            Instruction::Loop { program } => {
                writeln!(f, "Start Loop:")?;
                for instruction in IntermediateRepresentation::new(program) {
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
