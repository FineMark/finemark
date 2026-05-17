use crate::error::FineMarkError;

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
pub struct ParseContext<'i> {
    pub recursion_depth: usize,
    pub body_depth: usize,
    pub original_input: &'i [u8],
    /// Active parse guards. The same guard must be exited in LIFO order.
    pub guard_stack: Vec<ParseGuard>,
    pub max_recursion_depth: usize,
    pub section_counter: usize,
}

impl<'i> ParseContext<'i> {
    // Creates the default parser context for a top-level document parse.
    pub fn new(input: &'i str) -> Self {
        Self {
            recursion_depth: 0,
            body_depth: 0,
            original_input: input.as_bytes(),
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
            });
        }
        self.recursion_depth = new_depth;
        Ok(())
    }

    pub fn decrease_depth(&mut self) {
        self.recursion_depth = self.recursion_depth.saturating_sub(1);
    }

    pub fn enter_body(&mut self) {
        self.body_depth += 1;
    }

    pub fn exit_body(&mut self) {
        self.body_depth = self.body_depth.saturating_sub(1);
    }

    pub fn is_in_body(&self) -> bool {
        self.body_depth > 0
    }

    pub fn is_at_line_start(&self, offset: usize) -> bool {
        offset == 0 || self.original_input.get(offset - 1) == Some(&b'\n')
    }

    pub fn is_at_max_depth(&self) -> bool {
        self.recursion_depth >= self.max_recursion_depth
    }

    pub fn current_depth(&self) -> usize {
        self.recursion_depth
    }

    pub fn remaining_depth(&self) -> usize {
        self.max_recursion_depth
            .saturating_sub(self.recursion_depth)
    }

    pub fn next_section_index(&mut self) -> usize {
        let idx = self.section_counter;
        self.section_counter += 1;
        idx
    }

    pub fn is_guard_active(&self, guard: ParseGuard) -> bool {
        self.guard_stack.contains(&guard)
    }

    pub fn enter_guard(&mut self, guard: ParseGuard) {
        self.guard_stack.push(guard);
    }

    pub fn exit_guard(&mut self, guard: ParseGuard) {
        let popped = self.guard_stack.pop();
        debug_assert_eq!(
            popped,
            Some(guard),
            "guard stack mismatch: expected to pop {:?}, got {:?}",
            guard,
            popped,
        );
    }
}
