use crate::parser::ParserInput;
use crate::parser::utils::line_break;
use finemark_ast::{Element, EscapeElement, HardBreakElement, Span};
use winnow::Result;
use winnow::combinator::{alt, preceded};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{any, literal};

pub fn escape_parser(parser_input: &mut ParserInput) -> Result<Element> {
    alt((hard_break_escape_parser, character_escape_parser)).parse_next(parser_input)
}

fn hard_break_escape_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    preceded(literal("\\"), line_break).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::HardBreak(HardBreakElement {
        span: Span { start, end },
    }))
}

fn character_escape_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    let parsed_content = preceded(literal("\\"), any).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Escape(EscapeElement {
        span: Span { start, end },
        value: parsed_content.to_string(),
    }))
}
