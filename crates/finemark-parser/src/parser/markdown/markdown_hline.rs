use crate::parser::ParserInput;
use crate::parser::utils::line_break_or_eof;
use finemark_ast::{Element, HLineElement, Span};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;

pub(crate) fn markdown_hline_parser(parser_input: &mut ParserInput) -> Result<Element> {
    // Policy: an hline is a whole line made of 3 to 9 `-` markers.
    // Other marker styles can be added later, but `-` is the only accepted form now.
    let start = parser_input.current_token_start();
    take_while(3..=9, '-').parse_next(parser_input)?;
    line_break_or_eof(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::HLine(HLineElement {
        span: Span { start, end },
    }))
}
