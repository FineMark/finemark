use std::fmt;
use winnow::error::ContextError;

#[derive(Debug, Clone, PartialEq)]
pub enum FineMarkError {
    RecursionDepthExceeded { depth: usize, max_depth: usize },
}

impl fmt::Display for FineMarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FineMarkError::RecursionDepthExceeded { depth, max_depth } => {
                write!(f, "Recursion depth exceeded: {} > {}", depth, max_depth)
            }
        }
    }
}

impl std::error::Error for FineMarkError {}

impl FineMarkError {
    /// Converts `FineMarkError` into a `winnow::error::ContextError`.
    pub fn into_context_error(self) -> ContextError {
        ContextError::new()
    }
}
