extern crate hex_dump;

use hex_dump_derive::HexDump;

#[derive(HexDump, Copy, Clone)]
#[repr(C, packed)]
struct Test{
    magic: u32,
    pub number: u16,
}
#[cfg(test)]
#[test]
fn test_macro(){
    use hex_dump_core::{hex_dump_trait::HexDump, struct_type::DecodeType};

    
    let bytes: Vec<u8> = vec![1, 0, 0 ,0, 4, 0];
    
    let out = r#"-----------------------------------------------------------
Row:      0   01 00 00 00  | magic: 1
Row:      1   04 00  | number: 4
-----------------------------------------------------------
"#;
    assert_eq!(out, Test::hex_dump(&bytes, 0, DecodeType::LittleEndian).unwrap());
}
