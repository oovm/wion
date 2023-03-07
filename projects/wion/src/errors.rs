use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// The result type for wion
pub type Result<T> = std::result::Result<T, WasiError>;

/// The error type for wion
#[derive(Debug, Copy, Clone)]
pub enum WasiError {
    UnknownError,
}

impl Error for WasiError {}
impl Display for WasiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WasiError::UnknownError => f.write_str("An unknown error occurred"),
        }
    }
}
