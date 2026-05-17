use crate::parser::ParserInput;
use finemark_ast::{Element, InlineCodeElement, Span};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::{any, literal, take_while};

pub(crate) fn markdown_inline_code_parser<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    let opening = take_while(1.., '`').parse_next(parser_input)?;
    let fence_len = opening.len();
    let open_end = parser_input.previous_token_end();

    let content_checkpoint = parser_input.checkpoint();
    let content_start = parser_input.current_token_start();
    consume_until_closing_run(parser_input, fence_len)?;
    let content_len = parser_input.current_token_start() - content_start;
    parser_input.reset(&content_checkpoint);

    let value = parser_input.input.next_slice(content_len);
    let close_start = parser_input.current_token_start();
    literal("`".repeat(fence_len).as_str()).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::InlineCode(InlineCodeElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        value,
    }))
}

fn consume_until_closing_run(parser_input: &mut ParserInput<'_>, fence_len: usize) -> Result<()> {
    let closing = "`".repeat(fence_len);

    loop {
        if parser_input.input.starts_with(closing.as_str()) {
            return Ok(());
        }
        if parser_input.input.starts_with("\n") || parser_input.input.is_empty() {
            return Err(winnow::error::ContextError::new());
        }
        any.parse_next(parser_input)?;
    }
}
