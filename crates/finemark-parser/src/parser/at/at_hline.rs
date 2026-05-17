use crate::parser::ParserInput;
use crate::parser::at::utils::parse_at_head;
use finemark_ast::{Element, HLineElement, Span};
use winnow::Result;
use winnow::stream::Location as StreamLocation;

pub(crate) fn at_hline_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let head = parse_at_head(parser_input, "hline")?;
    let end = parser_input.previous_token_end();

    Ok(Element::HLine(HLineElement {
        span: Span {
            start: head.start,
            end,
        },
        parameters: head.parameters,
    }))
}
