use crate::parser::ParserInput;
use finemark_ast::{Element, Span, TextElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Fallback parser for an `@` that does not match any known AT command.
/// Consumes exactly `@` and emits it as a `Text` element, allowing the
/// document parser to continue past the unrecognised marker.
pub fn token_at_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    literal("@").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Text(TextElement {
        span: Span { start, end },
        value: "@",
    }))
}
