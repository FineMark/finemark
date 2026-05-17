use crate::{Element, Span};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Parameter<'i> {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub key: &'i str,
    pub value: Vec<Element<'i>>,
}

/// Parameter map that keeps source order while supporting direct key lookup.
pub type Parameters<'i> = IndexMap<&'i str, Parameter<'i>>;
