use crate::parser::ParserInput;
use crate::parser::comment::{inline_comment_parser, multiline_comment_parser};
use crate::parser::escape::escape_parser;
use crate::parser::text::text_parser;
use crate::parser::token::{token_backslash_parser, token_newline_parser, token_slash};
use finemark_ast::Element;
use winnow::combinator::{alt, peek, repeat};
use winnow::prelude::*;
use winnow::token::any;
use winnow::{Result, dispatch};

pub(crate) fn element_parser(parser_input: &mut ParserInput) -> Result<Element> {
    dispatch! {peek(any);
        '\\' => alt((escape_parser, token_backslash_parser)),
        '/' => alt((multiline_comment_parser, inline_comment_parser, token_slash)),
        '\n' => token_newline_parser,
         _ => text_parser,
    }
    .parse_next(parser_input)
}

pub(crate) fn inline_content_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    repeat(1.., element_parser).parse_next(parser_input)
}
