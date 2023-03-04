use super::*;


impl From<i8> for WasiValue {
    fn from(value: i8) -> Self {
        Self::Integer8(value)
    }
}

