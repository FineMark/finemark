use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use finemark_ast::Element;
use winnow::Result;
use winnow::combinator::repeat;
use winnow::prelude::*;

/// Parses a full document as generic FineMark elements.
pub fn document_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    repeat(0.., element_parser).parse_next(parser_input)
}
