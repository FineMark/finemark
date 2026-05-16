use std::fmt;
use std::ops::Deref;

use winnow::stream::{
    AsBStr, AsBytes, Compare, CompareResult, FindSlice, LocatingSlice, Location, Needed, Offset,
    SliceLen, Stream, StreamIsPartial, UpdateSlice,
};

#[derive(Debug, Clone)]
pub struct SourceSegment {
    pub logical_start: usize,
    pub original_start: usize,
    pub len: usize,
}

impl SourceSegment {
    fn logical_end(&self) -> usize {
        self.logical_start + self.len
    }

    fn original_end(&self) -> usize {
        self.original_start + self.len
    }
}

#[derive(Debug, Clone)]
enum SourceMap {
    Identity {
        base: usize,
    },
    Segmented {
        segments: Vec<SourceSegment>,
        empty_original_offset: usize,
    },
}

impl SourceMap {
    fn segment_for_start(segments: &[SourceSegment], offset: usize) -> Option<&SourceSegment> {
        let index = segments.partition_point(|segment| segment.logical_start <= offset);
        if index == 0 {
            return None;
        }

        let segment = &segments[index - 1];
        (offset < segment.logical_end()).then_some(segment)
    }

    fn segment_for_end(segments: &[SourceSegment], offset: usize) -> Option<&SourceSegment> {
        let index = segments.partition_point(|segment| segment.logical_start < offset);
        if index == 0 {
            return None;
        }

        let segment = &segments[index - 1];
        (offset <= segment.logical_end()).then_some(segment)
    }

    fn map_start(&self, offset: usize) -> usize {
        match self {
            SourceMap::Identity { base } => base + offset,
            SourceMap::Segmented {
                segments,
                empty_original_offset,
            } => {
                if segments.is_empty() {
                    return *empty_original_offset;
                }

                if let Some(segment) = Self::segment_for_start(segments, offset) {
                    return segment.original_start + (offset - segment.logical_start);
                }

                segments
                    .last()
                    .map(SourceSegment::original_end)
                    .unwrap_or(*empty_original_offset)
            }
        }
    }

    fn map_end(&self, offset: usize) -> usize {
        match self {
            SourceMap::Identity { base } => base + offset,
            SourceMap::Segmented {
                segments,
                empty_original_offset,
            } => {
                if segments.is_empty() {
                    return *empty_original_offset;
                }

                if let Some(segment) = Self::segment_for_end(segments, offset) {
                    return segment.original_start + (offset - segment.logical_start);
                }

                // Fallback when the offset is not covered by any segment.
                // Symmetric with map_start's fallback (segments.last().original_end()):
                // when the offset is out of range, return the original position at the
                // end of the last segment.
                // Bug: the old code returned segments.first().original_start here,
                // which is the opposite direction from map_start's fallback and could
                // place a non-char-boundary offset into Span.end.
                segments
                    .last()
                    .map(SourceSegment::original_end)
                    .unwrap_or(*empty_original_offset)
            }
        }
    }
}

#[derive(Clone)]
pub struct InputSource<'i> {
    original: &'i str,
    logical: LocatingSlice<&'i str>,
    source_map: SourceMap,
}

impl<'i> InputSource<'i> {
    pub fn new(input: &'i str) -> Self {
        Self::new_at(input, 0)
    }

    pub fn new_at(input: &'i str, base: usize) -> Self {
        Self {
            original: input,
            logical: LocatingSlice::new(input),
            source_map: SourceMap::Identity { base },
        }
    }

    pub fn new_segmented(
        input: &'i str,
        segments: Vec<SourceSegment>,
        empty_original_offset: usize,
    ) -> Self {
        Self {
            original: input,
            logical: LocatingSlice::new(input),
            source_map: SourceMap::Segmented {
                segments,
                empty_original_offset,
            },
        }
    }

    pub fn is_at_line_start(&self) -> bool {
        let offset = self.logical.current_token_start();
        offset == 0 || self.original.as_bytes().get(offset - 1) == Some(&b'\n')
    }

    /// Creates a child `InputSource` for a slice that was extracted from this source's
    /// logical string starting at `logical_start`.
    ///
    /// For an Identity parent the child is also Identity with the correct base offset.
    /// For a Segmented parent the child inherits the relevant segments, so positions
    /// computed inside the child map correctly back to original file offsets even when
    /// the content spans multiple lines (e.g. inside a markdown blockquote where each
    /// line had its `> ` prefix stripped).
    pub fn child_source_for_slice(&self, content: &'i str, logical_start: usize) -> Self {
        let logical_end = logical_start + content.len();
        match &self.source_map {
            SourceMap::Identity { base } => InputSource::new_at(content, base + logical_start),
            SourceMap::Segmented { segments, .. } => {
                let child_segments: Vec<SourceSegment> = segments
                    .iter()
                    .filter(|seg| {
                        seg.logical_start < logical_end && seg.logical_end() > logical_start
                    })
                    .map(|seg| {
                        let clip_start = seg.logical_start.max(logical_start);
                        let clip_end = seg.logical_end().min(logical_end);
                        SourceSegment {
                            logical_start: clip_start - logical_start,
                            original_start: seg.original_start + (clip_start - seg.logical_start),
                            len: clip_end - clip_start,
                        }
                    })
                    .collect();
                let empty_offset = self.source_map.map_start(logical_start);
                InputSource::new_segmented(content, child_segments, empty_offset)
            }
        }
    }

    /// Creates a child `InputSource` for a `content` slice that was produced by
    /// reading from this source's logical stream.
    ///
    /// The logical start offset is computed automatically via pointer arithmetic —
    /// `content` must be a subslice of this source's logical string, which is
    /// always the case when it comes from winnow stream operations
    /// (`next_slice`, `take_till`, `.trim()`, etc.).
    ///
    /// This avoids requiring callers to explicitly track the logical offset.
    pub fn child_source_for_content(&self, content: &'i str) -> Self {
        let base = self.original.as_ptr() as usize;
        let ptr = content.as_ptr() as usize;
        debug_assert!(
            ptr >= base && ptr + content.len() <= base + self.original.len(),
            "content (ptr={ptr:#x}, len={}) is not a subslice of this source's logical string \
             (base={base:#x}, len={})",
            content.len(),
            self.original.len(),
        );
        let logical_start = ptr - base;
        self.child_source_for_slice(content, logical_start)
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
        self.source_map.map_end(self.logical.previous_token_end())
    }

    fn current_token_start(&self) -> usize {
        self.source_map
            .map_start(self.logical.current_token_start())
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

    /// Verifies that map_end's fallback is symmetric with map_start's fallback
    /// (both point to the end of the last segment, not the start of the first).
    /// Old bug: map_end fell back to segments.first().original_start, which is the
    /// opposite direction and could write a non-char-boundary offset into Span.end.
    #[test]
    fn map_end_fallback_symmetric_with_map_start() {
        let segments = vec![SourceSegment {
            logical_start: 0,
            original_start: 10,
            len: 6,
        }];
        let map = SourceMap::Segmented {
            segments,
            empty_original_offset: 0,
        };

        // offset == 6 (last byte of the only segment) must be found by segment_for_end
        assert_eq!(map.map_end(6), 16);

        // out-of-range: old bug returned 10, fixed returns 16 (= last().original_end())
        assert_eq!(
            map.map_end(100),
            16,
            "out-of-range offset must fall back to last().original_end()"
        );
        assert_eq!(
            map.map_start(100),
            map.map_end(100),
            "map_start and map_end must agree on out-of-range offsets"
        );
    }

    /// Verifies that byte offsets within a multi-byte (Korean) segment map correctly.
    #[test]
    fn map_end_korean_multibyte_within_segment() {
        // "한글" is 6 bytes; logical[0..6] maps to original[5..11]
        let segments = vec![SourceSegment {
            logical_start: 0,
            original_start: 5,
            len: 6,
        }];
        let map = SourceMap::Segmented {
            segments,
            empty_original_offset: 0,
        };
        // logical 3 = start of '글' → original 8
        assert_eq!(map.map_end(3), 8);
        assert_eq!(map.map_start(3), 8);
        // logical 6 = end of segment → original 11
        assert_eq!(map.map_end(6), 11);
    }

    /// Verifies boundary and second-segment mapping with two segments.
    #[test]
    fn map_end_two_segments() {
        let segments = vec![
            SourceSegment {
                logical_start: 0,
                original_start: 0,
                len: 3,
            },
            SourceSegment {
                logical_start: 3,
                original_start: 10,
                len: 3,
            },
        ];
        let map = SourceMap::Segmented {
            segments,
            empty_original_offset: 0,
        };
        assert_eq!(map.map_end(2), 2);
        assert_eq!(map.map_end(3), 3); // end of seg1
        assert_eq!(map.map_start(3), 10); // start of seg2
        assert_eq!(map.map_end(5), 12);
        assert_eq!(map.map_end(6), 13);
        // out-of-range: last().original_end() = 13
        assert_eq!(map.map_end(999), 13);
    }

    /// When segments is empty, returns empty_original_offset.
    #[test]
    fn map_end_empty_segments() {
        let map = SourceMap::Segmented {
            segments: vec![],
            empty_original_offset: 42,
        };
        assert_eq!(map.map_end(0), 42);
        assert_eq!(map.map_end(100), 42);
        assert_eq!(map.map_start(0), 42);
    }

    /// Verifies that child_source_for_slice propagates segment mappings correctly
    /// for a multi-line slice extracted from a Segmented parent.
    ///
    /// This is the core fix for the blockquote + multi-line brace content panic:
    /// the old code used new_at(content, original_start) which assumed the original
    /// file bytes matched the logical bytes 1:1 — wrong when "> " was stripped.
    #[test]
    fn child_source_for_slice_segmented_multiline() {
        // Simulate: "> 가나다\n> 라마바\n"
        // Original layout:
        //   byte 0: '>'
        //   byte 1: ' '
        //   byte 2-10: "가나다" (9 bytes)
        //   byte 11: '\n'
        //   byte 12: '>'
        //   byte 13: ' '
        //   byte 14-22: "라마바" (9 bytes)
        //   byte 23: '\n'
        //
        // Blockquote parser strips "> " from each line and produces:
        //   logical: "가나다\n라마바\n"
        //
        // Segments:
        //   {logical_start:0, original_start:2, len:9}   — "가나다"
        //   {logical_start:9, original_start:11, len:1}  — "\n"
        //   {logical_start:10, original_start:14, len:9} — "라마바"
        //   {logical_start:19, original_start:23, len:1} — "\n"
        let logical = "가나다\n라마바\n";
        let segments = vec![
            SourceSegment {
                logical_start: 0,
                original_start: 2,
                len: 9,
            },
            SourceSegment {
                logical_start: 9,
                original_start: 11,
                len: 1,
            },
            SourceSegment {
                logical_start: 10,
                original_start: 14,
                len: 9,
            },
            SourceSegment {
                logical_start: 19,
                original_start: 23,
                len: 1,
            },
        ];
        let parent = InputSource::new_segmented(logical, segments, 0);

        // A brace parser would extract the body between delimiters — here simulate
        // extracting the full content "가나다\n라마바" starting at logical offset 0.
        let content = "가나다\n라마바"; // 19 bytes (9 + 1 + 9)
        let logical_start = 0usize;

        let child = parent.child_source_for_slice(content, logical_start);

        // The child should have segments covering "가나다" (0..9) and "\n" (9..10)
        // and "라마바" (10..19), each remapped relative to logical_start=0.
        // Verify by checking current_token_start mappings on the child:
        // At logical 0: map_start(0) = original 2
        use winnow::stream::Location as StreamLocation;
        assert_eq!(
            child.current_token_start(),
            2,
            "start of 가나다 must map to original 2"
        );

        // At logical 10 (start of "라마바"): map_start(10) = original 14
        // We can't directly set the position, but we can verify the segment via map_start.
        // Access the source_map indirectly through a temporary child:
        let child2 = parent.child_source_for_slice(content, logical_start);
        // The second segment in the child covers logical[10..19] → original[14..23].
        // Check by creating a grandchild slice for "라마바" at logical 10.
        let content2 = "라마바";
        let grandchild = child2.child_source_for_slice(content2, 10);
        assert_eq!(
            grandchild.current_token_start(),
            14,
            "start of 라마바 must map to original 14"
        );
    }

    /// Verifies that child_source_for_slice on an Identity parent behaves like new_at.
    #[test]
    fn child_source_for_slice_identity() {
        let text = "hello world";
        let parent = InputSource::new_at(text, 100);

        use winnow::stream::Location as StreamLocation;

        // Slice "world" starting at logical offset 6
        let content = "world";
        let child = parent.child_source_for_slice(content, 6);
        assert_eq!(
            child.current_token_start(),
            106,
            "identity: base + logical_start"
        );
    }
}
