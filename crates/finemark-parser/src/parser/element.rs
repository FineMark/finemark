use crate::parser::ParserInput;
use finemark_ast::Element;
use winnow::Result;
use winnow::combinator::repeat;
use winnow::prelude::*;

pub(crate) fn element_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let _ = parser_input;

    todo!("Implement a single-element dispatcher that matches the SevenMark parser structure")
}

pub(crate) fn inline_content_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    repeat(1.., element_parser).parse_next(parser_input)
}
