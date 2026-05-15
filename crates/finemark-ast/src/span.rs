use serde::{Deserialize, Serialize};

use super::Element;

/// 위치 정보 (바이트 오프셋)
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