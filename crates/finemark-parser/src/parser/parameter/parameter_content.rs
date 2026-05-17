use crate::parser::ParserInput;
use crate::parser::escape::escape_parser;
use crate::parser::parameter::parameter_text::parameter_text_parser;
use finemark_ast::Element;
use winnow::Result;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;

pub(crate) fn parameter_content_parser<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<Vec<Element<'i>>> {
    repeat(0.., alt((escape_parser, parameter_text_parser))).parse_next(parser_input)
}
