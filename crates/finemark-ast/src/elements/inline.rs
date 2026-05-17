use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TextElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_open_span: Option<Span>,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_close_span: Option<Span>,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SoftBreakElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct HardBreakElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct EscapeElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct LinkElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_open_span: Option<Span>,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_close_span: Option<Span>,
    pub parameters: Parameters<'i>,
    pub children: Vec<Element<'i>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InlineCodeElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeXElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub is_block: bool,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct TextStyleElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub children: Vec<Element<'i>>,
}
