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

/// Stack-free balanced-delimiter scanner.
///
/// `block_depth` counts extra nested block-width delimiters beyond the initial one.
/// `single_depth` counts open single-character delimiters. Once inside a single context,
/// new block-width sequences are also treated as singles — so the logical delimiter
/// state is always `Block×(block_depth+1)` then `Single×single_depth`, which is why
/// two counters are equivalent to a `Vec` and avoid any heap allocation.
pub(crate) fn find_balanced_block_close(
    bytes: &[u8],
    open_byte: u8,
    close_byte: u8,
    block_width: usize,
) -> Option<usize> {
    let mut i = 0usize;
    let mut block_depth = 0usize;
    let mut single_depth = 0usize;

    while i < bytes.len() {
        let Some(rel_idx) = memchr2(open_byte, close_byte, &bytes[i..]) else {
            break;
        };
        i += rel_idx;

        if has_odd_preceding_backslashes(bytes, i) {
            i += 1;
            continue;
        }

        let b = bytes[i];
        if b == open_byte {
            if single_depth == 0 && has_repeated_byte(bytes, i, open_byte, block_width) {
                block_depth += 1;
                i += block_width;
            } else {
                single_depth += 1;
                i += 1;
            }
            continue;
        }

        if b == close_byte {
            if single_depth > 0 {
                single_depth -= 1;
                i += 1;
            } else if has_repeated_byte(bytes, i, close_byte, block_width) {
                if block_depth == 0 {
                    return Some(i);
                }
                block_depth -= 1;
                i += block_width;
            } else {
                i += 1;
            }
        }
    }

    None
}

pub(crate) fn has_repeated_byte(bytes: &[u8], offset: usize, byte: u8, width: usize) -> bool {
    bytes
        .get(offset..offset + width)
        .is_some_and(|candidate| candidate.iter().all(|&b| b == byte))
}

/// Parse raw block body until matching triple-brace depth is closed.
///
/// - Initial depth is 1 (the opening `{{{` already consumed by caller).
/// - Block delimiters (`{{{` / `}}}`) are nested while not inside a single `{ ... }`.
/// - Single braces are balanced separately, so `{}` may be adjacent to the closing `}}}`.
/// - Delimiters escaped by an odd number of preceding backslashes are content.
pub fn parse_raw_until_balanced_triple_brace<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<RawBlockParseResult<'i>> {
    let remaining: &'i str = parser_input.peek_slice(parser_input.eof_offset());
    let bytes = remaining.as_bytes();

    let Some(close_idx) = find_balanced_block_close(bytes, b'{', b'}', 3) else {
        return Err(winnow::error::ContextError::new());
    };

    // `close_idx` comes from scanning ASCII delimiters (`{{{` / `}}}`) in UTF-8 bytes,
    // so it is a valid byte boundary for slicing `&str`.
    let value: &'i str = parser_input.next_slice(close_idx);

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    Ok(RawBlockParseResult {
        value,
        close_start,
        close_end,
    })
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

pub(crate) fn skip_single_bracket(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start + 1;
    while i < bytes.len() {
        let rel = memchr::memchr(b']', &bytes[i..])?;
        i += rel;
        if !has_odd_preceding_backslashes(bytes, i) {
            return Some(i + 1);
        }
        i += 1;
    }
    None
}

/// Parse raw block body until matching double-bracket depth is closed.
///
/// - Initial depth is 1 (the opening `[[` already consumed by caller).
/// - Block delimiters (`[[` / `]]`) are nested while not inside a single `[ ... ]`.
/// - Single brackets are balanced separately, so `[macro]` adjacent to the closing `]]`
///   is not consumed prematurely (e.g. `[now]]]` → content `[now]`, close `]]`).
/// - Delimiters escaped by an odd number of preceding backslashes are content.
/// - Note: an unclosed single `[` (malformed input) will cause this to return an error.
pub fn parse_raw_until_balanced_double_bracket<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<RawBlockParseResult<'i>> {
    let remaining: &'i str = parser_input.peek_slice(parser_input.eof_offset());
    let bytes = remaining.as_bytes();

    let Some(close_idx) = find_balanced_block_close(bytes, b'[', b']', 2) else {
        return Err(winnow::error::ContextError::new());
    };

    let value: &'i str = parser_input.next_slice(close_idx);

    let close_start = parser_input.current_token_start();
    literal("]]").parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    Ok(RawBlockParseResult {
        value,
        close_start,
        close_end,
    })
}

/// Parse raw single-bracket body until the next unmatched `]`.
///
/// - Initial depth is 1 (the opening `[` already consumed by caller).
/// - Nested bracketed constructs such as `[br]` and `[[link]]` are skipped as content.
/// - Delimiters escaped by an odd number of preceding backslashes are content.
pub fn parse_raw_until_balanced_single_bracket<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<RawBlockParseResult<'i>> {
    let remaining: &'i str = parser_input.peek_slice(parser_input.eof_offset());
    let bytes = remaining.as_bytes();

    let Some(close_idx) = find_balanced_single_bracket_close(bytes) else {
        return Err(winnow::error::ContextError::new());
    };

    let value: &'i str = parser_input.next_slice(close_idx);

    let close_start = parser_input.current_token_start();
    literal(']').parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    Ok(RawBlockParseResult {
        value,
        close_start,
        close_end,
    })
}

pub(crate) fn find_balanced_single_bracket_close(bytes: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    let mut depth = 0usize;

    while i < bytes.len() {
        let Some(rel_idx) = memchr2(b'[', b']', &bytes[i..]) else {
            break;
        };
        i += rel_idx;

        if has_odd_preceding_backslashes(bytes, i) {
            i += 1;
            continue;
        }

        match bytes[i] {
            b'[' => {
                depth += 1;
                i += 1;
            }
            b']' if depth == 0 => return Some(i),
            b']' => {
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

    fn assert_triple_brace_raw_value(value: &str) {
        let input = [value, "}}}"].concat();
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(&input),
            state: context,
        };

        let result = parse_raw_until_balanced_triple_brace(&mut parser_input)
            .expect("raw block parse should succeed");

        assert_eq!(result.value, value);
        assert_eq!(result.close_start, value.len());
        assert_eq!(result.close_end, input.len());
        assert!(parser_input.input.is_empty());
    }

    fn assert_double_bracket_raw_value(value: &str) {
        let input = [value, "]]"].concat();
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(&input),
            state: context,
        };

        let result = parse_raw_until_balanced_double_bracket(&mut parser_input)
            .expect("raw block parse should succeed");

        assert_eq!(result.value, value);
        assert_eq!(result.close_start, value.len());
        assert_eq!(result.close_end, input.len());
        assert!(parser_input.input.is_empty());
    }

    #[test]
    fn parse_balanced_triple_brace_with_utf8_content() {
        assert_triple_brace_raw_value("한글🙂{{{중첩}}}끝");
    }

    #[test]
    fn parse_escaped_closing_triple_brace_as_content() {
        assert_triple_brace_raw_value("literal \\}}} still content");
    }

    #[test]
    fn parse_individually_escaped_opening_braces_as_content() {
        assert_triple_brace_raw_value(r"\{\{\{");
    }

    #[test]
    fn parse_single_braces_adjacent_to_closing_triple_brace() {
        assert_triple_brace_raw_value("{}");
    }

    #[test]
    fn parse_nested_single_braces_adjacent_to_closing_triple_brace() {
        assert_triple_brace_raw_value("{{}}");
    }

    #[test]
    fn parse_escaped_single_braces_as_content() {
        assert_triple_brace_raw_value("literal \\{ \\} still content");
    }

    #[test]
    fn parse_escaped_single_braces_leave_following_unescaped_brace_balanced() {
        assert_triple_brace_raw_value(r"\{\{{}");
    }

    #[test]
    fn parse_even_backslashes_do_not_escape_single_brace() {
        assert_triple_brace_raw_value(r"\\{}");
    }

    #[test]
    fn parse_single_brackets_adjacent_to_closing_double_bracket() {
        assert_double_bracket_raw_value("[]");
    }

    #[test]
    fn parse_nested_double_bracket_before_closing_double_bracket() {
        assert_double_bracket_raw_value("[[nested]]tail");
    }

    #[test]
    fn parse_even_backslashes_do_not_escape_single_bracket() {
        assert_double_bracket_raw_value(r"\\[]");
    }

    #[test]
    fn parse_escaped_double_brackets_as_content() {
        assert_double_bracket_raw_value(r"literal \[\[x\]\] still content");
    }

    #[test]
    fn parse_escaped_single_brackets_as_content() {
        assert_double_bracket_raw_value(r"literal \[ \] still content");
    }

    #[test]
    fn macro_bracket_adjacent_to_closing_double_bracket() {
        // [now]]] → content "[now]", close "]]"
        // Previously the scanner would see the ] from [now] + first ] of ]] as ]] and stop early.
        assert_double_bracket_raw_value("[now]");
    }

    #[test]
    fn macro_bracket_in_text_adjacent_to_closing_double_bracket() {
        assert_double_bracket_raw_value("Current time: [now]");
    }

    #[test]
    fn multiple_macros_adjacent_to_closing_double_bracket() {
        assert_double_bracket_raw_value("[foo] and [bar]");
    }

    #[test]
    fn parse_single_bracket_body_with_nested_macro_and_link() {
        let input = "note [br] [[Target]] end] tail";
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(input),
            state: context,
        };

        let result = parse_raw_until_balanced_single_bracket(&mut parser_input)
            .expect("single bracket raw parse should succeed");

        assert_eq!(result.value, "note [br] [[Target]] end");
        assert_eq!(result.close_start, "note [br] [[Target]] end".len());
        assert_eq!(result.close_end, "note [br] [[Target]] end]".len());
    }

    #[test]
    fn parse_single_bracket_body_with_escaped_close() {
        let input = r"note \] still content] tail";
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(input),
            state: context,
        };

        let result = parse_raw_until_balanced_single_bracket(&mut parser_input)
            .expect("single bracket raw parse should succeed");

        assert_eq!(result.value, r"note \] still content");
    }
}