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
    // new context
    pub fn new() -> Self {
        Self {
            recursion_depth: 0,
            block_mode: BlockMode::FullDocument,
            guard_stack: Vec::new(),
            max_recursion_depth: 16,
            section_counter: 1,
        }
    }

    pub fn increase_depth(&mut self) -> Result<(), FineMarkError> {
        let new_depth = self.recursion_depth + 1;
        if new_depth > self.max_recursion_depth {
            return Err(FineMarkError::RecursionDepthExceeded {
                depth: new_depth,
                max_depth: self.max_recursion_depth,
            })
        }
        self.recursion_depth = new_depth;
        Ok(())
    }

    pub fn decrease_depth(&mut self) {
        self.recursion_depth = self.recursion_depth.saturating_sub(1);
    }

    pub fn is_at_max_depth(&self) -> bool {
        self.recursion_depth >= self.max_recursion_depth
    }

    pub fn current_depth(&self) -> usize {
        self.recursion_depth
    }
}
