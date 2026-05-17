use crate::parser::ParserInput;
use crate::parser::utils::is_line_end_char;
use finemark_ast::{Element, Span, TextElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;

pub fn text_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    let start = parser_input.current_token_start();
    let stop_at_body_close = parser_input.state.is_in_body();
    let parsed_content = take_while(1.., |c: char| {
        !matches!(c, '@' | '*' | '_' | '~' | '^' | ',' | '\\' | '`' | '$')
            && !(stop_at_body_close && c == '}')
            && !is_line_end_char(c)
    })
    .parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Text(TextElement {
        span: Span { start, end },
        value: parsed_content,
    }))
}
