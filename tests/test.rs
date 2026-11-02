extern crate typed_hexdump;

use typed_hexdump::hex_dump_trait::HexDump;
use typed_hexdump::parse_options::ParseOptions;
use typed_hexdump_core::struct_type::DecodeType;
use typed_hexdump_derive::HexDump;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(HexDump)]
#[repr(C, packed)]
struct Test {
    magic: u32,
    pub number: u16,
}

#[derive(HexDump)]
#[repr(C, packed)]
struct AllPrimitives {
    f_u8: u8,
    f_u16: u16,
    f_u32: u32,
    f_u64: u64,
    f_i8: i8,
    f_i16: i16,
    f_i32: i32,
    f_i64: i64,
    f_f32: f32,
    f_f64: f64,
}

#[test]
fn test_macro_basic() {
    let bytes: Vec<u8> = vec![1, 0, 0, 0, 4, 0];

    let out = r#"-----------------------------------------------------------
Row:      0   01 00 00 00  | magic: 1
Row:      1   04 00  | number: 4
-----------------------------------------------------------
"#;
    assert_eq!(
        out,
        Test::hex_dump(&bytes, 0, DecodeType::LittleEndian).unwrap()
    );
}

#[test]
fn test_macro_all_primitives() {
    // 1 + 2 + 4 + 8 + 1 + 2 + 4 + 8 + 4 + 8 = 42 bytes
    assert_eq!(AllPrimitives::SELF_SIZE, 42);
    assert_eq!(AllPrimitives::FIELDS.len(), 10);

    let mut bytes = Vec::new();
    bytes.push(0x42); // u8
    bytes.extend_from_slice(&1000u16.to_le_bytes()); // u16
    bytes.extend_from_slice(&100000u32.to_le_bytes()); // u32
    bytes.extend_from_slice(&10000000000u64.to_le_bytes()); // u64
    bytes.push((-10i8) as u8); // i8
    bytes.extend_from_slice(&(-500i16).to_le_bytes()); // i16
    bytes.extend_from_slice(&(-50000i32).to_le_bytes()); // i32
    bytes.extend_from_slice(&(-5000000000i64).to_le_bytes()); // i64
    bytes.extend_from_slice(&1.5f32.to_le_bytes()); // f32
    bytes.extend_from_slice(&3.14159f64.to_le_bytes()); // f64

    let output = AllPrimitives::hex_dump(&bytes, 10, DecodeType::LittleEndian).unwrap();

    assert!(output.contains("Row:     10"));
    assert!(output.contains("f_u8: 66"));
    assert!(output.contains("f_u16: 1000"));
    assert!(output.contains("f_u32: 100000"));
    assert!(output.contains("f_u64: 10000000000"));
    assert!(output.contains("f_i8: -10"));
    assert!(output.contains("f_i16: -500"));
    assert!(output.contains("f_i32: -50000"));
    assert!(output.contains("f_i64: -5000000000"));
    assert!(output.contains("f_f32: 1.5"));
    assert!(output.contains("f_f64: 3.14159"));
}

#[test]
fn test_endianness_formatting() {
    let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0x01, 0x02];

    let le_out = Test::hex_dump(&bytes, 0, DecodeType::LittleEndian).unwrap();
    assert!(le_out.contains("12 34 56 78  | magic: 2018915346"));
    assert!(le_out.contains("01 02  | number: 513"));

    let be_out = Test::hex_dump(&bytes, 0, DecodeType::BigEndian).unwrap();
    assert!(be_out.contains("12 34 56 78  | magic: 305419896"));
    assert!(be_out.contains("01 02  | number: 258"));
}

#[test]
fn test_row_offset_index() {
    let bytes: Vec<u8> = vec![1, 0, 0, 0, 4, 0];
    let out = Test::hex_dump(&bytes, 42, DecodeType::LittleEndian).unwrap();
    assert!(out.contains("Row:     42   01 00 00 00  | magic: 1"));
    assert!(out.contains("Row:     43   04 00  | number: 4"));
}

#[test]
fn test_parse_options_builder() {
    let path = PathBuf::from("/tmp/dummy.bin");
    let options = ParseOptions::create(path.clone())
        .offset(16)
        .length(64)
        .counter(5)
        .decode_type(DecodeType::BigEndian);

    let parsed = options.parse().expect("Expected successful parse");
    assert_eq!(parsed.counter(), 5);
    assert_eq!(parsed.offset(), 16);
    assert_eq!(parsed.length(), 64);
}

#[test]
fn test_parse_file_with_tempfile() {
    let mut temp = NamedTempFile::new().unwrap();
    let bytes: Vec<u8> = vec![
        1, 0, 0, 0, 4, 0, // item 0
        2, 0, 0, 0, 8, 0, // item 1
    ];
    temp.write_all(&bytes).unwrap();
    temp.flush().unwrap();

    let parse_opts = ParseOptions::create(temp.path().to_path_buf())
        .offset(0)
        .length(bytes.len() as u64)
        .counter(2)
        .decode_type(DecodeType::LittleEndian)
        .parse()
        .unwrap();

    // Verify calling parse_file executes without panic
    Test::parse_file(parse_opts);
}

