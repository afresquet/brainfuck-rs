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
    use std::io::{self, ErrorKind, Read, Write};

    use super::{InputStream, OutputStream};

    impl<T> InputStream for T
    where
        T: Read,
    {
        type Error = io::Error;

        fn read(&mut self) -> Result<u8, Self::Error> {
            self.bytes()
                .next()
                .ok_or_else(|| io::Error::new(ErrorKind::UnexpectedEof, "unexpected EOF"))?
        }
    }

    impl<T> OutputStream for T
    where
        T: Write,
    {
        type Error = io::Error;

        fn write(&mut self, value: u8) -> Result<(), Self::Error> {
            self.write_all(&[value])
        }
    }
}
