use crate::parser::ParserInput;
use crate::parser::at::utils::{
    AfterClosePolicy, BodyWhitespacePolicy, parse_at_head, parse_optional_document_body,
};
use finemark_ast::{BlockQuoteElement, Element, Span};
use winnow::Result;

pub(crate) fn at_quote_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let head = parse_at_head(parser_input, "quote")?;
    let body = parse_optional_document_body(
        parser_input,
        BodyWhitespacePolicy::TrimAsciiWhitespace,
        AfterClosePolicy::ConsumeWhitespace,
    )?;

    Ok(Element::BlockQuote(BlockQuoteElement {
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
