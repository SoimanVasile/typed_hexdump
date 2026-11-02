use bytemuck::{Pod, Zeroable};
use crate::value::Value;

pub enum StructType{
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

#[derive(Clone, Copy)]
pub enum DecodeType{
    BigEndian,
    LittleEndian,
    NativeEndian,
}

impl StructType{
    pub fn get_size(&self) -> usize{
        match self{
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
            Self::I8 =>  1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }

    pub fn representation<T: Pod + Zeroable>(&self, bytes: &[u8]) -> Result<T, ()>{
        if bytes.len() != size_of::<T>(){
            return Err(());
        }

        Ok(*bytemuck::from_bytes(bytes))
    }
    fn decode_ne(&self, bytes: &[u8])  -> Value  {
        macro_rules! native_endian{
            ($variant:ident, $t:ty) => {{
                let v = <$t>::from_ne_bytes(bytes.try_into().unwrap()); 
                Value::$variant(v)                    
            }};
        }
        match self {
            Self::U8  => native_endian!(U8, u8),   Self::I8  => native_endian!(I8, i8),
            Self::U16 => native_endian!(U16, u16), Self::I16 => native_endian!(I16, i16),
            Self::U32 => native_endian!(U32, u32), Self::I32 => native_endian!(I32, i32),
            Self::U64 => native_endian!(U64, u64), Self::I64 => native_endian!(I64, i64),
            Self::F32 => native_endian!(F32, f32), Self::F64 => native_endian!(F64, f64),
        }
    }

    fn decode_le(&self, bytes: &[u8]) -> Value{

        macro_rules! little_endian{
            ($variant:ident, $t:ty) => {{
                let v = <$t>::from_le_bytes(bytes.try_into().unwrap()); 
                Value::$variant(v)                    
            }};
        }
        match self {
            Self::U8  => little_endian!(U8, u8),   Self::I8  => little_endian!(I8, i8),
            Self::U16 => little_endian!(U16, u16), Self::I16 => little_endian!(I16, i16),
            Self::U32 => little_endian!(U32, u32), Self::I32 => little_endian!(I32, i32),
            Self::U64 => little_endian!(U64, u64), Self::I64 => little_endian!(I64, i64),
            Self::F32 => little_endian!(F32, f32), Self::F64 => little_endian!(F64, f64),
        }
    }


    fn decode_be(&self, bytes: &[u8]) -> Value{

        macro_rules! big_endian{
            ($variant:ident, $t:ty) => {{
                let v = <$t>::from_be_bytes(bytes.try_into().unwrap()); 
                Value::$variant(v)                    
            }};
        }
        match self {
            Self::U8  => big_endian!(U8, u8),   Self::I8  => big_endian!(I8, i8),
            Self::U16 => big_endian!(U16, u16), Self::I16 => big_endian!(I16, i16),
            Self::U32 => big_endian!(U32, u32), Self::I32 => big_endian!(I32, i32),
            Self::U64 => big_endian!(U64, u64), Self::I64 => big_endian!(I64, i64),
            Self::F32 => big_endian!(F32, f32), Self::F64 => big_endian!(F64, f64),
        }
    }

    pub fn decode(&self, bytes: &[u8], decode_type: DecodeType) -> Value{
       match decode_type{
            DecodeType::BigEndian => self.decode_be(bytes),
            DecodeType::LittleEndian => self.decode_le(bytes),
            DecodeType::NativeEndian => self.decode_ne(bytes),
       } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_type_sizes() {
        assert_eq!(StructType::U8.get_size(), 1);
        assert_eq!(StructType::U16.get_size(), 2);
        assert_eq!(StructType::U32.get_size(), 4);
        assert_eq!(StructType::U64.get_size(), 8);

        assert_eq!(StructType::I8.get_size(), 1);
        assert_eq!(StructType::I16.get_size(), 2);
        assert_eq!(StructType::I32.get_size(), 4);
        assert_eq!(StructType::I64.get_size(), 8);

        assert_eq!(StructType::F32.get_size(), 4);
        assert_eq!(StructType::F64.get_size(), 8);
    }

    #[test]
    fn test_decode_little_endian() {
        assert_eq!(StructType::U8.decode(&[0x12], DecodeType::LittleEndian), Value::U8(0x12));
        assert_eq!(StructType::U16.decode(&[0x34, 0x12], DecodeType::LittleEndian), Value::U16(0x1234));
        assert_eq!(StructType::U32.decode(&[0x78, 0x56, 0x34, 0x12], DecodeType::LittleEndian), Value::U32(0x12345678));
        assert_eq!(
            StructType::U64.decode(&[0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01], DecodeType::LittleEndian),
            Value::U64(0x0123456789abcdef)
        );

        assert_eq!(StructType::I8.decode(&[0xfe], DecodeType::LittleEndian), Value::I8(-2));
        assert_eq!(StructType::I16.decode(&[0xfe, 0xff], DecodeType::LittleEndian), Value::I16(-2));
        assert_eq!(StructType::I32.decode(&[0xfe, 0xff, 0xff, 0xff], DecodeType::LittleEndian), Value::I32(-2));
        assert_eq!(
            StructType::I64.decode(&[0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], DecodeType::LittleEndian),
            Value::I64(-2)
        );

        let f32_bytes = 1.5f32.to_le_bytes();
        assert_eq!(StructType::F32.decode(&f32_bytes, DecodeType::LittleEndian), Value::F32(1.5));

        let f64_bytes = 3.14159f64.to_le_bytes();
        assert_eq!(StructType::F64.decode(&f64_bytes, DecodeType::LittleEndian), Value::F64(3.14159));
    }

    #[test]
    fn test_decode_big_endian() {
        assert_eq!(StructType::U16.decode(&[0x12, 0x34], DecodeType::BigEndian), Value::U16(0x1234));
        assert_eq!(StructType::U32.decode(&[0x12, 0x34, 0x56, 0x78], DecodeType::BigEndian), Value::U32(0x12345678));
        assert_eq!(
            StructType::U64.decode(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], DecodeType::BigEndian),
            Value::U64(0x0123456789abcdef)
        );

        assert_eq!(StructType::I16.decode(&[0xff, 0xfe], DecodeType::BigEndian), Value::I16(-2));
        assert_eq!(StructType::I32.decode(&[0xff, 0xff, 0xff, 0xfe], DecodeType::BigEndian), Value::I32(-2));

        let f32_bytes = 1.5f32.to_be_bytes();
        assert_eq!(StructType::F32.decode(&f32_bytes, DecodeType::BigEndian), Value::F32(1.5));

        let f64_bytes = 3.14159f64.to_be_bytes();
        assert_eq!(StructType::F64.decode(&f64_bytes, DecodeType::BigEndian), Value::F64(3.14159));
    }

    #[test]
    fn test_decode_native_endian() {
        let val: u32 = 42;
        let bytes = val.to_ne_bytes();
        assert_eq!(StructType::U32.decode(&bytes, DecodeType::NativeEndian), Value::U32(42));
    }

    #[test]
    fn test_representation() {
        let bytes = 12345u32.to_ne_bytes();
        let repr: Result<u32, ()> = StructType::U32.representation(&bytes);
        assert_eq!(repr, Ok(12345u32));

        let invalid_slice = &[0u8; 2];
        let repr_err: Result<u32, ()> = StructType::U32.representation(invalid_slice);
        assert_eq!(repr_err, Err(()));
    }
}
