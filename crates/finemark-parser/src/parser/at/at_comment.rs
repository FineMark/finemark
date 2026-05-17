use crate::parser::ParserInput;
use crate::parser::at::utils::{AfterClosePolicy, BodyWhitespacePolicy, parse_at_head};
use crate::parser::utils::parse_optional_brace_body;
use finemark_ast::{CommentElement, Element, Span};
use winnow::Result;
use winnow::stream::Location as StreamLocation;

pub(crate) fn at_comment_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let head = parse_at_head(parser_input, "comment")?;
    let body = parse_optional_brace_body(
        parser_input,
        BodyWhitespacePolicy::Preserve,
        AfterClosePolicy::Preserve,
    )?;
    let end = body
        .as_ref()
        .map(|body| body.end)
        .unwrap_or_else(|| parser_input.previous_token_end());
    let (body_open_span, body_close_span, value) = body
        .map(|body| {
            (
                Some(body.open_span),
                Some(body.close_span),
                body.content.to_string(),
            )
        })
        .unwrap_or_default();

    // Comments keep raw brace content because renderers usually discard them
    // instead of rendering child elements.
    Ok(Element::Comment(CommentElement {
        span: Span {
            start: head.start,
            end,
        },
        body_open_span,
        body_close_span,
        value,
    }))
}
