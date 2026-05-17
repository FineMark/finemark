use crate::parser::ParserInput;
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

pub fn with_body<'i, T, F>(input: &mut ParserInput<'i>, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput<'i>) -> Result<T>,
{
    input.state.enter_body();
    let result = parser(input);
    input.state.exit_body();
    result
}
