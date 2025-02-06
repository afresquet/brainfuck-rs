use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
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
    Loop(Vec<Instruction>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::MoveRight(amount) => write!(f, "Increment pointer {amount} times"),
            Instruction::MoveLeft(amount) => write!(f, "Decrement pointer {amount} times"),
            Instruction::Increment(amount) => write!(f, "Increment {amount} times"),
            Instruction::Decrement(amount) => write!(f, "Decrement {amount} times"),
            Instruction::Output => write!(f, "Output"),
            Instruction::Input => write!(f, "Input"),
            Instruction::Loop(instructions) => {
                _ = writeln!(f, "Start Loop:");
                for instruction in instructions {
                    _ = writeln!(f, "{instruction}");
                }
                write!(f, "End Loop")
            }
        }
    }
}
