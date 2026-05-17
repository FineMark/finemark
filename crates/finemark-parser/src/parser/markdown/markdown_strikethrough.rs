use crate::context::ParseGuard;
use crate::parser::ParserInput;
use crate::parser::markdown::utils::parse_text_style;
use finemark_ast::Element;
use winnow::Result;

pub(crate) fn markdown_strikethrough_parser<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<Element<'i>> {
    parse_text_style(
        parser_input,
        "~~",
        ParseGuard::Strikethrough,
        Element::Strikethrough,
    )
}
