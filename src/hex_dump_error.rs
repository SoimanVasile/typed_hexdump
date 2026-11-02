
use core::fmt::Error;

#[derive(Debug)]
pub enum HexDumpError{
    WriteLine(Error),
    NoPath,
}

impl From<Error> for HexDumpError{
    fn from(e: Error) -> Self{
        Self::WriteLine(e)
    }
}

