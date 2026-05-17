use crate::parser::ParserInput;
use crate::parser::at::utils::{
    AfterClosePolicy, BodyWhitespacePolicy, ParsedAtBody, parse_at_head,
    parse_optional_document_body,
};
use crate::parser::utils::parse_optional_brace_body;
use crate::parser::{InputSource, SourceSegment};
use finemark_ast::{
    Element, ErrorElement, Span, TableColumnElement, TableElement, TableRowElement,
};
use winnow::Result;
use winnow::combinator::repeat;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};

pub(crate) fn at_table_parser(parser_input: &mut ParserInput) -> Result<Element> {
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

fn parse_table_body<'i>(parser_input: &mut ParserInput<'i>) -> Result<ParsedAtBody> {
    parse_structural_body(parser_input, at_row_parser)
}

fn at_row_parser(parser_input: &mut ParserInput) -> Result<Element> {
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

fn parse_row_body<'i>(parser_input: &mut ParserInput<'i>) -> Result<ParsedAtBody> {
    parse_structural_body(parser_input, at_column_parser)
}

fn parse_structural_body<'i, F>(
    parser_input: &mut ParserInput<'i>,
    mut child_parser: F,
) -> Result<ParsedAtBody>
where
    F: FnMut(&mut ParserInput<'i>) -> Result<Element>,
{
    let body = parse_optional_brace_body(
        parser_input,
        BodyWhitespacePolicy::TrimAsciiWhitespace,
        AfterClosePolicy::ConsumeWhitespace,
    )?;
    let Some(body) = body else {
        let end = parser_input.previous_token_end();
        return Ok(ParsedAtBody {
            children: Vec::new(),
            open_span: None,
            close_span: None,
            end,
        });
    };

    let content = body.content;
    let content_start = body.content_start;
    let mut child_input = ParserInput {
        input: InputSource::new_segmented(
            content,
            if content.is_empty() {
                Vec::new()
            } else {
                vec![SourceSegment {
                    logical_start: 0,
                    original_start: content_start,
                    len: content.len(),
                }]
            },
            content_start,
        ),
        state: parser_input.state.clone(),
    };

    // Structural bodies first isolate the balanced brace content, then let the
    // owning grammar decide which child elements are valid inside that slice.
    let children = repeat(0.., &mut child_parser).parse_next(&mut child_input)?;
    let mut children: Vec<Element> = children;
    if !child_input.input.is_empty() {
        let start = child_input.current_token_start();
        let value = child_input
            .input
            .peek_slice(child_input.input.eof_offset())
            .to_string();
        child_input.input.finish();
        let end = child_input.previous_token_end();
        children.push(Element::Error(ErrorElement {
            span: Span { start, end },
            value,
        }));
    }
    parser_input.state = child_input.state;

    Ok(ParsedAtBody {
        children,
        open_span: Some(body.open_span),
        close_span: Some(body.close_span),
        end: body.end,
    })
}

fn at_column_parser(parser_input: &mut ParserInput) -> Result<Element> {
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
