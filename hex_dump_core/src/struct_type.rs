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
