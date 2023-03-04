#[derive(Debug, Copy, Clone)]
pub enum WasiError {
    UnknownError
}

pub type Result<T> = std::result::Result<T, WasiError>;
