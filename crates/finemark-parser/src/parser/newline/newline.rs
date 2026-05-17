use crate::parser::utils::line_break;
use crate::parser::ParserInput;
use finemark_ast::{Element, SoftBreakElement, Span};
use winnow::stream::Location as StreamLocation;
use winnow::Result;

pub fn token_newline_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    line_break(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::SoftBreak(SoftBreakElement {
        span: Span { start, end },
    }))
}
