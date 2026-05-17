use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::{with_body, with_depth};
use finemark_ast::{Element, Parameters, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{opt, preceded, repeat};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub(crate) struct ParsedAtHead<'i> {
    pub start: usize,
    pub open_span: Span,
    pub parameters: Parameters<'i>,
}

pub(crate) struct ParsedAtBody<'i> {
    pub children: Vec<Element<'i>>,
    pub open_span: Option<Span>,
    pub close_span: Option<Span>,
    pub end: usize,
}

pub(crate) use crate::parser::utils::{AfterClosePolicy, BodyWhitespacePolicy};

pub(crate) fn parse_at_head<'i>(
    parser_input: &mut ParserInput<'i>,
    keyword: &'static str,
) -> Result<ParsedAtHead<'i>> {
    let start = parser_input.current_token_start();
    literal("@").parse_next(parser_input)?;
    literal(keyword).parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    // Allow optional whitespace between the keyword and the opening `(`.
    let parameters = opt(preceded(multispace0, parameter_core_parser))
        .parse_next(parser_input)?
        .unwrap_or_default();

    Ok(ParsedAtHead {
        start,
        open_span: Span {
            start,
            end: open_end,
        },
        parameters,
    })
}

pub(crate) fn parse_optional_document_body<'i>(
    parser_input: &mut ParserInput<'i>,
    body_policy: BodyWhitespacePolicy,
    after_close_policy: AfterClosePolicy,
) -> Result<ParsedAtBody<'i>> {
    // Allow optional whitespace between the parameter list (or keyword) and `{`.
    multispace0.parse_next(parser_input)?;
    let Some(open_span) = opt(parse_body_open).parse_next(parser_input)? else {
        let end = parser_input.previous_token_end();
        return Ok(ParsedAtBody {
            children: Vec::new(),
            open_span: None,
            close_span: None,
            end,
        });
    };

    let children = with_depth(parser_input, |input| {
        with_body(input, |input| repeat(0.., element_parser).parse_next(input))
    })?;
    let children = apply_body_policy_to_elements(children, body_policy);
    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("}").parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    if matches!(after_close_policy, AfterClosePolicy::ConsumeWhitespace) {
        multispace0.parse_next(parser_input)?;
    }

    Ok(ParsedAtBody {
        children,
        open_span: Some(open_span),
        close_span: Some(Span {
            start: close_start,
            end: close_end,
        }),
        end: close_end,
    })
}

fn parse_body_open(parser_input: &mut ParserInput<'_>) -> Result<Span> {
    let start = parser_input.current_token_start();
    literal("{").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();
    Ok(Span { start, end })
}

fn apply_body_policy_to_elements<'i>(
    mut children: Vec<Element<'i>>,
    body_policy: BodyWhitespacePolicy,
) -> Vec<Element<'i>> {
    if matches!(body_policy, BodyWhitespacePolicy::Preserve) {
        return children;
    }

    while children.first().is_some_and(is_ascii_whitespace_element) {
        children.remove(0);
    }
    while children.last().is_some_and(is_ascii_whitespace_element) {
        children.pop();
    }
    children
}

fn is_ascii_whitespace_element(element: &Element<'_>) -> bool {
    match element {
        Element::Text(text) => text.value.chars().all(|c| c.is_ascii_whitespace()),
        Element::SoftBreak(_) | Element::ParagraphBreak(_) => true,
        _ => false,
    }
}
