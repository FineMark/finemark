use crate::context::ParseGuard;
use crate::parser::ParserInput;
use crate::parser::markdown::utils::parse_text_style;
use finemark_ast::Element;
use winnow::Result;

pub(crate) fn markdown_subscript_parser(parser_input: &mut ParserInput) -> Result<Element> {
    parse_text_style(
        parser_input,
        ",,",
        ParseGuard::Subscript,
        Element::Subscript,
    )
}
