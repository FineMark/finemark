use std::fmt;
use std::ops::Deref;

use winnow::stream::{
    AsBStr, AsBytes, Compare, CompareResult, FindSlice, LocatingSlice, Location, Needed, Offset,
    SliceLen, Stream, StreamIsPartial, UpdateSlice,
};

/// A lightweight wrapper around `LocatingSlice` that maintains an absolute base offset.
/// This ensures that Spans computed in child parsers (e.g., inside block content)
/// remain relative to the original start of the document.
#[derive(Clone)]
pub struct InputSource<'i> {
    original: &'i str,
    logical: LocatingSlice<&'i str>,
    base: usize,
}

impl<'i> InputSource<'i> {
    pub fn new(input: &'i str) -> Self {
        Self::new_at(input, 0)
    }

    pub fn new_at(input: &'i str, base: usize) -> Self {
        Self {
            original: input,
            logical: LocatingSlice::new(input),
            base,
        }
    }

    /// Creates a child `InputSource` for a `content` slice that was produced by
    /// reading from this source's logical stream.
    ///
    /// The absolute offset is preserved by adding the relative offset of `content`
    /// to the current `base`.
    pub fn child_source_for_content(&self, content: &'i str) -> Self {
        let parent_ptr = self.original.as_ptr() as usize;
        let child_ptr = content.as_ptr() as usize;

        // Pointer arithmetic to find the relative offset within this slice
        let logical_start = child_ptr - parent_ptr;
        Self::new_at(content, self.base + logical_start)
    }
}

impl fmt::Debug for InputSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logical.fmt(f)
    }
}

impl fmt::Display for InputSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logical.fmt(f)
    }
}

impl Deref for InputSource<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.logical.as_ref()
    }
}

impl SliceLen for InputSource<'_> {
    fn slice_len(&self) -> usize {
        self.logical.slice_len()
    }
}

impl<'i> Stream for InputSource<'i> {
    type Token = <LocatingSlice<&'i str> as Stream>::Token;
    type Slice = <LocatingSlice<&'i str> as Stream>::Slice;
    type IterOffsets = <LocatingSlice<&'i str> as Stream>::IterOffsets;
    type Checkpoint = <LocatingSlice<&'i str> as Stream>::Checkpoint;

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.logical.iter_offsets()
    }

    fn eof_offset(&self) -> usize {
        self.logical.eof_offset()
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        self.logical.next_token()
    }

    fn peek_token(&self) -> Option<Self::Token> {
        self.logical.peek_token()
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.logical.offset_for(predicate)
    }

    fn offset_at(&self, tokens: usize) -> Result<usize, Needed> {
        self.logical.offset_at(tokens)
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        self.logical.next_slice(offset)
    }

    unsafe fn next_slice_unchecked(&mut self, offset: usize) -> Self::Slice {
        unsafe { self.logical.next_slice_unchecked(offset) }
    }

    fn peek_slice(&self, offset: usize) -> Self::Slice {
        self.logical.peek_slice(offset)
    }

    unsafe fn peek_slice_unchecked(&self, offset: usize) -> Self::Slice {
        unsafe { self.logical.peek_slice_unchecked(offset) }
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.logical.checkpoint()
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.logical.reset(checkpoint);
    }

    fn trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logical.trace(f)
    }
}

impl Location for InputSource<'_> {
    fn previous_token_end(&self) -> usize {
        self.logical.previous_token_end() + self.base
    }

    fn current_token_start(&self) -> usize {
        self.logical.current_token_start() + self.base
    }
}

impl<'i> StreamIsPartial for InputSource<'i> {
    type PartialState = <LocatingSlice<&'i str> as StreamIsPartial>::PartialState;

    fn complete(&mut self) -> Self::PartialState {
        self.logical.complete()
    }

    fn restore_partial(&mut self, state: Self::PartialState) {
        self.logical.restore_partial(state);
    }

    fn is_partial_supported() -> bool {
        <LocatingSlice<&'i str> as StreamIsPartial>::is_partial_supported()
    }

    fn is_partial(&self) -> bool {
        self.logical.is_partial()
    }
}

impl<'i> Offset<<InputSource<'i> as Stream>::Checkpoint> for InputSource<'i> {
    fn offset_from(&self, other: &<InputSource<'i> as Stream>::Checkpoint) -> usize {
        self.logical.offset_from(other)
    }
}

impl AsBytes for InputSource<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.logical.as_bytes()
    }
}

impl AsBStr for InputSource<'_> {
    fn as_bstr(&self) -> &[u8] {
        self.logical.as_bstr()
    }
}

impl<T> Compare<T> for InputSource<'_>
where
    for<'a> &'a str: Compare<T>,
{
    fn compare(&self, other: T) -> CompareResult {
        self.logical.as_ref().compare(other)
    }
}

impl<T> FindSlice<T> for InputSource<'_>
where
    for<'a> &'a str: FindSlice<T>,
{
    fn find_slice(&self, substr: T) -> Option<std::ops::Range<usize>> {
        self.logical.as_ref().find_slice(substr)
    }
}

impl<'i> UpdateSlice for InputSource<'i> {
    fn update_slice(mut self, inner: Self::Slice) -> Self {
        self.logical = self.logical.update_slice(inner);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::stream::Location;

    #[test]
    fn test_initial_base_offset() {
        let input = "hello";
        let source = InputSource::new_at(input, 100);
        assert_eq!(source.current_token_start(), 100);
    }

    #[test]
    fn test_child_source_offset() {
        let input = "0123456789";
        let source = InputSource::new_at(input, 100);

        // "567" is at relative offset 5
        let child_text = &input[5..8];
        let child_source = source.child_source_for_content(child_text);

        // Absolute offset should be 100 (base) + 5 (relative) = 105
        assert_eq!(child_source.current_token_start(), 105);
    }

    #[test]
    fn test_multibyte_utf8_offset() {
        // "한글" is 6 bytes (3 bytes each)
        let input = "한글 world";
        let source = InputSource::new_at(input, 1000);

        // "world" starts after 6 bytes of "한글" and 1 space = 7 bytes
        let child_text = &input[7..];
        let child_source = source.child_source_for_content(child_text);

        assert_eq!(child_source.current_token_start(), 1007);
    }
}
