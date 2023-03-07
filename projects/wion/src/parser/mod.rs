use crate::{WasiError, WasiValue};
use std::str::FromStr;

impl FromStr for WasiValue {
    type Err = WasiError;

    fn from_str(s: &str) -> Result<Self, WasiError> {
        todo!()
    }
}

impl WasiValue {
    pub fn parse(text: &str) -> Result<Self, Vec<WasiError>> {
        todo!()
    }
}
