use crate::parser::at::utils::{
    AfterClosePolicy, BodyWhitespacePolicy, parse_at_head, parse_optional_document_body,
};
use crate::parser::utils::parse_optional_brace_body;
use crate::parser::{ParserInput};
use finemark_ast::{Element, ErrorElement, ListElement, ListItem, Span};
use winnow::Result;
use winnow::combinator::repeat;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};

struct ParsedListBody {
    items: Vec<ListItem>,
    end: usize,
}

pub(crate) fn at_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let head = parse_at_head(parser_input, "list")?;
    let body = parse_list_body(parser_input)?;

    Ok(Element::List(ListElement {
        span: Span {
            start: head.start,
            end: body.end,
        },
        parameters: head.parameters,
        items: body.items,
    }))
}

fn parse_list_body(parser_input: &mut ParserInput) -> Result<ParsedListBody> {
    let body = parse_optional_brace_body(
        parser_input,
        BodyWhitespacePolicy::TrimAsciiWhitespace,
        AfterClosePolicy::ConsumeWhitespace,
    )?;
    let Some(body) = body else {
        let end = parser_input.previous_token_end();
        return Ok(ParsedListBody {
            items: Vec::new(),
            end,
        });
    };

    let mut child_input = ParserInput {
        input: parser_input.input.child_source_for_content(body.content),
        state: parser_input.state.clone(),
    };

    let mut items: Vec<ListItem> = repeat(0.., at_item_parser).parse_next(&mut child_input)?;
    if !child_input.input.is_empty() {
        let start = child_input.current_token_start();
        let value = child_input
            .input
            .peek_slice(child_input.input.eof_offset())
            .to_string();
        child_input.input.finish();
        let end = child_input.previous_token_end();
        items.push(ListItem {
            span: Span { start, end },
            children: vec![Element::Error(ErrorElement {
                span: Span { start, end },
                value,
            })],
        });
    }
    parser_input.state = child_input.state;

    Ok(ParsedListBody {
        items,
        end: body.end,
    })
}

fn at_item_parser(parser_input: &mut ParserInput) -> Result<ListItem> {
    let head = parse_at_head(parser_input, "item")?;
    let body = parse_optional_document_body(
        parser_input,
        BodyWhitespacePolicy::TrimAsciiWhitespace,
        AfterClosePolicy::ConsumeWhitespace,
    )?;

    Ok(ListItem {
        span: Span {
            start: head.start,
            end: body.end,
        },
        children: body.children,
    })
}
