use serde::{Deserialize, Serialize};

/// Byte offsets within the original source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    /// Creates a synthesized span for elements generated during preprocessing
    pub fn synthesized() -> Self {
        Self { start: 0, end: 0 }
    }
}
