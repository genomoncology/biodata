//! Owned VCF site values without input framing or float conversion.
use crate::{Contig, Position1, Sequence};

/// One VCF alternate in source order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Alternate {
    /// Literal bases.
    Sequence(Sequence),
    /// Spanning deletion marker.
    Star,
    /// Angle-bracket symbolic allele.
    Symbolic(Box<str>),
    /// Breakend notation retained verbatim.
    Breakend(Box<str>),
    /// Missing alternate marker.
    Missing,
}

/// A VCF site's typed fields, independent of the raw source line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    /// Source CHROM spelling.
    chromosome: Contig,
    /// One-based POS.
    position: Position1,
    /// Source ID, absent for the dot marker.
    id: Option<Box<str>>,
    /// Padded source REF.
    reference: Sequence,
    /// Source ALTs in order.
    alternates: Box<[Alternate]>,
}

/// A VCF site field violates its value contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SiteError {
    /// REF must hold at least one base.
    EmptyReference,
    /// ALT does not name a supported VCF value.
    InvalidAlternate,
}
impl SiteError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::EmptyReference => "vcf.reference.empty",
            Self::InvalidAlternate => "vcf.alternate.invalid",
        }
    }
}
impl std::fmt::Display for SiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for SiteError {}
impl Alternate {
    /// Parse one VCF ALT field without changing its spelling.
    pub fn parse(text: &str) -> Result<Self, SiteError> {
        match text {
            "." => Ok(Self::Missing),
            "*" => Ok(Self::Star),
            _ if text.starts_with('<') && text.ends_with('>') && text.len() > 2 => {
                Ok(Self::Symbolic(text.into()))
            }
            _ if text.contains('[') || text.contains(']') => Ok(Self::Breakend(text.into())),
            _ => text
                .parse::<Sequence>()
                .ok()
                .filter(|sequence| !sequence.is_empty())
                .map(Self::Sequence)
                .ok_or(SiteError::InvalidAlternate),
        }
    }
    fn is_valid(&self) -> bool {
        match self {
            Self::Sequence(sequence) => !sequence.is_empty(),
            Self::Star | Self::Missing => true,
            Self::Symbolic(text) => matches!(Self::parse(text), Ok(Self::Symbolic(_))),
            Self::Breakend(text) => matches!(Self::parse(text), Ok(Self::Breakend(_))),
        }
    }
}
impl Site {
    /// Construct a site with a padded REF and ordered ALT values.
    pub fn new(
        chromosome: Contig,
        position: Position1,
        id: Option<Box<str>>,
        reference: Sequence,
        alternates: impl Into<Box<[Alternate]>>,
    ) -> Result<Self, SiteError> {
        if reference.is_empty() {
            return Err(SiteError::EmptyReference);
        }
        let alternates = alternates.into();
        if !alternates.iter().all(Alternate::is_valid) {
            return Err(SiteError::InvalidAlternate);
        }
        Ok(Self {
            chromosome,
            position,
            id,
            reference,
            alternates,
        })
    }
    /// Source CHROM spelling.
    pub fn chromosome(&self) -> &Contig {
        &self.chromosome
    }
    /// One-based POS.
    pub fn position(&self) -> Position1 {
        self.position
    }
    /// Source ID, absent for the dot marker.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    /// Padded REF bases.
    pub fn reference(&self) -> &Sequence {
        &self.reference
    }
    /// Ordered source ALTs.
    pub fn alternates(&self) -> &[Alternate] {
        &self.alternates
    }
}
