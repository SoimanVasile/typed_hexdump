use core::fmt::Error;

#[derive(Debug)]
pub enum HexDumpError{
    WriteLine(Error),
}

impl From<Error> for HexDumpError{
    fn from(e: Error) -> Self{
        Self::WriteLine(e)
    }
}

