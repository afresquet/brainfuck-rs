use std::io::{Read, Stdin, Stdout, Write};

use thiserror::Error;

use crate::{IRError, Instruction, IntermediateRepresentation};

#[derive(Debug)]
pub struct Executor<I, O, const N: usize = 30000> {
    data: [u8; N],
    pointer: usize,
    input: I,
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
            input,
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
                let mut input = [0; 5];
                let len = self.input.read(&mut input)?;
                self.data[self.pointer] = parse_input(&input, len)?;
            }
            Instruction::Loop { program } => {
                while self.data[self.pointer] > 0 {
                    for instruction in IntermediateRepresentation::new(program) {
                        self.execute(&instruction?)?;
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
    #[error("error from input/output: {0}")]
    IO(#[from] std::io::Error),
    #[error("error parsing input to an int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IRError(#[from] IRError),
    #[error(transparent)]
    ParseInputError(#[from] ParseInputError),
}

fn parse_input(input: &[u8], len: usize) -> Result<u8, ParseInputError> {
    match len {
        2..=4 => {
            let mut value: u8 = 0;

            for (i, byte) in input[0..len - 1].iter().rev().enumerate() {
                let c = *byte as char;
                let n = c
                    .to_digit(10)
                    .map(|n| n as u8)
                    .ok_or(ParseInputError::InvalidCharacter(c))?
                    .checked_mul(10_u8.pow(i as u32))
                    .ok_or(ParseInputError::NumberTooBig)?;
                value = value.checked_add(n).ok_or(ParseInputError::NumberTooBig)?;
            }

            Ok(value)
        }
        ..=1 | 5.. => Err(ParseInputError::BadInput),
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseInputError {
    #[error("bad input")]
    BadInput,
    #[error("invalid character: '{0}'")]
    InvalidCharacter(char),
    #[error("number is bigger than u8::MAX (255)")]
    NumberTooBig,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::char_lit_as_u8)]

    use super::*;

    #[test]
    fn parse_input_one_digit() {
        let mut input = [0; 3];
        input[0] = '9' as u8;
        assert_eq!(parse_input(&input, 2), Ok(9));
        input[0] = '5' as u8;
        assert_eq!(parse_input(&input, 2), Ok(5));
    }

    #[test]
    fn parse_input_two_digit() {
        let mut input = [0; 3];
        input[0] = '9' as u8;
        input[1] = '5' as u8;
        assert_eq!(parse_input(&input, 3), Ok(95));
        input[0] = '2' as u8;
        input[1] = '8' as u8;
        assert_eq!(parse_input(&input, 3), Ok(28));
    }

    #[test]
    fn parse_input_three_digit() {
        let mut input = [0; 3];
        input[0] = '1' as u8;
        input[1] = '9' as u8;
        input[2] = '8' as u8;
        assert_eq!(parse_input(&input, 4), Ok(198));
        input[0] = '2' as u8;
        input[1] = '1' as u8;
        input[2] = '4' as u8;
        assert_eq!(parse_input(&input, 4), Ok(214));
    }

    #[test]
    fn parse_bad_input_error() {
        assert_eq!(parse_input(&[], 0), Err(ParseInputError::BadInput));
        assert_eq!(parse_input(&[], 1), Err(ParseInputError::BadInput));
        assert_eq!(parse_input(&[], 5), Err(ParseInputError::BadInput));
        assert_eq!(parse_input(&[], 10), Err(ParseInputError::BadInput));
    }

    #[test]
    fn parse_input_invalid_character_error() {
        let mut input = [0; 3];
        input[0] = 'a' as u8;
        assert_eq!(
            parse_input(&input, 2),
            Err(ParseInputError::InvalidCharacter('a'))
        );
        input[0] = '5' as u8;
        input[1] = 'z' as u8;
        assert_eq!(
            parse_input(&input, 3),
            Err(ParseInputError::InvalidCharacter('z'))
        );
    }

    #[test]
    fn parse_input_too_big_error() {
        let mut input = [0; 3];
        input[0] = '2' as u8;
        input[1] = '5' as u8;
        input[2] = '6' as u8;
        assert_eq!(parse_input(&input, 4), Err(ParseInputError::NumberTooBig));
    }
}
