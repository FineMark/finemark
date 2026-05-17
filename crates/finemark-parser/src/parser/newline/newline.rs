use crate::parser::ParserInput;
use crate::parser::utils::line_break;
use finemark_ast::{Element, SoftBreakElement, Span};
use winnow::Result;
use winnow::stream::Location as StreamLocation;

pub fn token_newline_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    line_break(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::SoftBreak(SoftBreakElement {
        span: Span { start, end },
    }))
}
