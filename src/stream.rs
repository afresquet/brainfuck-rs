pub trait InputStream {
    type Error;

    fn read(&mut self) -> Result<u8, Self::Error>;
}

pub trait OutputStream {
    type Error;

    fn write(&mut self, value: u8) -> Result<(), Self::Error>;
}

#[cfg(feature = "std")]
pub mod stdio {
    use std::io::{Read, Stdin, Stdout, Write};

    use thiserror::Error;

    use super::{InputStream, OutputStream};

    impl InputStream for Stdin {
        type Error = StdinError;

        fn read(&mut self) -> Result<u8, Self::Error> {
            let mut input = [0; 5];
            let len = Read::read(self, &mut input)?;
            Ok(parse_input(&input, len)?)
        }
    }

    impl OutputStream for Stdout {
        type Error = std::io::Error;

        fn write(&mut self, value: u8) -> Result<(), Self::Error> {
            self.write_all(&[value])?;
            Ok(())
        }
    }

    fn parse_input(input: &[u8], len: usize) -> Result<u8, ParseInputError> {
        assert!(input.len() >= len);

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
            _ => Err(ParseInputError::BadInput),
        }
    }

    #[derive(Debug, Error)]
    pub enum StdinError {
        #[error("error from stdin: {0}")]
        Stdin(#[from] std::io::Error),
        #[error("error parsing input: {0}")]
        ParseInput(#[from] ParseInputError),
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
            let mut input = [0; 5];
            input[0] = '9' as u8;
            assert_eq!(parse_input(&input, 2), Ok(9));
            input[0] = '5' as u8;
            assert_eq!(parse_input(&input, 2), Ok(5));
        }

        #[test]
        fn parse_input_two_digit() {
            let mut input = [0; 5];
            input[0] = '9' as u8;
            input[1] = '5' as u8;
            assert_eq!(parse_input(&input, 3), Ok(95));
            input[0] = '2' as u8;
            input[1] = '8' as u8;
            assert_eq!(parse_input(&input, 3), Ok(28));
        }

        #[test]
        fn parse_input_three_digit() {
            let mut input = [0; 5];
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
            assert_eq!(parse_input(&[0; 0], 0), Err(ParseInputError::BadInput));
            assert_eq!(parse_input(&[0; 1], 1), Err(ParseInputError::BadInput));
            assert_eq!(parse_input(&[0; 5], 5), Err(ParseInputError::BadInput));
            assert_eq!(parse_input(&[0; 10], 10), Err(ParseInputError::BadInput));
        }

        #[test]
        fn parse_input_invalid_character_error() {
            let mut input = [0; 5];
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
            let mut input = [0; 5];
            input[0] = '2' as u8;
            input[1] = '5' as u8;
            input[2] = '6' as u8;
            assert_eq!(parse_input(&input, 4), Err(ParseInputError::NumberTooBig));
        }

        #[test]
        #[should_panic]
        fn parse_input_panic_if_bad_len() {
            assert_eq!(parse_input(&[], 1), Err(ParseInputError::BadInput));
        }
    }
}
