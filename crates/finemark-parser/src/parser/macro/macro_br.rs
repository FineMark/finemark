use crate::parser::ParserInput;
use finemark_ast::{Element, HardBreakElement, Span};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub(crate) fn macro_br_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    literal("@br").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::HardBreak(HardBreakElement {
        span: Span { start, end },
    }))
}
