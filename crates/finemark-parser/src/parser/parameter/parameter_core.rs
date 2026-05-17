use crate::parser::ParserInput;
use crate::parser::parameter::parameter_content::parameter_content_parser;
use finemark_ast::{Parameter, Parameters, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt, preceded, separated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

fn identifier<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == '-').parse_next(parser_input)
}

fn parameter_parser(parser_input: &mut ParserInput) -> Result<(String, Parameter)> {
    // Keep spans tight around the actual parameter, excluding separator whitespace.
    multispace0.parse_next(parser_input)?;
    let start = parser_input.current_token_start();

    // Parse `key` or `key="value"`. Values are element lists so later stages can
    // resolve escapes or variables without making the parameter parser semantic.
    let key = identifier(parser_input)?;
    let value = opt(preceded(
        delimited(multispace0, literal('='), multispace0),
        delimited(literal('"'), parameter_content_parser, literal('"')),
    ))
    .parse_next(parser_input)?
    .unwrap_or_default();

    let end = parser_input.previous_token_end();
    multispace0.parse_next(parser_input)?;

    let key = key.to_string();
    let parameter = Parameter {
        span: Span { start, end },
        key: key.clone(),
        value,
    };

    Ok((key, parameter))
}

fn comma_separator<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    delimited(multispace0, literal(","), multispace0).parse_next(parser_input)
}

fn parameter_list_parser(parser_input: &mut ParserInput) -> Result<Parameters> {
    // Parse only entries and separators. Delimiters belong to the caller.
    // IndexMap preserves source order for rendering/LSP while supporting lookup.
    separated(0.., parameter_parser, comma_separator)
        .map(|pairs: Vec<_>| pairs.into_iter().collect::<Parameters>())
        .parse_next(parser_input)
}

pub(crate) fn parameter_core_parser(parser_input: &mut ParserInput) -> Result<Parameters> {
    // Parameter lists are not balanced raw bodies. `)` only closes the list after
    // parameter entries have been parsed, and quoted values consume any `)` they
    // contain as value text (for example: `(label="a ) b")`). Nested parameter
    // parentheses are not part of the grammar, so a raw balanced scanner would accept
    // invalid structure instead of enforcing the parameter-list grammar.
    delimited(
        literal("("),
        delimited(multispace0, parameter_list_parser, multispace0),
        literal(")"),
    )
    .parse_next(parser_input)
}
