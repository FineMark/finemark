use crate::error::FineMarkError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockMode {
    #[default]
    FullDocument,
    NestedDocument,
    InlineContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseGuard {
    Bold,
    Italic,
    Strikethrough,
    Subscript,
    Superscript,
    Underline,
    Footnote,
}

#[derive(Debug, Clone)]
pub struct ParseContext {
    pub recursion_depth: usize,
    pub block_mode: BlockMode,
    /// Active parse guards. The same guard must be exited in LIFO order.
    pub guard_stack: Vec<ParseGuard>,
    pub max_recursion_depth: usize,
    pub section_counter: usize,
}

impl ParseContext {
    pub fn new() -> Self {
        Self {
            recursion_depth: 0,
            block_mode: BlockMode::FullDocument,
            guard_stack: Vec::new(),
            max_recursion_depth: 16,
            section_counter: 1,
        }
    }
}
