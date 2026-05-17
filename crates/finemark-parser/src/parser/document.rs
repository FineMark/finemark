use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use finemark_ast::Element;
use winnow::Result;
use winnow::combinator::repeat;
use winnow::prelude::*;

/// Parses a full document as generic FineMark elements.
pub fn document_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Vec<Element<'i>>> {
    repeat(0.., element_parser).parse_next(parser_input)
}
