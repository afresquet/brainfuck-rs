use core::{
    iter::Peekable,
    ops::{Index, Range},
    slice::SliceIndex,
};

use thiserror::Error;

use crate::{Instruction, InstructionLoop, Token, TokenIterator};

/// Transformer from [`Token`]s to [`Instruction`]s.
#[derive(Debug)]
pub struct IntermediateRepresentation<P, I> {
    program: P,
    iter: I,
}

impl<'a, P> IntermediateRepresentation<&'a P, Peekable<P::IntoIter>>
where
    P: TokenIterator<'a> + ?Sized,
{
    pub fn new(program: &'a P) -> Self {
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

impl<'a, P> Iterator for IntermediateRepresentation<&'a P, Peekable<P::IntoIter>>
where
    P: TokenIterator<'a> + Index<Range<usize>, Output = P> + ?Sized,
    Range<usize>: SliceIndex<P>,
{
    type Item = Result<Instruction<&'a <P as Index<Range<usize>>>::Output>, IRError>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! instruction_amount {
            ($token:path, $instruction:path, $int:ty) => {{
                let mut amount: $int = 1;

                while let Some(($token, _)) = self.iter.peek() {
                    self.iter.next();
                    amount = amount.wrapping_add(1);
                }

                Some(Ok($instruction(amount)))
            }};
        }

        match self.iter.next()? {
            (Token::MoveRight, _) => {
                instruction_amount!(Token::MoveRight, Instruction::MoveRight, usize)
            }
            (Token::MoveLeft, _) => {
                instruction_amount!(Token::MoveLeft, Instruction::MoveLeft, usize)
            }
            (Token::Increment, _) => {
                instruction_amount!(Token::Increment, Instruction::Increment, u8)
            }
            (Token::Decrement, _) => {
                instruction_amount!(Token::Decrement, Instruction::Decrement, u8)
            }
            (Token::Output, _) => Some(Ok(Instruction::Output)),
            (Token::Input, _) => Some(Ok(Instruction::Input)),
            (Token::LoopStart, start) => {
                let mut open: usize = 1;
                let program = loop {
                    match self.iter.next() {
                        Some((Token::LoopStart, _)) => open += 1,
                        Some((Token::LoopEnd, end)) => {
                            open -= 1;
                            if open == 0 {
                                // The + 1 skips the [ at the start.
                                break &self.program[(start + 1)..end];
                            }
                        }
                        Some(_) => (),
                        None => return Some(Err(IRError::UnmatchedLoopStart)),
                    }
                };

                Some(Ok(Instruction::Loop(InstructionLoop::new(program))))
            }
            (Token::LoopEnd, _) => Some(Err(IRError::UnmatchedLoopEnd)),
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
