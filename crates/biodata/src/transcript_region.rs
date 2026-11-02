//! Position of an exon, intron, or transcript flank in transcript order.
use crate::Position1;

/// Kind of transcript region.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranscriptRegionKind {
    /// A transcribed exon.
    Exon,
    /// A gap between transcribed exons.
    Intron,
    /// Before the transcript's first base.
    Upstream,
    /// After the transcript's last base.
    Downstream,
}

/// One region with its one-based ordinal and total when numbered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TranscriptRegion {
    kind: TranscriptRegionKind,
    number: Option<u16>,
    total: Option<u16>,
}

impl TranscriptRegion {
    /// Region before the transcript's first base.
    pub const UPSTREAM: Self = Self {
        kind: TranscriptRegionKind::Upstream,
        number: None,
        total: None,
    };
    /// Region after the transcript's last base.
    pub const DOWNSTREAM: Self = Self {
        kind: TranscriptRegionKind::Downstream,
        number: None,
        total: None,
    };
    /// Build a numbered exon or intron. Its ordinal must lie within its total.
    pub const fn numbered(kind: TranscriptRegionKind, number: u16, total: u16) -> Option<Self> {
        if !matches!(
            kind,
            TranscriptRegionKind::Exon | TranscriptRegionKind::Intron
        ) || number == 0
            || total == 0
            || number > total
        {
            return None;
        }
        Some(Self {
            kind,
            number: Some(number),
            total: Some(total),
        })
    }

    /// Build an unnumbered transcript flank.
    pub const fn flank(kind: TranscriptRegionKind) -> Option<Self> {
        if !matches!(
            kind,
            TranscriptRegionKind::Upstream | TranscriptRegionKind::Downstream
        ) {
            return None;
        }
        Some(Self {
            kind,
            number: None,
            total: None,
        })
    }

    /// Exon, intron, upstream, or downstream.
    pub const fn kind(self) -> TranscriptRegionKind {
        self.kind
    }
    /// One-based ordinal when the region is numbered.
    pub const fn number(self) -> Option<u16> {
        self.number
    }
    /// Number of regions of this kind when numbered.
    pub const fn total(self) -> Option<u16> {
        self.total
    }
}

/// An exon or intron in a source transcript annotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(transform = crate::schema_compat::closed_string_enum))]
pub enum RegionKind {
    /// Exon in transcript order.
    Exon,
    /// Intron in transcript order.
    Intron,
}

/// A numbered exon or intron with its stated total.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RegionFields"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from))]
pub struct Region {
    kind: RegionKind,
    number: Position1,
    total: Position1,
}

impl Region {
    /// Refuse an ordinal beyond its source-stated total.
    pub fn new(kind: RegionKind, number: Position1, total: Position1) -> Option<Self> {
        (number <= total).then_some(Self {
            kind,
            number,
            total,
        })
    }

    /// Exon or intron.
    pub const fn kind(self) -> RegionKind {
        self.kind
    }

    /// One-based ordinal.
    pub const fn number(self) -> Position1 {
        self.number
    }

    /// One-based total.
    pub const fn total(self) -> Position1 {
        self.total
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct RegionFields {
    kind: RegionKind,
    number: Position1,
    total: Position1,
}

#[cfg(feature = "serde")]
impl TryFrom<RegionFields> for Region {
    type Error = &'static str;

    fn try_from(fields: RegionFields) -> Result<Self, Self::Error> {
        Self::new(fields.kind, fields.number, fields.total).ok_or("transcript.region.number")
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(Region, RegionFields);
