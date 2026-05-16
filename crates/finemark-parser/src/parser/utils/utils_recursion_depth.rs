use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::ParserInput;
use finemark_ast::Element;
use winnow::Result;

pub fn with_depth<'i, T, F>(input: &mut ParserInput<'i>, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput<'i>) -> Result<T>,
{
    input
        .state
        .increase_depth()
        .map_err(|err| err.into_context_error())?;

    let result = parser(input);

    input.state.decrease_depth();

    result
}

pub fn parse_child_with_depth_at<'i, T, F>(
    parent_input: &mut ParserInput<'i>,
    content: &'i str,
    parser: F,
) -> Result<T>
where
    F: FnOnce(&mut ParserInput<'i>) -> Result<T>,
{
    let mut child_input = ParserInput {
        input: parent_input.input.child_source_for_content(content),
        state: parent_input.state.clone(),
    };

    let result = with_depth(&mut child_input, parser);
    parent_input.state = child_input.state;

    result
}

pub fn parse_nested_document_at<'i>(
    parent_input: &mut ParserInput<'i>,
    content: &'i str,
) -> Result<Vec<Element>> {
    parse_child_with_depth_at(parent_input, content, |child_input| {
        let previous_block_mode = child_input
            .state
            .replace_block_mode(BlockMode::NestedDocument);
        let parsed_content = parse_document_input(child_input);
        child_input.state.replace_block_mode(previous_block_mode);
        Ok(parsed_content)
    })
}
