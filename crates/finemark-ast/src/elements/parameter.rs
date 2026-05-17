use crate::{Element, Span};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Parameter {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub key: String,
    pub value: Vec<Element>,
}

/// Parameter map that keeps source order while supporting direct key lookup.
pub type Parameters = IndexMap<String, Parameter>;
