use crate::{Element, Parameters, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ListItem<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub children: Vec<Element<'i>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListElement<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub parameters: Parameters<'i>,
    pub items: Vec<ListItem<'i>>,
}
