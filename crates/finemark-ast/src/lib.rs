mod elements;
mod span;

pub use elements::*;
use serde::Serialize;
pub use span::*;

#[derive(Debug, Clone, Serialize)]
pub enum Element {
    // Basic text elements
    Text(TextElement),
    Comment(CommentElement),
    Escape(EscapeElement),
    Error(ErrorElement),

    // Block elements
    Code(CodeElement),
    TeX(TeXElement),
}
