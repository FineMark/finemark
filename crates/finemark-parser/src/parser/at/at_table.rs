use crate::parser::ParserInput;
use crate::parser::at::utils::{
    AfterClosePolicy, BodyWhitespacePolicy, ParsedAtBody, parse_at_head,
    parse_optional_document_body,
};
use crate::parser::utils::{with_body, with_depth};
use finemark_ast::{Element, Span, TableColumnElement, TableElement, TableRowElement};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub(crate) fn at_table_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let head = parse_at_head(parser_input, "table")?;
    let body = parse_table_body(parser_input)?;

    Ok(Element::Table(TableElement {
        span: Span {
            start: head.start,
            end: body.end,
        },
        body_open_span: body.open_span,
        body_close_span: body.close_span,
        parameters: head.parameters,
        children: body.children,
    }))
}

fn parse_table_body<'i>(parser_input: &mut ParserInput<'i>) -> Result<ParsedAtBody<'i>> {
    parse_structural_body(parser_input, at_row_parser)
}

fn at_row_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let head = parse_at_head(parser_input, "row")?;
    let body = parse_row_body(parser_input)?;

    Ok(Element::TableRow(TableRowElement {
        span: Span {
            start: head.start,
            end: body.end,
        },
        body_open_span: body.open_span,
        body_close_span: body.close_span,
        parameters: head.parameters,
        children: body.children,
    }))
}

fn parse_row_body<'i>(parser_input: &mut ParserInput<'i>) -> Result<ParsedAtBody<'i>> {
    parse_structural_body(parser_input, at_column_parser)
}

fn parse_structural_body<'i, F>(
    parser_input: &mut ParserInput<'i>,
    mut child_parser: F,
) -> Result<ParsedAtBody<'i>>
where
    F: FnMut(&mut ParserInput<'i>) -> Result<Element<'i>>,
{
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
        with_body(input, |input| {
            let mut children = Vec::new();
            loop {
                multispace0.parse_next(input)?;
                if input.input.starts_with('}') {
                    break;
                }
                children.push(child_parser(input)?);
            }
            Ok(children)
        })
    })?;

    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("}").parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();
    multispace0.parse_next(parser_input)?;

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

fn at_column_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let head = parse_at_head(parser_input, "column")?;
    let body = parse_optional_document_body(
        parser_input,
        BodyWhitespacePolicy::TrimAsciiWhitespace,
        AfterClosePolicy::ConsumeWhitespace,
    )?;

    Ok(Element::TableColumn(TableColumnElement {
        span: Span {
            start: head.start,
            end: body.end,
        },
        body_open_span: body.open_span,
        body_close_span: body.close_span,
        parameters: head.parameters,
        children: body.children,
    }))
}
