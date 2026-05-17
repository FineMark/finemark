use crate::parser::ParserInput;
use crate::parser::utils::line_break;
use finemark_ast::{Element, ParagraphBreakElement, Span};
use winnow::Result;
use winnow::ascii::space0;
use winnow::combinator::{preceded, repeat};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;

pub fn token_paragraph_break_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    line_break(parser_input)?;
    space0.parse_next(parser_input)?;
    line_break(parser_input)?;
    let _: () = repeat(0.., preceded(space0, line_break)).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::ParagraphBreak(ParagraphBreakElement {
        span: Span { start, end },
    }))
}
