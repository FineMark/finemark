use crate::parser::ParserInput;
use crate::parser::at::{
    at_comment_parser, at_h1_parser, at_h2_parser, at_h3_parser, at_h4_parser, at_h5_parser,
    at_h6_parser, at_hline_parser, at_link_parser, at_list_parser, at_quote_parser,
    at_table_parser,
};
use crate::parser::code::code_block_parser;
use crate::parser::escape::escape_parser;
use crate::parser::r#macro::macro_br_parser;
use crate::parser::markdown::{
    markdown_bold_parser, markdown_inline_code_parser, markdown_italic_parser,
    markdown_strikethrough_parser, markdown_subscript_parser, markdown_superscript_parser,
    markdown_underline_parser,
};
use crate::parser::text::text_parser;
use crate::parser::token::{
    token_asterisk_parser, token_at_parser, token_backslash_parser, token_caret_parser,
    token_comma_parser, token_newline_parser, token_paragraph_break_parser, token_tilde_parser,
    token_underscore_parser,
};
use finemark_ast::Element;
use winnow::combinator::{alt, peek, repeat};
use winnow::prelude::*;
use winnow::token::any;
use winnow::{Result, dispatch};

pub(crate) fn element_parser<'i>(parser_input: &mut ParserInput<'i>) -> Result<Element<'i>> {
    dispatch! {peek(any);
        '@' => alt((
            macro_br_parser,
            alt((at_h1_parser, at_h2_parser, at_h3_parser, at_h4_parser, at_h5_parser, at_h6_parser)),
            alt((at_quote_parser, at_hline_parser, at_link_parser, at_table_parser, at_list_parser, at_comment_parser)),
            token_at_parser,
        )),
        '*' => alt((markdown_bold_parser, markdown_italic_parser, token_asterisk_parser)),
        '_' => alt((markdown_underline_parser, token_underscore_parser)),
        '~' => alt((markdown_strikethrough_parser, token_tilde_parser)),
        '^' => alt((markdown_superscript_parser, token_caret_parser)),
        ',' => alt((markdown_subscript_parser, token_comma_parser)),
        '\\' => alt((escape_parser, token_backslash_parser)),
        '`' => alt((code_block_parser, markdown_inline_code_parser)),
        '\n' => alt((token_paragraph_break_parser, token_newline_parser)),
         _ => text_parser,
    }
    .parse_next(parser_input)
}

pub(crate) fn inline_content_parser<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<Vec<Element<'i>>> {
    repeat(1.., element_parser).parse_next(parser_input)
}
