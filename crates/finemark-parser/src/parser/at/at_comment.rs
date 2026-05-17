use crate::parser::ParserInput;
use crate::parser::at::utils::parse_at_head;
use finemark_ast::{CommentElement, Element, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::stream::Stream;
use winnow::token::literal;

pub(crate) fn at_comment_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let head = parse_at_head(parser_input, "comment")?;
    multispace0.parse_next(parser_input)?;
    let Some(open_span) = opt(parse_body_open).parse_next(parser_input)? else {
        let end = parser_input.previous_token_end();
        return Ok(Element::Comment(CommentElement {
            span: Span {
                start: head.start,
                end,
            },
            body_open_span: None,
            body_close_span: None,
            value: "",
        }));
    };

    let remaining = parser_input.input.peek_slice(parser_input.input.eof_offset());
    let Some(close_idx) = find_unescaped_close_brace(remaining) else {
        return Err(winnow::error::ContextError::new());
    };
    let value = parser_input.input.next_slice(close_idx);
    let close_start = parser_input.current_token_start();
    literal("}").parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    // Comments keep raw brace content because renderers usually discard them
    // instead of rendering child elements.
    Ok(Element::Comment(CommentElement {
        span: Span {
            start: head.start,
            end: close_end,
        },
        body_open_span: Some(open_span),
        body_close_span: Some(Span {
            start: close_start,
            end: close_end,
        }),
        value,
    }))
}

fn parse_body_open(parser_input: &mut ParserInput<'_>) -> Result<Span> {
    let start = parser_input.current_token_start();
    literal("{").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();
    Ok(Span { start, end })
}

fn find_unescaped_close_brace(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let rel = memchr::memchr(b'}', &bytes[i..])?;
        i += rel;
        if !has_odd_preceding_backslashes(bytes, i) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn has_odd_preceding_backslashes(bytes: &[u8], offset: usize) -> bool {
    let mut count = 0usize;
    let mut i = offset;
    while i > 0 && bytes[i - 1] == b'\\' {
        count += 1;
        i -= 1;
    }
    count % 2 == 1
}
