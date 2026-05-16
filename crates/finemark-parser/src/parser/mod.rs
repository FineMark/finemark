use crate::context::ParseContext;
use winnow::stream::Stateful;

mod element;
mod input_source;
mod utils;

pub use block::block_document_parser;
pub use input_source::{InputSource, SourceSegment};
pub type ParserInput<'input> = Stateful<InputSource<'input>, ParseContext>;
