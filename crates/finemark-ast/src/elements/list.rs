use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ListItem {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters,
    pub items: Vec<ListItem>,
}
