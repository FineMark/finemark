use crate::context::ParseContext;
use crate::parser::ParserInput;
use crate::parser::document::document_parser;
use finemark_ast::{Element, ErrorElement, Span};
use winnow::stream::{LocatingSlice, Location as StreamLocation, Stream};

pub fn parse_document(input: &str) -> Vec<Element<'_>> {
    let context = ParseContext::new(input);

    let mut stateful_input = ParserInput {
        input: LocatingSlice::new(input),
        state: context,
    };

    parse_document_input(&mut stateful_input)
}

pub(crate) fn parse_document_input<'i>(parser_input: &mut ParserInput<'i>) -> Vec<Element<'i>> {
    let initial_checkpoint = parser_input.checkpoint();
    let initial_state = parser_input.state.clone();

    match document_parser(parser_input) {
        Ok(mut elements) => {
            // Parse remaining content as Error element if any
            if !parser_input.input.is_empty() {
                let start = parser_input.current_token_start();
                let remaining = parser_input.input.peek_finish();
                parser_input.input.finish();
                let end = parser_input.previous_token_end();

                elements.push(Element::Error(ErrorElement {
                    span: Span { start, end },
                    value: remaining,
                }));
            }
            elements
        }
        Err(_) => {
            // If parser fails, treat entire input as single Error element
            parser_input.reset(&initial_checkpoint);
            parser_input.state = initial_state;

            let start = parser_input.current_token_start();
            let value = parser_input.input.peek_finish();
            parser_input.input.finish();
            let end = parser_input.previous_token_end();

            vec![Element::Error(ErrorElement {
                span: Span { start, end },
                value,
            })]
        }
    }
}
