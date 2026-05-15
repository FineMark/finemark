use crate::context::ParseContext;
use winnow::stream::Stateful;

mod utils;
mod input_source;
mod element;

pub use block::block_document_parser;
pub use input_source::{InputSource, SourceSegment};
pub type ParserInput<'input> = Stateful<InputSource<'input>, ParseContext>;