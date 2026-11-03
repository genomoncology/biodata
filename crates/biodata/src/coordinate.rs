//! Checked base positions, half-open spans and structural allele placements.
use crate::{Allele, Base, Build, Chromosome, Sequence, SequenceLocation, Strand};
use std::{fmt, str::FromStr};

/// A coordinate or allele violates its construction contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateError {
    /// An uncertain boundary has neither side stated.
    EmptyBound,
    /// An uncertain boundary's minimum exceeds its maximum.
    ReversedBound,
    /// A base position must be positive.
    ZeroPosition,
    /// The end precedes the start.
    ReversedBounds,
    /// Arithmetic or integer narrowing cannot represent the result.
    PositionOverflow,
    /// A span endpoint cannot be represented.
    SpanOverflow,
    /// A span lies outside its containing sequence or span.
    OutOfBounds,
    /// A stated span disagrees with the span of its paired allele.
    SpanMismatch,
    /// An interval has no stated build.
    BuildAbsent,
    /// A strand-aware result has no stated direction.
    StrandAbsent,
    /// Intervals name different builds or contigs.
    BuildMismatch,
    /// Intervals name different chromosomes, or an accession is not primary.
    ChromosomeMismatch,
    /// A typed allele accession disagrees with its stated chromosome.
    AccessionChromosomeMismatch,
    /// Reference bases do not fill the replaced span.
    ReferenceLengthMismatch,
    /// DNA text contains an invalid nucleotide.
    InvalidSequence,
    /// DNA text contains RNA uracil.
    RnaSequence,
    /// Both allele sides are empty.
    InvalidAlleleShape,
    /// A position is not an unsigned decimal integer.
    InvalidPosition,
}
impl CoordinateError {
    /// Stable machine-readable construction refusal.
    pub const fn code(self) -> &'static str {
        match self {
            Self::EmptyBound => "coord.bound.empty",
            Self::ReversedBound => "coord.bound.order",
            Self::BuildAbsent => "coord.interval.build_absent",
            Self::StrandAbsent => "coord.interval.strand_absent",
            Self::InvalidSequence => "sequence.base.invalid",
            Self::RnaSequence => "sequence.base.rna",
            Self::ZeroPosition => "coord.position.zero",
            Self::ReversedBounds => "coord.span.order",
            Self::PositionOverflow => "coord.position.overflow",
            Self::SpanOverflow => "coord.span.overflow",
            Self::OutOfBounds => "coord.span.outside",
            Self::SpanMismatch => "coord.span.mismatch",
            Self::BuildMismatch => "coord.interval.build_mismatch",
            Self::ChromosomeMismatch => "coord.interval.chromosome_mismatch",
            Self::AccessionChromosomeMismatch => "coord.chromosome.mismatch",
            Self::ReferenceLengthMismatch => "allele.reference.length",
            Self::InvalidAlleleShape => "allele.state.empty",
            Self::InvalidPosition => "coord.position.invalid",
        }
    }
}
impl fmt::Display for CoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for CoordinateError {}

/// A one-based base position; insertion boundaries use `Span0` instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Position1(u32);
impl TryFrom<u32> for Position1 {
    type Error = CoordinateError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<Position1> for u32 {
    fn from(value: Position1) -> Self {
        value.get()
    }
}
impl Position1 {
    /// Reject position zero.
    pub fn new(value: u32) -> Result<Self, CoordinateError> {
        if value == 0 {
            Err(CoordinateError::ZeroPosition)
        } else {
            Ok(Self(value))
        }
    }
    /// The one-based integer.
    pub fn get(self) -> u32 {
        self.0
    }
    /// The zero-based offset of this base.
    pub fn offset0(self) -> u32 {
        self.0 - 1
    }
    /// Advance by a count without overflow.
    pub fn checked_add(self, count: u32) -> Result<Self, CoordinateError> {
        Self::new(
            self.0
                .checked_add(count)
                .ok_or(CoordinateError::PositionOverflow)?,
        )
    }
    /// Retreat by a count without underflow or reaching zero.
    pub fn checked_sub(self, count: u32) -> Result<Self, CoordinateError> {
        Self::new(
            self.0
                .checked_sub(count)
                .ok_or(CoordinateError::PositionOverflow)?,
        )
    }
}
impl FromStr for Position1 {
    type Err = CoordinateError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::new(text.parse().map_err(|_| CoordinateError::InvalidPosition)?)
    }
}
impl fmt::Display for Position1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Position1 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <u32 as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// A zero-based half-open span. Empty spans identify boundaries.
/// This value proves order, not sequence identity or coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "SpanFields"))]
pub struct Span0 {
    start: u32,
    end: u32,
}
impl Span0 {
    /// Check boundary order, allowing equal endpoints.
    pub fn new(start: u32, end: u32) -> Result<Self, CoordinateError> {
        if start > end {
            Err(CoordinateError::ReversedBounds)
        } else {
            Ok(Self { start, end })
        }
    }
    /// Convert a zero-based offset and count with checked narrowing and addition.
    pub fn from_offset_count(start: u32, count: usize) -> Result<Self, CoordinateError> {
        let count = u32::try_from(count).map_err(|_| CoordinateError::SpanOverflow)?;
        Self::new(
            start,
            start
                .checked_add(count)
                .ok_or(CoordinateError::SpanOverflow)?,
        )
    }
    /// Convert an explicit one-based start and base count.
    pub fn from_start_count(start: Position1, count: usize) -> Result<Self, CoordinateError> {
        Self::from_offset_count(start.offset0(), count)
    }
    /// Check the actual sequence length before allocating or indexing.
    pub fn check_bounds(self, length: u32) -> Result<Self, CoordinateError> {
        if self.end > length {
            Err(CoordinateError::OutOfBounds)
        } else {
            Ok(self)
        }
    }
    /// Whether all boundaries of the inner span lie within this span.
    pub fn contains(self, inner: Self) -> bool {
        self.start <= inner.start && inner.end <= self.end
    }
    /// Express a contained span relative to its outer span.
    pub fn relative_to(self, outer: Self) -> Result<Self, CoordinateError> {
        if !outer.contains(self) {
            return Err(CoordinateError::OutOfBounds);
        }
        Self::new(self.start - outer.start, self.end - outer.start)
    }
    /// Inclusive zero-based start boundary.
    pub fn start(self) -> u32 {
        self.start
    }
    /// Exclusive zero-based end boundary.
    pub fn end(self) -> u32 {
        self.end
    }
    /// Number of bases, usable for buffer sizes on supported platforms.
    pub fn len(self) -> usize {
        (self.end - self.start) as usize
    }
    /// Whether this span names only a boundary.
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

crate::serde_object::object_record! {
    /// A source interval with its stated build and strand, when available.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct GenomicInterval {
        /// Stated reference build.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub build: Option<Build>,
        /// Canonical chromosome.
        pub chromosome: Chromosome,
        /// Zero-based half-open bounds.
        pub span: Span0,
        /// Stated direction.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub strand: Option<Strand>,
    }
}
impl GenomicInterval {
    /// Bind an explicitly selected build to a chromosome and bounds.
    pub fn new(build: Build, chromosome: Chromosome, span: Span0) -> Self {
        Self {
            build: Some(build),
            chromosome,
            span,
            strand: None,
        }
    }
    /// State a direction on this genomic interval.
    pub fn with_strand(mut self, strand: Strand) -> Self {
        self.strand = Some(strand);
        self
    }
    /// Require the source to state a build.
    pub fn build(self) -> Result<Build, CoordinateError> {
        self.build.ok_or(CoordinateError::BuildAbsent)
    }
    /// Canonical chromosome.
    pub fn chromosome(self) -> Chromosome {
        self.chromosome
    }
    /// Zero-based half-open bounds.
    pub fn span(self) -> Span0 {
        self.span
    }
    /// Require matching explicit coordinate spaces before comparing bounds.
    pub fn contains(self, inner: Self) -> Result<bool, CoordinateError> {
        if self.build()? != inner.build()? {
            return Err(CoordinateError::BuildMismatch);
        }
        if self.chromosome != inner.chromosome {
            return Err(CoordinateError::ChromosomeMismatch);
        }
        Ok(self.span.contains(inner.span))
    }
    /// Inclusive bounds, absent for an empty interval.
    pub fn inclusive_bounds(self) -> Option<(Position1, Position1)> {
        if self.span.is_empty() {
            return None;
        }
        Some((Position1(self.span.start() + 1), Position1(self.span.end())))
    }
}

crate::serde_object::object_record! {
    /// A stated allele associated with the caller's selected reference build.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
    pub struct AlleleKey {
        build: Build,
        allele: Allele,
    }
}
impl AlleleKey {
    /// Construct a primary chromosome allele from explicit genomic bounds.
    pub fn new(
        interval: GenomicInterval,
        reference: impl Into<Box<[Base]>>,
        alternate: impl Into<Box<[Base]>>,
    ) -> Result<Self, CoordinateError> {
        let build = interval.build()?;
        let location = SequenceLocation::new(interval.chromosome.accession(build), interval.span);
        let allele = Allele::new(location, Sequence::new(reference), Sequence::new(alternate))?;
        Ok(Self::from_allele(build, allele))
    }
    /// Retain the selected build beside any checked source allele.
    pub fn from_allele(build: Build, allele: Allele) -> Self {
        Self { build, allele }
    }
    /// Caller-selected build.
    pub fn build(&self) -> Build {
        self.build
    }
    /// Checked source allele.
    pub fn allele(&self) -> &Allele {
        &self.allele
    }
    /// Resolve a primary genomic interval, refusing non-primary sequence accessions.
    pub fn interval(&self) -> Result<GenomicInterval, CoordinateError> {
        let location = self.allele.location();
        let chromosome = Chromosome::from_accession(location.sequence(), self.build)
            .map_err(|_| CoordinateError::ChromosomeMismatch)?;
        Ok(GenomicInterval::new(
            self.build,
            chromosome,
            location.span(),
        ))
    }
    /// Stated reference bases.
    pub fn reference(&self) -> &[Base] {
        self.allele.reference().as_slice()
    }
    /// Stated replacement bases.
    pub fn alternate(&self) -> &[Base] {
        self.allele.alternate().as_slice()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
struct SpanFields {
    start: u32,
    end: u32,
}
#[cfg(feature = "serde")]
impl TryFrom<SpanFields> for Span0 {
    type Error = CoordinateError;
    fn try_from(value: SpanFields) -> Result<Self, Self::Error> {
        Self::new(value.start, value.end)
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(Span0, SpanFields);
