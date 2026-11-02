//! Owned results of a genomic interval lifted between reference builds.
use crate::{Allele, Build, CoordinateError, GenomicInterval};

/// Why a source span has no contiguous target mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiftFailure {
    /// No single chain block covers the full source span.
    NoContiguousBlock,
}

/// The mapped allele and strand-aware target interval, or a named refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiftOutcome {
    /// One mapped allele in target-plus sequence orientation.
    Mapped {
        /// Target interval, including chain direction.
        interval: GenomicInterval,
        /// Target replacement.
        allele: Allele,
    },
    /// Source span had no contiguous target.
    Unmapped(LiftFailure),
}

/// One lifted source interval and its optional known chain release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lifted {
    source: GenomicInterval,
    target_build: Build,
    outcome: LiftOutcome,
    chain_release: Option<Box<str>>,
}
impl Lifted {
    /// Bind a mapped allele to the same target build, chromosome and span.
    pub fn mapped(
        source: GenomicInterval,
        target: GenomicInterval,
        allele: Allele,
        chain_release: Option<Box<str>>,
    ) -> Result<Self, CoordinateError> {
        let target_build = target.build()?;
        if source.strand.is_none() || target.strand.is_none() {
            return Err(CoordinateError::StrandAbsent);
        }
        if target.chromosome().accession(target_build) != allele.location().sequence() {
            return Err(CoordinateError::AccessionChromosomeMismatch);
        }
        if target.span() != allele.location().span() {
            return Err(CoordinateError::SpanMismatch);
        }
        source.build()?;
        Ok(Self {
            source,
            target_build,
            outcome: LiftOutcome::Mapped {
                interval: target,
                allele,
            },
            chain_release,
        })
    }
    /// Name a source span that has no contiguous target block.
    pub fn unmapped(
        source: GenomicInterval,
        target_build: Build,
        reason: LiftFailure,
        chain_release: Option<Box<str>>,
    ) -> Result<Self, CoordinateError> {
        source.build()?;
        if source.strand.is_none() {
            return Err(CoordinateError::StrandAbsent);
        }
        Ok(Self {
            source,
            target_build,
            outcome: LiftOutcome::Unmapped(reason),
            chain_release,
        })
    }
    /// Strand-aware source interval.
    pub fn source(&self) -> GenomicInterval {
        self.source
    }
    /// Requested target build.
    pub fn target_build(&self) -> Build {
        self.target_build
    }
    /// Mapped allele or named failure.
    pub fn outcome(&self) -> &LiftOutcome {
        &self.outcome
    }
    /// Caller-supplied chain release, when known.
    pub fn chain_release(&self) -> Option<&str> {
        self.chain_release.as_deref()
    }
}
