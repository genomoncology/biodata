//! Checked transcript positions shared by projection and rendering.
use crate::hgvs::{Anchor, Position, PositionAnchor};
use std::num::NonZeroI32;

/// A known coding or noncoding transcript anchor with an optional intron offset.
/// The anchor's one-based distance or position and the offset's nonzero value
/// are checked by their respective types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TranscriptPosition {
    anchor: Anchor,
    offset: Option<NonZeroI32>,
}

impl TranscriptPosition {
    /// Construct a known transcript position with an optional signed intron offset.
    pub const fn new(anchor: Anchor, offset: Option<NonZeroI32>) -> Self {
        Self { anchor, offset }
    }

    /// The upstream, coding, downstream, or noncoding transcript anchor.
    pub const fn anchor(self) -> Anchor {
        self.anchor
    }

    /// Signed nonzero displacement from the anchor, when intronic.
    pub const fn offset(self) -> Option<NonZeroI32> {
        self.offset
    }
}

impl From<TranscriptPosition> for Position {
    fn from(value: TranscriptPosition) -> Self {
        Self {
            anchor: PositionAnchor::Transcript(value.anchor),
            offset: value.offset,
        }
    }
}
