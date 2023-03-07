use super::*;

impl From<u8> for WasiValue {
    fn from(value: u8) -> Self {
        Self::Unsigned8(value)
    }
}

impl From<u16> for WasiValue {
    fn from(value: u16) -> Self {
        Self::Unsigned16(value)
    }
}

impl From<u32> for WasiValue {
    fn from(value: u32) -> Self {
        Self::Unsigned32(value)
    }
}

impl From<u64> for WasiValue {
    fn from(value: u64) -> Self {
        Self::Unsigned64(value)
    }
}

impl From<i8> for WasiValue {
    fn from(value: i8) -> Self {
        Self::Integer8(value)
    }
}

impl From<i16> for WasiValue {
    fn from(value: i16) -> Self {
        Self::Integer16(value)
    }
}

impl From<i32> for WasiValue {
    fn from(value: i32) -> Self {
        Self::Integer32(value)
    }
}

impl From<i64> for WasiValue {
    fn from(value: i64) -> Self {
        Self::Integer64(value)
    }
}

impl From<f32> for WasiValue {
    fn from(value: f32) -> Self {
        Self::Float32(value)
    }
}

impl From<f64> for WasiValue {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

