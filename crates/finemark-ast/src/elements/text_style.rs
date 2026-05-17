use crate::{Element, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TextStyleElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub children: Vec<Element>,
}
