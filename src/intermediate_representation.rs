use core::iter::Peekable;

use thiserror::Error;

use crate::{Instruction, InstructionLoop, Ranged, Token, TokenIterator};

/// Transformer from [`Token`]s to [`Instruction`]s.
#[derive(Debug)]
pub struct IntermediateRepresentation<T, I> {
    program: T,
    iter: I,
}

impl<'a, T> IntermediateRepresentation<&'a T, Peekable<T::TokenIter>>
where
    T: TokenIterator<'a> + ?Sized,
{
    pub fn new(program: &'a T) -> Self {
        let iter = program.iter_token().peekable();
        Self { program, iter }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IRError {
    #[error("unmatched '[' encountered")]
    UnmatchedLoopStart,
    #[error("unmatched ']' encountered")]
    UnmatchedLoopEnd,
}

impl<'a, T> Iterator for IntermediateRepresentation<&'a T, Peekable<T::TokenIter>>
where
    T: TokenIterator<'a> + Ranged + ?Sized,
{
    type Item = Result<Instruction<&'a T>, IRError>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! instruction_amount {
            ($token:path, $instruction:path, $int:ty) => {{
                let mut amount: $int = 1;

                while let Some((_, $token)) = self.iter.peek() {
                    self.iter.next();
                    amount = amount.wrapping_add(1);
                }

                Some(Ok($instruction(amount)))
            }};
        }

        match self.iter.next()? {
            (_, Token::MoveRight) => {
                instruction_amount!(Token::MoveRight, Instruction::MoveRight, usize)
            }
            (_, Token::MoveLeft) => {
                instruction_amount!(Token::MoveLeft, Instruction::MoveLeft, usize)
            }
            (_, Token::Increment) => {
                instruction_amount!(Token::Increment, Instruction::Increment, u8)
            }
            (_, Token::Decrement) => {
                instruction_amount!(Token::Decrement, Instruction::Decrement, u8)
            }
            (_, Token::Output) => Some(Ok(Instruction::Output)),
            (_, Token::Input) => Some(Ok(Instruction::Input)),
            (start, Token::LoopStart) => {
                let mut open: usize = 1;
                let program = loop {
                    match self.iter.next() {
                        Some((_, Token::LoopStart)) => open += 1,
                        Some((end, Token::LoopEnd)) => {
                            open -= 1;
                            if open == 0 {
                                // The + 1 skips the [ at the start.
                                break self.program.range(start + 1, end);
                            }
                        }
                        Some(_) => (),
                        None => return Some(Err(IRError::UnmatchedLoopStart)),
                    }
                };

                Some(Ok(Instruction::Loop(InstructionLoop::new(program))))
            }
            (_, Token::LoopEnd) => Some(Err(IRError::UnmatchedLoopEnd)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::InstructionIterator;

    use super::*;

    #[test]
    fn parses_instructions() {
        let program = ">>><<++++-..,[[--->>]][++<]";
        let mut ir = program.iter_instruction();
        assert_eq!(ir.next(), Some(Ok(Instruction::MoveRight(3))));
        assert_eq!(ir.next(), Some(Ok(Instruction::MoveLeft(2))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(4))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Decrement(1))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Output)));
        assert_eq!(ir.next(), Some(Ok(Instruction::Output)));
        assert_eq!(ir.next(), Some(Ok(Instruction::Input)));
        let Some(Ok(Instruction::Loop(instruction_loop))) = ir.next() else {
            panic!("should be a loop");
        };
        assert_eq!(instruction_loop.program(), "[--->>]");
        let Some(Ok(Instruction::Loop(instruction_loop))) = instruction_loop.into_iter().next()
        else {
            panic!("should be a loop");
        };
        assert_eq!(instruction_loop.program(), "--->>");
        let mut inner_instruction_loop = instruction_loop.into_iter();
        assert_eq!(
            inner_instruction_loop.next(),
            Some(Ok(Instruction::Decrement(3)))
        );
        assert_eq!(
            inner_instruction_loop.next(),
            Some(Ok(Instruction::MoveRight(2)))
        );
        let Some(Ok(Instruction::Loop(instruction_loop))) = ir.next() else {
            panic!("should be a loop");
        };
        assert_eq!(instruction_loop.program(), "++<");
        let mut instruction_loop = instruction_loop.into_iter();
        assert_eq!(instruction_loop.next(), Some(Ok(Instruction::Increment(2))));
        assert_eq!(instruction_loop.next(), Some(Ok(Instruction::MoveLeft(1))));
        assert_eq!(ir.next(), None);
    }

    #[test]
    fn amount_wraps_around() {
        let program: String = core::iter::repeat_n('+', 260).collect();
        let mut ir = program.iter_instruction();
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(4))));
        assert_eq!(ir.next(), None);
    }

    #[test]
    fn errors_on_unmatched_loop_start() {
        let program = "+++[---";
        let mut ir = program.iter_instruction();
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(3))));
        assert_eq!(ir.next(), Some(Err(IRError::UnmatchedLoopStart)));
    }

    #[test]
    fn errors_on_unmatched_loop_end() {
        let program = "+++]---";
        let mut ir = program.iter_instruction();
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(3))));
        assert_eq!(ir.next(), Some(Err(IRError::UnmatchedLoopEnd)));
    }
}
