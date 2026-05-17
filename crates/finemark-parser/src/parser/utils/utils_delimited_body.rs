use crate::parser::ParserInput;
use crate::parser::utils::parse_raw_until_balanced_single_brace;
use finemark_ast::Span;
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BodyWhitespacePolicy {
    Preserve,
    TrimAsciiWhitespace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AfterClosePolicy {
    Preserve,
    ConsumeWhitespace,
}

pub(crate) struct ParsedDelimitedBody<'i> {
    pub content: &'i str,
    pub content_start: usize,
    pub open_span: Span,
    pub close_span: Span,
    pub end: usize,
}

pub(crate) fn parse_brace_body<'i>(
    parser_input: &mut ParserInput<'i>,
    body_policy: BodyWhitespacePolicy,
    after_close_policy: AfterClosePolicy,
) -> Result<ParsedDelimitedBody<'i>> {
    let open_start = parser_input.current_token_start();
    literal("{").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();
    let content_start = parser_input.current_token_start();
    let raw = parse_raw_until_balanced_single_brace(parser_input)?;
    let (content, leading_trimmed) = apply_body_policy(raw.value, body_policy);

    // Block-style bodies commonly use `@foo{\n  ...\n}` for readability.
    // Trimming here mirrors SevenMark's delimited-body policy and keeps
    // structural parsers from seeing formatting whitespace as content.
    if matches!(after_close_policy, AfterClosePolicy::ConsumeWhitespace) {
        multispace0.parse_next(parser_input)?;
    }

    Ok(ParsedDelimitedBody {
        content,
        content_start: content_start + leading_trimmed,
        open_span: Span {
            start: open_start,
            end: open_end,
        },
        close_span: Span {
            start: raw.close_start,
            end: raw.close_end,
        },
        end: raw.close_end,
    })
}

pub(crate) fn parse_optional_brace_body<'i>(
    parser_input: &mut ParserInput<'i>,
    body_policy: BodyWhitespacePolicy,
    after_close_policy: AfterClosePolicy,
) -> Result<Option<ParsedDelimitedBody<'i>>> {
    if parser_input.input.peek_token() != Some('{') {
        return Ok(None);
    }

    parse_brace_body(parser_input, body_policy, after_close_policy).map(Some)
}

fn apply_body_policy(content: &str, policy: BodyWhitespacePolicy) -> (&str, usize) {
    match policy {
        BodyWhitespacePolicy::Preserve => (content, 0),
        BodyWhitespacePolicy::TrimAsciiWhitespace => {
            let trimmed_start = content.trim_start_matches(|c: char| c.is_ascii_whitespace());
            let leading_trimmed = content.len() - trimmed_start.len();
            (
                trimmed_start.trim_end_matches(|c: char| c.is_ascii_whitespace()),
                leading_trimmed,
            )
        }
    }
}
