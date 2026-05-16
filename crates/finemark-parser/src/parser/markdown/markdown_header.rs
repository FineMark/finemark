use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content, with_depth};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use finemark_ast::{Element, HeadingElement, Span};
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

pub(crate) fn markdown_header_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    let header_marks: &str = take_while(1..=6, '#').parse_next(parser_input)?;
    let marker_end = parser_input.previous_token_end();
    let is_folded = opt(literal('!')).parse_next(parser_input)?.is_some();
    opt(literal(' ')).parse_next(parser_input)?;

    let content_start = parser_input.current_token_start();
    let content = line_content(parser_input)?;
    line_break_or_eof(parser_input)?;
    let end = parser_input.previous_token_end();

    let children = parse_header_content(content, content_start, parser_input)?;
    let section_index = parser_input.state.next_section_index();

    Ok(Element::Heading(HeadingElement {
        span: Span { start, end },
        marker_span: Span {
            start,
            end: marker_end,
        },
        level: header_marks.len() as u8,
        is_folded,
        section_index,
        children,
    }))
}

/// Header content is parsed in inline mode so a heading cannot introduce nested
/// block constructs. Section numbering is still written back through the shared
/// parser state after the child parse completes.
fn parse_header_content(
    content: &str,
    content_start: usize,
    parser_input: &mut ParserInput,
) -> Result<Vec<Element>> {
    let mut child_input = ParserInput {
        input: InputSource::new_segmented(
            content,
            if content.is_empty() {
                Vec::new()
            } else {
                vec![SourceSegment {
                    logical_start: 0,
                    original_start: content_start,
                    len: content.len(),
                }]
            },
            content_start,
        ),
        state: parser_input.state.clone(),
    };
    let children = with_depth(&mut child_input, |child_input| {
        let previous_block_mode = child_input
            .state
            .replace_block_mode(crate::context::BlockMode::InlineContent);
        let children = parse_document_input(child_input);
        child_input.state.replace_block_mode(previous_block_mode);
        Ok(children)
    })?;
    parser_input.state = child_input.state;
    Ok(children)
}
