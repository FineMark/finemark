use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HeadingElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub marker_span: Span,
    pub level: u8,
    pub section_index: usize,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockQuoteElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub marker_spans: Vec<Span>,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HLineElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters,
}

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlockElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub info: Option<String>,
    pub value: String,
}
