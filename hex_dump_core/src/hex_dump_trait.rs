use crate::hex_dump_field_info::FieldInfo;
use crate::hex_dump_error::HexDumpError;
use crate::struct_type::DecodeType;
use core::fmt::Write;
use core::result::Result;


pub trait HexDump{
    const FIELDS: &'static [FieldInfo];
    const LINE: &'static str = "-----------------------------------------------------------";

    fn hex_dump(bytes: &[u8], row: usize, decode_type: DecodeType) -> Result<String, HexDumpError>{ 
        let mut out = String::new();
        writeln!(out, "{}", Self::LINE)?;
        
        for (index, f) in Self::FIELDS.iter().enumerate(){
            write!(out, "Row: {:>6}   ", row + index )?;
            let slice = &bytes[f.offset .. f.offset + f.type_field.get_size() as usize];
            let nr = f.type_field.decode(slice, decode_type);
            for i in 0..f.type_field.get_size(){ 
                let b = bytes[f.offset + i];
                write!(out, "{b:02x} ")?;
            }

            write!(out, " | {}: {}", f.name, nr)?;
            write!(out, "\n")?;
        }

        writeln!(out, "{}", Self::LINE)?;
        Ok(out)
    }
}
