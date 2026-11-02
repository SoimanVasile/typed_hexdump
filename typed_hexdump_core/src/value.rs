use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    U8(u8), U16(u16), U32(u32), U64(u64),
    I8(i8), I16(i16), I32(i32), I64(i64),
    F32(f32), F64(f64),
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::U8(v)  => fmt::Display::fmt(v, f),
            Value::U16(v) => fmt::Display::fmt(v, f),
            Value::U32(v) => fmt::Display::fmt(v, f),
            Value::U64(v) => fmt::Display::fmt(v, f),
            Value::I8(v)  => fmt::Display::fmt(v, f),
            Value::I16(v) => fmt::Display::fmt(v, f),
            Value::I32(v) => fmt::Display::fmt(v, f),
            Value::I64(v) => fmt::Display::fmt(v, f),
            Value::F32(v) => fmt::Display::fmt(v, f),
            Value::F64(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display() {
        assert_eq!(Value::U8(10).to_string(), "10");
        assert_eq!(Value::U16(1000).to_string(), "1000");
        assert_eq!(Value::U32(100000).to_string(), "100000");
        assert_eq!(Value::U64(10000000000).to_string(), "10000000000");

        assert_eq!(Value::I8(-5).to_string(), "-5");
        assert_eq!(Value::I16(-500).to_string(), "-500");
        assert_eq!(Value::I32(-50000).to_string(), "-50000");
        assert_eq!(Value::I64(-5000000000).to_string(), "-5000000000");

        assert_eq!(Value::F32(2.5).to_string(), "2.5");
        assert_eq!(Value::F64(3.1415).to_string(), "3.1415");
    }
}

