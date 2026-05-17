use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use finemark_ast::{CodeBlockElement, Element, Span};
use memchr::memchr;
use winnow::Result;
use winnow::ascii::space0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::{literal, take_while};

pub(crate) fn code_block_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(start) {
        return Err(winnow::error::ContextError::new());
    }

    let fence = take_while(3.., '`').parse_next(parser_input)?;
    let fence_len = fence.len();
    let params = opt(parameter_core_parser)
        .parse_next(parser_input)?
        .unwrap_or_default();
    space0.parse_next(parser_input)?;
    literal("\n").parse_next(parser_input)?;
    let content_start = parser_input.current_token_start();

    let content_checkpoint = parser_input.checkpoint();
    consume_until_closing_fence(parser_input, fence_len)?;
    let content_len = parser_input.current_token_start() - content_start;
    parser_input.reset(&content_checkpoint);

    let value = parser_input.input.next_slice(content_len);
    let close_start = parser_input.current_token_start();
    parse_closing_fence(parser_input, fence_len)?;
    let end = parser_input.previous_token_end();

    Ok(Element::CodeBlock(CodeBlockElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: content_start,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        parameters: params,
        value,
    }))
}

fn consume_until_closing_fence(parser_input: &mut ParserInput<'_>, fence_len: usize) -> Result<()> {
    loop {
        let checkpoint = parser_input.checkpoint();
        if parse_closing_fence(parser_input, fence_len).is_ok() {
            parser_input.reset(&checkpoint);
            return Ok(());
        }
        parser_input.reset(&checkpoint);

        if parser_input.input.is_empty() {
            return Err(winnow::error::ContextError::new());
        }

        consume_line_chunk(parser_input);
    }
}

fn consume_line_chunk(parser_input: &mut ParserInput<'_>) {
    let remaining = parser_input
        .input
        .peek_slice(parser_input.input.eof_offset());
    let len = memchr(b'\n', remaining.as_bytes()).map_or(remaining.len(), |offset| offset + 1);
    parser_input.input.next_slice(len);
}

fn parse_closing_fence(parser_input: &mut ParserInput<'_>, fence_len: usize) -> Result<()> {
    space0.parse_next(parser_input)?;
    take_while(fence_len.., '`').parse_next(parser_input)?;
    space0.parse_next(parser_input)?;
    let has_newline = opt(literal("\n")).parse_next(parser_input)?.is_some();
    if !has_newline && !parser_input.input.is_empty() {
        return Err(winnow::error::ContextError::new());
    }
    Ok(())
}
