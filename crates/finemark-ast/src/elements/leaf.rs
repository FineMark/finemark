use crate::{Element, Parameters, Span};
use serde::Serialize;

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
pub struct SoftBreakElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct HardBreakElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub span: Span,
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
pub struct LinkElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineCodeElement {
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
