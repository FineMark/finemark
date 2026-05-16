use crate::{Element, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ListItem {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OrderedListStyle {
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ListKind {
    Unordered,
    Ordered { style: OrderedListStyle, start: u64 },
}

#[derive(Debug, Clone, Serialize)]
pub struct ListElement {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub kind: ListKind,
    pub items: Vec<ListItem>,
}
