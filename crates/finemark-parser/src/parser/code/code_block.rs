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

    let remaining = parser_input
        .input
        .peek_slice(parser_input.input.eof_offset());
    let Some((content_len, close_len)) = find_closing_fence(remaining, fence_len) else {
        return Err(winnow::error::ContextError::new());
    };

    let value = parser_input.input.next_slice(content_len);
    let close_start = parser_input.current_token_start();
    space0.parse_next(parser_input)?;
    let close = take_while(fence_len.., '`').parse_next(parser_input)?;
    debug_assert!(close.len() >= close_len);
    space0.parse_next(parser_input)?;
    opt(literal("\n")).parse_next(parser_input)?;
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

fn find_closing_fence(input: &str, fence_len: usize) -> Option<(usize, usize)> {
    let bytes = input.as_bytes();
    let mut line_start = 0usize;

    while line_start <= bytes.len() {
        let line_end = memchr(b'\n', &bytes[line_start..])
            .map(|idx| line_start + idx)
            .unwrap_or(bytes.len());
        let line = &bytes[line_start..line_end];
        let leading_ws = count_fence_space_prefix(line);
        let trimmed_start = &line[leading_ws..];
        let backticks = count_backticks(trimmed_start);
        if backticks >= fence_len {
            let rest = &trimmed_start[backticks..];
            if rest.iter().all(|&b| is_fence_space(b)) {
                return Some((line_start + leading_ws, backticks));
            }
        }
        if line_end == bytes.len() {
            break;
        }
        line_start = line_end + 1;
    }

    None
}

fn count_fence_space_prefix(bytes: &[u8]) -> usize {
    bytes.iter().take_while(|&&b| is_fence_space(b)).count()
}

fn count_backticks(bytes: &[u8]) -> usize {
    bytes.iter().take_while(|&&b| b == b'`').count()
}

fn is_fence_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t')
}
