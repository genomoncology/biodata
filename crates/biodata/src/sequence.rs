//! Literal DNA sequences and checked source allele locations.
use crate::{Accession, Base, CoordinateError, Span0};
use std::{fmt, str::FromStr};

/// A literal IUPAC DNA sequence, including an empty allele side.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sequence(Box<[Base]>);
impl Sequence {
    /// Preserve source DNA bases without normalization.
    pub fn new(bases: impl Into<Box<[Base]>>) -> Self {
        let bases = bases.into();
        Self(bases)
    }
    /// Source bases in order.
    pub fn as_slice(&self) -> &[Base] {
        &self.0
    }
    /// Number of bases.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Whether this allele side is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
/// Reverse and complement typed IUPAC DNA bases.
pub fn reverse_complement(bases: &[Base]) -> Vec<Base> {
    bases.iter().rev().map(|base| base.complement()).collect()
}
/// Count equal flanks that can be removed from a changed replacement.
pub fn trim_flanks(reference: &[Base], alternate: &[Base]) -> (usize, usize) {
    if reference == alternate {
        return (0, 0);
    }
    let prefix = reference
        .iter()
        .zip(alternate)
        .take_while(|(a, b)| a == b)
        .count();
    let suffix = reference
        .iter()
        .skip(prefix)
        .rev()
        .zip(alternate.iter().skip(prefix).rev())
        .take_while(|(a, b)| a == b)
        .count();
    (prefix, suffix)
}
impl FromStr for Sequence {
    type Err = CoordinateError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let bases = text
            .chars()
            .map(|c| {
                Base::from_letter(c).ok_or(if c == 'U' {
                    CoordinateError::RnaSequence
                } else {
                    CoordinateError::InvalidSequence
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(bases))
    }
}
impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for base in &self.0 {
            base.fmt(f)?;
        }
        Ok(())
    }
}

/// A stated replacement without normalization or reference lookup.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "AlleleFields"))]
pub struct Allele {
    location: SequenceLocation,
    reference: Sequence,
    alternate: Sequence,
}
impl Allele {
    /// Check the reference length and refuse two empty allele sides.
    pub fn new(
        location: SequenceLocation,
        reference: Sequence,
        alternate: Sequence,
    ) -> Result<Self, CoordinateError> {
        if reference.len() != location.span.len() {
            return Err(CoordinateError::ReferenceLengthMismatch);
        }
        if reference.is_empty() && alternate.is_empty() {
            return Err(CoordinateError::InvalidAlleleShape);
        }
        Ok(Self {
            location,
            reference,
            alternate,
        })
    }
    /// Versioned source location.
    pub fn location(&self) -> &SequenceLocation {
        &self.location
    }
    /// Stated reference bases.
    pub fn reference(&self) -> &Sequence {
        &self.reference
    }
    /// Stated replacement bases.
    pub fn alternate(&self) -> &Sequence {
        &self.alternate
    }
    /// Remove equal flanking bases without changing the represented replacement.
    pub fn trimmed(&self) -> Self {
        if self.reference == self.alternate {
            return self.clone();
        }
        let reference = self.reference.as_slice();
        let alternate = self.alternate.as_slice();
        let (prefix, suffix) = trim_flanks(reference, alternate);
        let span = Span0::new(
            self.location.span.start() + prefix as u32,
            self.location.span.end() - suffix as u32,
        )
        .unwrap_or(self.location.span);
        Self {
            location: SequenceLocation::new(self.location.sequence, span),
            reference: Sequence::new(
                reference
                    .iter()
                    .skip(prefix)
                    .take(reference.len() - prefix - suffix)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            alternate: Sequence::new(
                alternate
                    .iter()
                    .skip(prefix)
                    .take(alternate.len() - prefix - suffix)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
        }
    }
}
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
struct AlleleFields {
    location: SequenceLocation,
    reference: Sequence,
    alternate: Sequence,
}
#[cfg(feature = "serde")]
impl TryFrom<AlleleFields> for Allele {
    type Error = CoordinateError;
    fn try_from(value: AlleleFields) -> Result<Self, Self::Error> {
        Self::new(value.location, value.reference, value.alternate)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Sequence {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Sequence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for Base {
    type Err = CoordinateError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut letters = text.chars();
        let first = letters.next().ok_or(CoordinateError::InvalidSequence)?;
        if letters.next().is_some() {
            return Err(CoordinateError::InvalidSequence);
        }
        Base::from_letter(first).ok_or(if first == 'U' {
            CoordinateError::RnaSequence
        } else {
            CoordinateError::InvalidSequence
        })
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Base {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

crate::serde_object::object_record! {
    /// An interbase interval on one versioned sequence accession.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct SequenceLocation {
        /// Versioned source sequence, without an inferred genome build.
        #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_sequence"))]
        pub sequence: Accession,
        /// Zero-based half-open bounds.
        pub span: Span0,
    }
}

#[cfg(feature = "serde")]
fn serialize_sequence<S: serde::Serializer>(
    sequence: &Accession,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    checked_sequence(sequence).map_err(serde::ser::Error::custom)?;
    serde::Serialize::serialize(sequence, serializer)
}

// Accession's copied enum permits public field construction. Reuse its parser at
// receiving boundaries so unchecked enum fields cannot escape as an allele.
#[cfg(feature = "serde")]
fn checked_sequence(sequence: &Accession) -> Result<(), crate::Refusal> {
    sequence.to_string().parse::<Accession>().map(|_| ())
}

impl SequenceLocation {
    /// Bind a checked accession to checked interbase bounds.
    pub fn new(sequence: Accession, span: Span0) -> Self {
        Self { sequence, span }
    }
    /// Versioned sequence identifier.
    pub fn sequence(&self) -> Accession {
        self.sequence
    }
    /// Interbase source span.
    pub fn span(&self) -> Span0 {
        self.span
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(Allele, AlleleFields);
