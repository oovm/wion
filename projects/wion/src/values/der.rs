use crate::WasiValue;
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};
use std::fmt::Formatter;
use serde::de::{EnumAccess, MapAccess, SeqAccess};

struct WasiVisitor {
    value: WasiValue,
}

#[allow(unused)]
impl<'de> Deserialize<'de> for WasiValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> Visitor<'de> for WasiValue {
    type Value = WasiValue;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
         Ok(WasiValue::Boolean(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Integer8(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Integer16(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Integer32(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Integer64(v))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Unsigned8(v))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Unsigned16(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Unsigned32(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Unsigned64(v))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Float32(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Float64(v))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::Unicode(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::UTF8(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(WasiValue::UTF8(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
        todo!()
    }



    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
        todo!()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
        todo!()
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
        todo!()
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        todo!()
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error> where A: EnumAccess<'de> {
        todo!()
    }
}
