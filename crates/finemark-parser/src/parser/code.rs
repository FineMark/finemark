use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use finemark_ast::{CodeBlockElement, Element, Parameters, Span};
use winnow::Result;
use winnow::ascii::space0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::{literal, take_while};

pub(crate) fn code_block_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    let fence = take_while(3.., '`').parse_next(parser_input)?;
    let fence_len = fence.len();
    let params = opt(parameter_core_parser).parse_next(parser_input)?.unwrap_or_default();
    space0.parse_next(parser_input)?;
    literal("\n").parse_next(parser_input)?;
    let content_start = parser_input.current_token_start();

    let remaining = parser_input.input.peek_slice(parser_input.input.eof_offset());
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

pub(crate) fn inline_code_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    use finemark_ast::InlineCodeElement;

    let start = parser_input.current_token_start();
    literal("`").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();
    let remaining = parser_input.input.peek_slice(parser_input.input.eof_offset());
    let Some(close_offset) = remaining.find('`') else {
        return Err(winnow::error::ContextError::new());
    };
    if remaining[..close_offset].contains('\n') {
        return Err(winnow::error::ContextError::new());
    }
    let close_start = parser_input.current_token_start() + close_offset;
    let value = parser_input.input.next_slice(close_offset);
    literal("`").parse_next(parser_input)?;
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

fn find_closing_fence(input: &str, fence_len: usize) -> Option<(usize, usize)> {
    let bytes = input.as_bytes();
    let mut line_start = 0usize;

    while line_start <= bytes.len() {
        let line_end = bytes[line_start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|idx| line_start + idx)
            .unwrap_or(bytes.len());
        let line = &input[line_start..line_end];
        let trimmed_start = line.trim_start_matches([' ', '\t']);
        let leading_ws = line.len() - trimmed_start.len();
        let backticks = trimmed_start
            .as_bytes()
            .iter()
            .take_while(|&&b| b == b'`')
            .count();
        if backticks >= fence_len {
            let rest = &trimmed_start[backticks..];
            if rest.trim_matches([' ', '\t']).is_empty() {
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
