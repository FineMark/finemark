use crate::context::ParseContext;
use winnow::stream::Stateful;

mod at;
pub(crate) mod document;
pub(crate) mod element;
mod escape;
mod input_source;
mod markdown;
mod parameter;
mod text;
mod token;
mod utils;

pub use input_source::{InputSource, SourceSegment};
pub type ParserInput<'input> = Stateful<InputSource<'input>, ParseContext>;
