use std::iter::Peekable;

use thiserror::Error;

use crate::{Instruction, Token};

#[derive(Debug)]
pub struct IntermediateRepresentation<I> {
    iter: I,
}

impl<I> IntermediateRepresentation<Peekable<I>>
where
    I: Iterator<Item = Token>,
{
    pub fn new(iter: Peekable<I>) -> Self {
        Self { iter }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IRError {
    #[error("unmatched '[' encountered")]
    UnmatchedLoopStart,
    #[error("unmatched ']' encountered")]
    UnmatchedLoopEnd,
}

impl<I> Iterator for IntermediateRepresentation<Peekable<I>>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<Instruction, IRError>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! instruction_amount {
            ($token:path, $instruction:path, $int:ty) => {{
                let mut amount: $int = 1;

                while let Some($token) = self.iter.peek() {
                    self.iter.next();
                    amount = amount.wrapping_add(1);
                }

                Some(Ok($instruction(amount)))
            }};
        }

        match self.iter.next()? {
            Token::MoveRight => {
                instruction_amount!(Token::MoveRight, Instruction::MoveRight, usize)
            }
            Token::MoveLeft => {
                instruction_amount!(Token::MoveLeft, Instruction::MoveLeft, usize)
            }
            Token::Increment => {
                instruction_amount!(Token::Increment, Instruction::Increment, u8)
            }
            Token::Decrement => {
                instruction_amount!(Token::Decrement, Instruction::Decrement, u8)
            }
            Token::Output => Some(Ok(Instruction::Output)),
            Token::Input => Some(Ok(Instruction::Input)),
            Token::LoopStart => {
                let mut instructions = Vec::new();

                loop {
                    if matches!(self.iter.peek(), Some(Token::LoopEnd)) {
                        self.iter.next();
                        break;
                    }

                    match self.next() {
                        Some(Ok(instruction)) => instructions.push(instruction),
                        Some(Err(error)) => return Some(Err(error)),
                        None => return Some(Err(IRError::UnmatchedLoopStart)),
                    }
                }

                Some(Ok(Instruction::Loop(instructions)))
            }
            Token::LoopEnd => Some(Err(IRError::UnmatchedLoopEnd)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Lexer;

    use super::*;

    #[test]
    fn parses_instructions() {
        let input = ">>><<++++-..,[[--->>]][++<]";
        let lexer = Lexer::new(input.chars());
        let mut ir = IntermediateRepresentation::new(lexer.peekable());
        assert_eq!(ir.next(), Some(Ok(Instruction::MoveRight(3))));
        assert_eq!(ir.next(), Some(Ok(Instruction::MoveLeft(2))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(4))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Decrement(1))));
        assert_eq!(ir.next(), Some(Ok(Instruction::Output)));
        assert_eq!(ir.next(), Some(Ok(Instruction::Output)));
        assert_eq!(ir.next(), Some(Ok(Instruction::Input)));
        assert_eq!(
            ir.next(),
            Some(Ok(Instruction::Loop(vec![Instruction::Loop(vec![
                Instruction::Decrement(3),
                Instruction::MoveRight(2)
            ])])))
        );
        assert_eq!(
            ir.next(),
            Some(Ok(Instruction::Loop(vec![
                Instruction::Increment(2),
                Instruction::MoveLeft(1)
            ])))
        );
        assert_eq!(ir.next(), None);
    }

    #[test]
    fn amount_wraps_around() {
        let input = std::iter::repeat_n('+', 260);
        let lexer = Lexer::new(input);
        let mut ir = IntermediateRepresentation::new(lexer.peekable());
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(4))));
        assert_eq!(ir.next(), None);
    }

    #[test]
    fn errors_on_unmatched_loop_start() {
        let input = "+++[---";
        let lexer = Lexer::new(input.chars());
        let mut ir = IntermediateRepresentation::new(lexer.peekable());
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(3))));
        assert_eq!(ir.next(), Some(Err(IRError::UnmatchedLoopStart)));
    }

    #[test]
    fn errors_on_unmatched_loop_end() {
        let input = "+++]---";
        let lexer = Lexer::new(input.chars());
        let mut ir = IntermediateRepresentation::new(lexer.peekable());
        assert_eq!(ir.next(), Some(Ok(Instruction::Increment(3))));
        assert_eq!(ir.next(), Some(Err(IRError::UnmatchedLoopEnd)));
    }
}
