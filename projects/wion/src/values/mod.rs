mod convert;

/// https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
pub enum WasiValue {
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Float32(f32),
    Float64(f64),
    Boolean(bool),
    Unicode(char),
}

