use serde::Serialize;
use crate::span::Span;

// === Text-carrying leaf nodes ===
#[derive(Debug, Clone, Serialize)]
pub struct TextElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EscapeElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: String,
}