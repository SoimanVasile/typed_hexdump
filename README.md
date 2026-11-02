# typed-hexdump

A structured, schema-aware hex dump library for Rust.

Unlike standard hex dump utilities that dump flat bytes, `typed-hexdump` formats raw binary data using `#[repr(C, packed)]` struct layouts. Each field is broken down row-by-row with its offset, raw hex bytes, field name, and decoded typed value according to your chosen endianness.

## Features

- **Schema-driven hex dumps**: Derive `HexDump` on any `#[repr(C, packed)]` struct.
- **Typed value decoding**: Supports primitive types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`).
- **Configurable endianness**: Decode using `LittleEndian`, `BigEndian`, or `NativeEndian`.
- **Memory-mapped file parsing**: Parse binary records directly from disk using fast memory mapping (`mmap`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typed-hexdump = "0.1.0"
```

## Quick Start

```rust
use typed_hexdump::{HexDump, DecodeType};

#[derive(HexDump)]
#[repr(C, packed)]
struct PacketHeader {
    magic: u32,
    packet_id: u16,
}

fn main() {
    let bytes: [u8; 6] = [0x01, 0x00, 0x00, 0x00, 0x2a, 0x00];

    let dump = PacketHeader::hex_dump(&bytes, 0, DecodeType::LittleEndian).unwrap();
    println!("{}", dump);
}
```

### Output:

```text
-----------------------------------------------------------
Row:      0   01 00 00 00  | magic: 1
Row:      1   2a 00  | packet_id: 42
-----------------------------------------------------------
```

## Parsing Binary Files

You can stream records directly from files:

```rust
use std::path::PathBuf;
use typed_hexdump::{HexDump, ParseOptions, DecodeType};

#[derive(HexDump)]
#[repr(C, packed)]
struct Record {
    id: u32,
    value: f32,
}

fn main() {
    let options = ParseOptions::create(PathBuf::from("data.bin"))
        .offset(0)
        .counter(10) // Parse 10 records
        .decode_type(DecodeType::LittleEndian)
        .parse()
        .unwrap();

    Record::parse_file(options);
}
```

## License

This project is licensed under the [MIT License](LICENSE).
