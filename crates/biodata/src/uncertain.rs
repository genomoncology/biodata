//! Partial coordinate bounds and spans on a stated sequence.
use crate::{Accession, CoordinateError};

/// A possible interbase boundary with at least one stated limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "BoundFields", into = "BoundFields")
)]
pub struct Bound {
    min: Option<u32>,
    max: Option<u32>,
}

impl Bound {
    /// Keep partial uncertainty while refusing empty or reversed limits.
    pub fn new(min: Option<u32>, max: Option<u32>) -> Result<Self, CoordinateError> {
        if min.is_none() && max.is_none() {
            return Err(CoordinateError::EmptyBound);
        }
        if min.zip(max).is_some_and(|(a, b)| a > b) {
            return Err(CoordinateError::ReversedBound);
        }
        Ok(Self { min, max })
    }

    /// Lowest possible boundary, if supplied.
    pub const fn min(self) -> Option<u32> {
        self.min
    }

    /// Highest possible boundary, if supplied.
    pub const fn max(self) -> Option<u32> {
        self.max
    }
}

/// A zero-based interbase span with uncertain boundaries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "UncertainSpanFields"))]
pub struct UncertainSpan0 {
    start: Bound,
    end: Bound,
}

impl UncertainSpan0 {
    /// Convert one-based inclusive source bounds without collapsing uncertainty.
    pub fn from_inclusive(start: Bound, end: Bound) -> Result<Self, CoordinateError> {
        if [start.min(), start.max(), end.min(), end.max()].contains(&Some(0)) {
            return Err(CoordinateError::ZeroPosition);
        }
        if start.min().zip(end.max()).is_some_and(|(a, b)| a > b) {
            return Err(CoordinateError::ReversedBounds);
        }
        let preceding = |bound: Option<u32>| {
            bound
                .map(|value| value.checked_sub(1).ok_or(CoordinateError::ZeroPosition))
                .transpose()
        };
        Self::new(
            Bound::new(preceding(start.min())?, preceding(start.max())?)?,
            end,
        )
    }

    /// Refuse a span whose start must follow its end.
    pub fn new(start: Bound, end: Bound) -> Result<Self, CoordinateError> {
        if start.min().zip(end.max()).is_some_and(|(a, b)| a > b) {
            return Err(CoordinateError::ReversedBounds);
        }
        Ok(Self { start, end })
    }

    /// Start boundary range.
    pub const fn start(self) -> Bound {
        self.start
    }

    /// End boundary range.
    pub const fn end(self) -> Bound {
        self.end
    }
}

/// A partial span on its source sequence accession.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct UncertainLocation {
    /// Versioned source sequence.
    pub sequence: Accession,
    /// Zero-based interbase span.
    pub span: UncertainSpan0,
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct BoundFields {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    min: Option<u32>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    max: Option<u32>,
}

#[cfg(feature = "serde")]
impl From<Bound> for BoundFields {
    fn from(bound: Bound) -> Self {
        Self {
            min: bound.min,
            max: bound.max,
        }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<BoundFields> for Bound {
    type Error = CoordinateError;

    fn try_from(fields: BoundFields) -> Result<Self, Self::Error> {
        Self::new(fields.min, fields.max)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct UncertainSpanFields {
    start: Bound,
    end: Bound,
}

#[cfg(feature = "serde")]
impl TryFrom<UncertainSpanFields> for UncertainSpan0 {
    type Error = CoordinateError;

    fn try_from(fields: UncertainSpanFields) -> Result<Self, Self::Error> {
        Self::new(fields.start, fields.end)
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(Bound, BoundFields);

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(UncertainSpan0, UncertainSpanFields);
