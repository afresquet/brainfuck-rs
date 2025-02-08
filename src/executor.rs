use core::{
    ops::{Index, Range},
    slice::SliceIndex,
};

use thiserror::Error;

use crate::{IRError, InputStream, Instruction, OutputStream, TokenIterator};

/// Program runner.
#[derive(Debug)]
pub struct Executor<I, O, const N: usize = 30000> {
    /// Byte array.
    data: [u8; N],
    /// Cursor to the current cell in `data`.
    pointer: usize,
    /// Input stream.
    input: I,
    /// Output stream.
    output: O,
}

impl<I, O, const N: usize> Executor<I, O, N> {
    pub fn new(input: I, output: O) -> Self {
        Self {
            data: [0; N],
            pointer: 0,
            input,
            output,
        }
    }
}

impl<I, O, IE, OE, const N: usize> Executor<I, O, N>
where
    I: InputStream<Error = IE>,
    O: OutputStream<Error = OE>,
{
    pub fn execute<'a, P>(
        &mut self,
        instruction: Instruction<&'a P>,
    ) -> Result<(), ExecutorError<IE, OE>>
    where
        P: TokenIterator<'a> + Index<Range<usize>, Output = P> + ?Sized,
        Range<usize>: SliceIndex<P>,
    {
        match instruction {
            Instruction::MoveRight(amount) => {
                self.pointer = self.pointer.wrapping_add(amount) % N;
            }
            Instruction::MoveLeft(amount) => {
                self.pointer = self.pointer.wrapping_sub(amount) % N;
            }
            Instruction::Increment(amount) => {
                self.data[self.pointer] = self.data[self.pointer].wrapping_add(amount);
            }
            Instruction::Decrement(amount) => {
                self.data[self.pointer] = self.data[self.pointer].wrapping_sub(amount);
            }
            Instruction::Output => {
                self.output
                    .write(self.data[self.pointer])
                    .map_err(ExecutorError::OutputError)?;
            }
            Instruction::Input => {
                self.data[self.pointer] = self.input.read().map_err(ExecutorError::InputError)?
            }
            Instruction::Loop(instructions) => {
                while self.data[self.pointer] > 0 {
                    for instruction in &instructions {
                        self.execute(instruction?)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ExecutorError<I, O> {
    #[error("error from input stream: {0}")]
    InputError(I),
    #[error("error from output stream: {0}")]
    OutputError(O),
    #[error(transparent)]
    IRError(#[from] IRError),
}

#[cfg(feature = "std")]
mod stdio {
    use std::io::{stdin, stdout, Stdin, Stdout};

    use super::Executor;

    impl Default for Executor<Stdin, Stdout> {
        fn default() -> Self {
            Self::new(stdin(), stdout())
        }
    }
}
