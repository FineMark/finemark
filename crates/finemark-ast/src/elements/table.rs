use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TableElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableRowElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableColumnElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}
