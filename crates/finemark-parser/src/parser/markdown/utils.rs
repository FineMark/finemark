use crate::context::ParseGuard;
use crate::parser::ParserInput;
use crate::parser::element::inline_content_parser;
use crate::parser::utils::with_depth;
use finemark_ast::{Element, Span, TextStyleElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub(crate) fn parse_text_style<'i, F>(
    parser_input: &mut ParserInput<'i>,
    delimiter: &'static str,
    guard: ParseGuard,
    make_element: F,
) -> Result<Element<'i>>
where
    F: FnOnce(TextStyleElement<'i>) -> Element<'i>,
{
    if parser_input.state.is_guard_active(guard) {
        return Err(winnow::error::ContextError::new());
    }

    let start = parser_input.current_token_start();
    let open_start = start;
    literal(delimiter).parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    parser_input.state.enter_guard(guard);
    let children = with_depth(parser_input, inline_content_parser);
    parser_input.state.exit_guard(guard);
    let children = children?;

    let close_start = parser_input.current_token_start();
    literal(delimiter).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(make_element(TextStyleElement {
        span: Span { start, end },
        open_span: Span {
            start: open_start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        children,
    }))
}
