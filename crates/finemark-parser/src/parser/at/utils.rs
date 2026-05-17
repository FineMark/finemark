use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::{parse_nested_document_at, parse_optional_brace_body};
use finemark_ast::{Element, Parameters, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{opt, preceded};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub(crate) struct ParsedAtHead {
    pub start: usize,
    pub open_span: Span,
    pub parameters: Parameters,
}

pub(crate) struct ParsedAtBody {
    pub children: Vec<Element>,
    pub open_span: Option<Span>,
    pub close_span: Option<Span>,
    pub end: usize,
}

pub(crate) use crate::parser::utils::{AfterClosePolicy, BodyWhitespacePolicy};

pub(crate) fn parse_at_head(
    parser_input: &mut ParserInput,
    keyword: &'static str,
) -> Result<ParsedAtHead> {
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

pub(crate) fn parse_optional_document_body(
    parser_input: &mut ParserInput,
    body_policy: BodyWhitespacePolicy,
    after_close_policy: AfterClosePolicy,
) -> Result<ParsedAtBody> {
    // Allow optional whitespace between the parameter list (or keyword) and `{`.
    multispace0.parse_next(parser_input)?;
    let body = parse_optional_brace_body(parser_input, body_policy, after_close_policy)?;
    let Some(body) = body else {
        let end = parser_input.previous_token_end();
        return Ok(ParsedAtBody {
            children: Vec::new(),
            open_span: None,
            close_span: None,
            end,
        });
    };

    let children = parse_nested_document_at(parser_input, body.content)?;

    Ok(ParsedAtBody {
        children,
        open_span: Some(body.open_span),
        close_span: Some(body.close_span),
        end: body.end,
    })
}
