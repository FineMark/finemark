use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TableElement<'i> {
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
pub struct TableRowElement<'i> {
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
pub struct TableColumnElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_open_span: Option<Span>,
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub body_close_span: Option<Span>,
    pub parameters: Parameters<'i>,
    pub children: Vec<Element<'i>>,
}
