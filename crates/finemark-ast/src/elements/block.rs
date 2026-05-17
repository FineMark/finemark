use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HeadingElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub marker_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_open_span: Option<Span>,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_close_span: Option<Span>,
    pub level: u8,
    pub section_index: usize,
    pub parameters: Parameters<'i>,
    pub children: Vec<Element<'i>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockQuoteElement<'i> {
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
pub struct HLineElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters<'i>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CodeBlockElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub parameters: Parameters<'i>,
    pub value: &'i str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParagraphBreakElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
}
