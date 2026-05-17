use crate::parser::ParserInput;
use crate::parser::utils::line_break;
use finemark_ast::{Element, EscapeElement, HardBreakElement, Span};
use winnow::Result;
use winnow::combinator::{alt, preceded};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take};

pub fn escape_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    alt((hard_break_escape_parser, character_escape_parser)).parse_next(parser_input)
}

fn hard_break_escape_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    preceded(literal("\\"), line_break).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::HardBreak(HardBreakElement {
        span: Span { start, end },
    }))
}

fn character_escape_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    let parsed_content = preceded(literal("\\"), take(1usize)).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Escape(EscapeElement {
        span: Span { start, end },
        value: parsed_content,
    }))
}
