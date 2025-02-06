use std::io::{BufRead, BufReader, Read, Stdin, Stdout, Write};

use thiserror::Error;

use crate::Instruction;

#[derive(Debug)]
pub struct Executor<I, O, const N: usize = 30000> {
    data: [u8; N],
    pointer: usize,
    input: BufReader<I>,
    output: O,
}

impl<I, O, const N: usize> Executor<I, O, N>
where
    I: Read,
    O: Write,
{
    pub fn new(input: I, output: O) -> Self {
        Self {
            data: [0; N],
            pointer: 0,
            input: BufReader::new(input),
            output,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<(), ExecutorError> {
        match instruction {
            Instruction::MoveRight(amount) => {
                self.pointer = self.pointer.wrapping_add(*amount) % N;
            }
            Instruction::MoveLeft(amount) => {
                self.pointer = self.pointer.wrapping_sub(*amount) % N;
            }
            Instruction::Increment(amount) => {
                self.data[self.pointer] = self.data[self.pointer].wrapping_add(*amount);
            }
            Instruction::Decrement(amount) => {
                self.data[self.pointer] = self.data[self.pointer].wrapping_sub(*amount);
            }
            Instruction::Output => {
                self.output.write_all(&[self.data[self.pointer]])?;
            }
            Instruction::Input => {
                // TODO: figure out how to get rid of this heap allocation
                let mut input = String::new();
                self.input.read_line(&mut input)?;
                let value: u8 = input.trim().parse()?;
                self.data[self.pointer] = value;
            }
            Instruction::Loop(instructions) => {
                while self.data[self.pointer] > 0 {
                    for instruction in instructions {
                        self.execute(instruction)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for Executor<Stdin, Stdout> {
    fn default() -> Self {
        Self::new(std::io::stdin(), std::io::stdout())
    }
}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("error from stdin/stdout: {0}")]
    IO(#[from] std::io::Error),
    #[error("error parsing input to an int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
