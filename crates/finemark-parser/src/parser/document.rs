use crate::parser::ParserInput;
use crate::parser::block_document_parser;
use finemark_ast::Element;
use winnow::Result;

/// Parses a full document using block-first dispatch.
pub fn document_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    block_document_parser(parser_input)
}
