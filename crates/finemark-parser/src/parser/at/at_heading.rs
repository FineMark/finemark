use crate::parser::ParserInput;
use crate::parser::at::utils::{
    AfterClosePolicy, BodyWhitespacePolicy, parse_at_head, parse_optional_document_body,
};
use finemark_ast::{Element, HeadingElement, Span};
use winnow::Result;

pub(crate) fn at_h1_parser(parser_input: &mut ParserInput) -> Result<Element> {
    at_heading_parser(parser_input, "h1", 1)
}

pub(crate) fn at_h2_parser(parser_input: &mut ParserInput) -> Result<Element> {
    at_heading_parser(parser_input, "h2", 2)
}

pub(crate) fn at_h3_parser(parser_input: &mut ParserInput) -> Result<Element> {
    at_heading_parser(parser_input, "h3", 3)
}

pub(crate) fn at_h4_parser(parser_input: &mut ParserInput) -> Result<Element> {
    at_heading_parser(parser_input, "h4", 4)
}

pub(crate) fn at_h5_parser(parser_input: &mut ParserInput) -> Result<Element> {
    at_heading_parser(parser_input, "h5", 5)
}

pub(crate) fn at_h6_parser(parser_input: &mut ParserInput) -> Result<Element> {
    at_heading_parser(parser_input, "h6", 6)
}

fn at_heading_parser(
    parser_input: &mut ParserInput,
    keyword: &'static str,
    level: u8,
) -> Result<Element> {
    let head = parse_at_head(parser_input, keyword)?;
    let body = parse_optional_document_body(
        parser_input,
        BodyWhitespacePolicy::TrimAsciiWhitespace,
        AfterClosePolicy::ConsumeWhitespace,
    )?;

    Ok(Element::Heading(HeadingElement {
        span: Span {
            start: head.start,
            end: body.end,
        },
        marker_span: head.open_span,
        level,
        section_index: parser_input.state.next_section_index(),
        parameters: head.parameters,
        children: body.children,
    }))
}
