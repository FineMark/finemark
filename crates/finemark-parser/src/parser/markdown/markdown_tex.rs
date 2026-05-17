use crate::parser::ParserInput;
use finemark_ast::{Element, Span, TeXElement};
use memchr::{memchr, memmem};
use winnow::Result;
use winnow::combinator::alt;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::literal;

pub(crate) fn markdown_tex_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    alt((tex_block_parser, tex_inline_parser)).parse_next(parser_input)
}

fn tex_block_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    tex_parser(parser_input, "$$", true, true)
}

fn tex_inline_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    tex_parser(parser_input, "$", false, false)
}

fn tex_parser<'i>(
    parser_input: &mut ParserInput<'i>,
    delimiter: &'static str,
    is_block: bool,
    allow_newline: bool,
) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    literal(delimiter).parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let content_checkpoint = parser_input.checkpoint();
    let content_start = parser_input.current_token_start();
    consume_until_delimiter(parser_input, delimiter, allow_newline)?;
    let content_len = parser_input.current_token_start() - content_start;
    parser_input.reset(&content_checkpoint);

    let value = parser_input.input.next_slice(content_len);
    let close_start = parser_input.current_token_start();
    literal(delimiter).parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::TeX(TeXElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        is_block,
        value,
    }))
}

fn consume_until_delimiter(
    parser_input: &mut ParserInput<'_>,
    delimiter: &'static str,
    allow_newline: bool,
) -> Result<()> {
    let remaining = parser_input
        .input
        .peek_slice(parser_input.input.eof_offset());
    let delimiter_offset = memmem::find(remaining.as_bytes(), delimiter.as_bytes());
    let Some(content_len) = delimiter_offset else {
        return Err(winnow::error::ContextError::new());
    };

    if !allow_newline && memchr(b'\n', &remaining.as_bytes()[..content_len]).is_some() {
        return Err(winnow::error::ContextError::new());
    }

    parser_input.input.next_slice(content_len);
    Ok(())
}
