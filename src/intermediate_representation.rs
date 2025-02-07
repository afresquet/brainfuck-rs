use core::{iter::Peekable, str::Chars};

use thiserror::Error;

use crate::{Instruction, Lexer, Token};

/// Transformer from [`Token`]s to [`Instruction`]s.
#[derive(Debug)]
pub struct IntermediateRepresentation<'a, I> {
    program: &'a str,
    iter: I,
}

impl<'a> IntermediateRepresentation<'a, Peekable<Lexer<Chars<'a>>>> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            iter: Lexer::new(program.chars()).peekable(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IRError {
    #[error("unmatched '[' encountered")]
    UnmatchedLoopStart,
    #[error("unmatched ']' encountered")]
    UnmatchedLoopEnd,
}

impl<'a, I> Iterator for IntermediateRepresentation<'a, Peekable<I>>
where
    I: Iterator<Item = (Token, usize)>,
{
    type Item = Result<Instruction<'a>, IRError>;

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

                Some(Ok(Instruction::Loop { program }))
            }
            (Token::LoopEnd, _) => Some(Err(IRError::UnmatchedLoopEnd)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_instructions() {
        let input = ">>><<++++-..,[[--->>]][++<]";
        let mut ir = IntermediateRepresentation::new(input);
        assert_eq!(ir.next(), Some(Ok(Instruction::MoveRight(3))));
        assert_eq!(ir.next(), Some(Ok(Instruction::MoveLeft(2))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(4))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Decrement(1))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Output)));
        assert_eq!(ir.next(), Some(Ok(Instruction::Output)));
        assert_eq!(ir.next(), Some(Ok(Instruction::Input)));
        let Some(Ok(Instruction::Loop { program })) = ir.next() else {
            panic!("should be a loop");
        };
        assert_eq!(program, "[--->>]");
        let Some(Ok(Instruction::Loop { program })) =
            IntermediateRepresentation::new(program).next()
        else {
            panic!("should be a loop");
        };
        assert_eq!(program, "--->>");
        let mut inner_instruction_loop = IntermediateRepresentation::new(program);
        assert_eq!(
            inner_instruction_loop.next(),
            Some(Ok(Instruction::Decrement(3)))
        );
        assert_eq!(
            inner_instruction_loop.next(),
            Some(Ok(Instruction::MoveRight(2)))
        );
        let Some(Ok(Instruction::Loop { program })) = ir.next() else {
            panic!("should be a loop");
        };
        assert_eq!(program, "++<");
        let mut instruction_loop = IntermediateRepresentation::new(program);
        assert_eq!(instruction_loop.next(), Some(Ok(Instruction::Increment(2))));
        assert_eq!(instruction_loop.next(), Some(Ok(Instruction::MoveLeft(1))));
        assert_eq!(ir.next(), None);
    }

    #[test]
    fn amount_wraps_around() {
        let input: String = core::iter::repeat_n('+', 260).collect();
        let mut ir = IntermediateRepresentation::new(&input);
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(4))));
        assert_eq!(ir.next(), None);
    }

    #[test]
    fn errors_on_unmatched_loop_start() {
        let input = "+++[---";
        let mut ir = IntermediateRepresentation::new(input);
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(3))));
        assert_eq!(ir.next(), Some(Err(IRError::UnmatchedLoopStart)));
    }

    #[test]
    fn errors_on_unmatched_loop_end() {
        let input = "+++]---";
        let mut ir = IntermediateRepresentation::new(input);
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(3))));
        assert_eq!(ir.next(), Some(Err(IRError::UnmatchedLoopEnd)));
    }
}
