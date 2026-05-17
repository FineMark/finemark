use crate::context::ParseContext;
use winnow::stream::Stateful;

mod block;
mod comment;
pub(crate) mod document;
pub(crate) mod element;
mod escape;
mod input_source;
mod markdown;
mod text;
mod token;
mod utils;

pub(crate) use block::block_document_parser;
pub use input_source::{InputSource, SourceSegment};
pub type ParserInput<'input> = Stateful<InputSource<'input>, ParseContext>;
