//! Nucleotide values preserve RNA bases inside the HGVS grammar.
use crate::{Base, CoordinateError};
use std::fmt;

/// An RNA base, with uracil and the IUPAC ambiguity codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RnaBase(u8);
impl RnaBase {
    fn from_letter(letter: char) -> Option<Self> {
        if letter == 'U' || (letter != 'T' && Base::from_letter(letter).is_some()) {
            Some(Self(letter as u8))
        } else {
            None
        }
    }
}
/// A nucleotide in an HGVS DNA or RNA expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nucleotide {
    /// DNA base.
    Dna(Base),
    /// RNA base.
    Rna(RnaBase),
}
impl Nucleotide {
    pub(crate) fn from_letter(letter: char) -> Option<Self> {
        Base::from_letter(letter).map(Self::Dna)
    }
    pub(crate) fn from_rna_letter(letter: char) -> Option<Self> {
        RnaBase::from_letter(letter).map(Self::Rna)
    }
    /// Require a DNA nucleotide before genomic projection.
    pub fn dna(self) -> Result<Base, CoordinateError> {
        match self {
            Self::Dna(base) => Ok(base),
            Self::Rna(_) => Err(CoordinateError::RnaSequence),
        }
    }
}
impl From<Base> for Nucleotide {
    fn from(base: Base) -> Self {
        Self::Dna(base)
    }
}
impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna(base) => base.fmt(f),
            Self::Rna(base) => write!(f, "{}", char::from(base.0)),
        }
    }
}
/// A nonempty sequence of HGVS nucleotides.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nucleotides(Box<[Nucleotide]>);
impl Nucleotides {
    /// Construct a DNA edit sequence, refusing an empty edit.
    pub fn new(bases: impl Into<Box<[Base]>>) -> Option<Self> {
        let values = bases
            .into()
            .iter()
            .copied()
            .map(Nucleotide::Dna)
            .collect::<Vec<_>>();
        Self::from_nucleotides(values)
    }
    pub(crate) fn from_nucleotides(values: Vec<Nucleotide>) -> Option<Self> {
        (!values.is_empty()).then(|| Self(values.into()))
    }
    /// Ordered HGVS nucleotides.
    pub fn as_slice(&self) -> &[Nucleotide] {
        &self.0
    }
    /// Require DNA before genomic projection.
    pub fn dna(&self) -> Result<Vec<Base>, CoordinateError> {
        self.0.iter().copied().map(Nucleotide::dna).collect()
    }
}
impl fmt::Display for Nucleotides {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for base in &self.0 {
            base.fmt(f)?;
        }
        Ok(())
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Nucleotide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Nucleotides {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
