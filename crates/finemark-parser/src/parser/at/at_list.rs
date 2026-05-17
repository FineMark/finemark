use crate::parser::ParserInput;
use crate::parser::at::utils::{
    AfterClosePolicy, BodyWhitespacePolicy, parse_at_head, parse_optional_document_body,
};
use crate::parser::utils::{with_body, with_depth};
use finemark_ast::{Element, ListElement, ListItem, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

struct ParsedListBody<'i> {
    items: Vec<ListItem<'i>>,
    end: usize,
}

pub(crate) fn at_list_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
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

fn parse_list_body<'i>(parser_input: &mut ParserInput<'i>) -> Result<ParsedListBody<'i>> {
    multispace0.parse_next(parser_input)?;
    let Some(_) = opt(parse_body_open).parse_next(parser_input)? else {
        let end = parser_input.previous_token_end();
        return Ok(ParsedListBody {
            items: Vec::new(),
            end,
        });
    };

    let items = with_depth(parser_input, |input| {
        with_body(input, |input| {
            let mut items = Vec::new();
            loop {
                multispace0.parse_next(input)?;
                if input.input.starts_with('}') {
                    break;
                }
                items.push(at_item_parser(input)?);
            }
            Ok(items)
        })
    })?;

    multispace0.parse_next(parser_input)?;
    literal("}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();
    multispace0.parse_next(parser_input)?;

    Ok(ParsedListBody { items, end })
}

fn parse_body_open(parser_input: &mut ParserInput<'_>) -> Result<Span> {
    let start = parser_input.current_token_start();
    literal("{").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();
    Ok(Span { start, end })
}

fn at_item_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<ListItem<'i>> {
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
