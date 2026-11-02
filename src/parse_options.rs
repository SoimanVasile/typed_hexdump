use std::path::PathBuf;

use typed_hexdump_core::struct_type::DecodeType;

use crate::hex_dump_error::HexDumpError;

pub struct ParseStruct{
    pub (crate) path: PathBuf,
    pub(crate) offset: u64,
    pub(crate) length: u64,
    pub(crate) counter: u64,
    pub(crate) decode_type: DecodeType,
}

impl ParseStruct{
    pub fn new(path: PathBuf) -> Self{
        Self{
            path,
            offset: 0,
            length: u64::MAX,
            counter: 1,
            decode_type: DecodeType::NativeEndian,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }

    pub fn decode_type(&self) -> DecodeType {
        self.decode_type
    }

    fn from_parse_options(parse_options: ParseOptions) -> Result<Self, HexDumpError>{
        
        if let Some(path) = parse_options.path{
            Ok(
                Self{
                    path,
                    counter: parse_options.counter,
                    offset: parse_options.offset,
                    length: parse_options.length,
                    decode_type: parse_options.decode_type,
                }
            )
        } else {
            Err(HexDumpError::NoPath)
        }
    }
}


pub struct ParseOptions{
    path: Option<PathBuf>,
    offset: u64,
    length: u64,
    counter: u64,
    decode_type: DecodeType,
}


impl ParseOptions {
    pub fn offset(self, offset: u64) -> Self{
        Self{
            offset,
            ..self
        }
    }

    pub fn length(self, length: u64) -> Self{
        Self{
            length,
            ..self
        }
    }

    pub fn counter(self, counter: u64) -> Self{
        Self{
            counter,
            ..self
        }
    }

    pub fn decode_type(self, decode_type: DecodeType) -> Self{
        Self{
            decode_type,
            ..self
        }
    }
        
    pub fn create(path: PathBuf) -> Self{
        Self{
            path: Some(path),
            counter: 1,
            offset: 0,
            length: u64::MAX,
            decode_type: DecodeType::NativeEndian,
        }
    }

    pub fn parse(self) -> Result<ParseStruct, HexDumpError>{
        ParseStruct::from_parse_options(self)
    }
}
