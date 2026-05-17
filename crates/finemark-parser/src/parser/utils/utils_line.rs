use crate::parser::ParserInput;
use winnow::Result;
use winnow::prelude::*;
use winnow::token::literal;

/// Returns whether a character terminates a logical FineMark line.
pub fn is_line_end_char(c: char) -> bool {
    c == '\n'
}

/// Consumes one FineMark line break token.
pub fn line_break<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    literal("\n").parse_next(parser_input)
}
