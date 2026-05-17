use crate::context::ParseContext;
use winnow::stream::{LocatingSlice, Stateful};

mod at;
mod code;
pub(crate) mod document;
pub(crate) mod element;
mod escape;
mod r#macro;
mod markdown;
mod newline;
mod parameter;
mod text;
mod token;
mod utils;

pub type ParserInput<'input> = Stateful<LocatingSlice<&'input str>, ParseContext<'input>>;
