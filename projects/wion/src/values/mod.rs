mod convert;
#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod der;

/// https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
pub enum WasiValue {
    /// The boolean value `true` or `false`
    Boolean(bool),
    /// The 8-bit unsigned integer from `0` to `255`
    Unsigned8(u8),
    /// The 16-bit unsigned integer from `0` to `65535`
    Unsigned16(u16),
    /// The 32-bit unsigned integer from `0` to `4294967295`
    Unsigned32(u32),
    /// The 64-bit unsigned integer from `0` to `18446744073709551615`
    Unsigned64(u64),
    /// The 8-bit signed integer from `-128` to `127`
    Integer8(i8),
    /// The 16-bit signed integer from `-32768` to `32767`
    Integer16(i16),
    /// The 32-bit signed integer from `-2147483648` to `2147483647`
    Integer32(i32),
    /// The 64-bit signed integer from `-9223372036854775808` to `9223372036854775807`
    Integer64(i64),
    /// The 32-bit floating point number
    Float32(f32),
    /// The 64-bit floating point number
    Float64(f64),
    /// The UTF-8 character
    Unicode(char),
    /// The UTF-8 string
    UTF8(String),
}

