use crate::{Element, Span};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Parameter {
    #[cfg_attr(not(feature = "include-locations"), serde(skip_serializing))]
    pub span: Span,
    pub key: String,
    pub value: Vec<Element>,
}

/// FineMark keeps parameter order and duplicates intact at parse time.
/// Semantic layers can normalize this into maps for specific elements.
pub type Parameters = Vec<Parameter>;
