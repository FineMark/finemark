use crate::Span;
use serde::Serialize;

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

#[derive(Debug, Clone, Serialize)]
pub struct CodeElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeXElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub is_block: bool,
    pub value: String,
}
