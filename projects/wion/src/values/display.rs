use crate::WasiValue;
use std::fmt::{Debug, Formatter};

impl Debug for WasiValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(v) => match v {
                true => f.write_str("true"),
                false => f.write_str("false"),
            },
            Self::Unsigned8(v) => {
                write!(f, "{}u8", v)
            }
            Self::Unsigned16(v) => {
                write!(f, "{}u16", v)
            }
            Self::Unsigned32(v) => {
                write!(f, "{}u32", v)
            }
            Self::Unsigned64(v) => {
                write!(f, "{}u64", v)
            }
            Self::Integer8(v) => {
                write!(f, "{}i8", v)
            }
            Self::Integer16(v) => {
                write!(f, "{}i16", v)
            }
            Self::Integer32(v) => {
                write!(f, "{}i32", v)
            }
            Self::Integer64(v) => {
                write!(f, "{}i64", v)
            }
            Self::Float32(v) => {
                write!(f, "{}f32", v)
            }
            Self::Float64(v) => {
                write!(f, "{}f64", v)
            }
            Self::Unicode(v) => {
                write!(f, "{}", v)
            }
            Self::UTF8(v) => {
                write!(f, "{:?}", v)
            }
        }
    }
}

