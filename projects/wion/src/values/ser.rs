use crate::WasiValue;
use serde::{Serialize, Serializer};

impl Serialize for WasiValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Boolean(v) => serializer.serialize_bool(*v),
            Self::Unsigned8(v) => serializer.serialize_u8(*v),
            Self::Unsigned16(v) => serializer.serialize_u16(*v),
            Self::Unsigned32(v) => serializer.serialize_u32(*v),
            Self::Unsigned64(v) => serializer.serialize_u64(*v),
            Self::Integer8(v) => serializer.serialize_i8(*v),
            Self::Integer16(v) => serializer.serialize_i16(*v),
            Self::Integer32(v) => serializer.serialize_i32(*v),
            Self::Integer64(v) => serializer.serialize_i64(*v),
            Self::Float32(v) => serializer.serialize_f32(*v),
            Self::Float64(v) => serializer.serialize_f64(*v),
            Self::Unicode(v) => serializer.serialize_char(*v),
            Self::UTF8(v) => serializer.serialize_str(v),
            WasiValue::None => {}
            WasiValue::Object(_) => {}
        }
    }
}
