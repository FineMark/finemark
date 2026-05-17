use crate::parser::ParserInput;
use crate::parser::at::{
    at_comment_parser, at_h1_parser, at_h2_parser, at_h3_parser, at_h4_parser, at_h5_parser,
    at_h6_parser, at_hline_parser, at_link_parser, at_quote_parser, at_table_parser,
};
use crate::parser::escape::escape_parser;
use crate::parser::text::text_parser;
use crate::parser::token::{token_backslash_parser, token_newline_parser};
use finemark_ast::Element;
use winnow::combinator::{alt, peek, repeat};
use winnow::prelude::*;
use winnow::token::any;
use winnow::{Result, dispatch};

pub(crate) fn element_parser(parser_input: &mut ParserInput) -> Result<Element> {
    dispatch! {peek(any);
        '@' => alt((
            alt((at_h1_parser, at_h2_parser, at_h3_parser, at_h4_parser, at_h5_parser, at_h6_parser)),
            alt((at_quote_parser, at_hline_parser, at_link_parser, at_table_parser, at_comment_parser)),
            text_parser,
        )),
        '\\' => alt((escape_parser, token_backslash_parser)),
        '\n' => token_newline_parser,
         _ => text_parser,
    }
    .parse_next(parser_input)
}

pub(crate) fn inline_content_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    repeat(1.., element_parser).parse_next(parser_input)
}
