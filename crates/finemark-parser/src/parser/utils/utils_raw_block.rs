use crate::parser::ParserInput;
use memchr::memchr2;
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::literal;

pub struct RawBlockParseResult<'i> {
    pub value: &'i str,
    pub close_start: usize,
    pub close_end: usize,
}

pub(crate) fn has_odd_preceding_backslashes(bytes: &[u8], offset: usize) -> bool {
    let mut count = 0usize;
    let mut i = offset;
    while i > 0 && bytes[i - 1] == b'\\' {
        count += 1;
        i -= 1;
    }
    count % 2 == 1
}

/// Parse raw body until the matching single-brace close for an already consumed `{`.
///
/// The same balanced scanner used by block delimiters handles nested `{ ... }`
/// and escaped delimiters, so the caller does not need to know which element
/// kinds may appear inside the body before reparsing it as a child document.
pub fn parse_raw_until_balanced_single_brace<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<RawBlockParseResult<'i>> {
    let remaining: &'i str = parser_input.peek_slice(parser_input.eof_offset());
    let bytes = remaining.as_bytes();

    let Some(close_idx) = find_balanced_single_brace_close(bytes) else {
        return Err(winnow::error::ContextError::new());
    };

    let value: &'i str = parser_input.next_slice(close_idx);

    let close_start = parser_input.current_token_start();
    literal('}').parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    Ok(RawBlockParseResult {
        value,
        close_start,
        close_end,
    })
}

pub(crate) fn find_balanced_single_brace_close(bytes: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    let mut depth = 0usize;

    while i < bytes.len() {
        let Some(rel_idx) = memchr2(b'{', b'}', &bytes[i..]) else {
            break;
        };
        i += rel_idx;

        if has_odd_preceding_backslashes(bytes, i) {
            i += 1;
            continue;
        }

        match bytes[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' if depth == 0 => return Some(i),
            b'}' => {
                depth -= 1;
                i += 1;
            }
            _ => unreachable!(),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ParseContext;
    use crate::parser::InputSource;

    fn assert_single_brace_raw_value(value: &str) {
        let input = [value, "}"].concat();
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(&input),
            state: context,
        };

        let result = parse_raw_until_balanced_single_brace(&mut parser_input)
            .expect("raw block parse should succeed");

        assert_eq!(result.value, value);
        assert_eq!(result.close_start, value.len());
        assert_eq!(result.close_end, input.len());
        assert!(parser_input.input.is_empty());
    }

    #[test]
    fn parse_balanced_single_brace_with_utf8_content() {
        assert_single_brace_raw_value("한글🙂{중첩}끝");
    }

    #[test]
    fn parse_escaped_single_braces_as_content() {
        assert_single_brace_raw_value("literal \\{ \\} still content");
    }

    #[test]
    fn parse_even_backslashes_do_not_escape_single_brace() {
        assert_single_brace_raw_value(r"\\{}");
    }
}
