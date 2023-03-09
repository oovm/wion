use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

/// `+[read, "write"]`
#[derive(Clone, Debug)]
pub struct IncludeFlags {
    pub flags: Vec<Arc<str>>,
}

/// `-[read, "write"]`
#[derive(Clone, Debug)]
pub struct ExcludeFlags {
    pub flags: Vec<Arc<str>>,
}
