//! Protein feature identity and source location, including uncertainty.
use crate::{Term, UncertainSpan0};

/// A UniProt protein or explicit isoform accession.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct UniProtIsoform(Box<str>);
impl UniProtIsoform {
    /// Check a UniProtKB accession and optional numeric isoform suffix.
    /// Source: https://www.uniprot.org/help/accession_numbers
    pub fn new(value: impl Into<Box<str>>) -> Option<Self> {
        let value = value.into();
        let (base, isoform) = value
            .split_once('-')
            .map_or((value.as_ref(), None), |(base, suffix)| {
                (base, Some(suffix))
            });
        let bytes = base.as_bytes();
        let uppercase = |index: usize| bytes.get(index).is_some_and(u8::is_ascii_uppercase);
        let digit = |index: usize| bytes.get(index).is_some_and(u8::is_ascii_digit);
        let alnum = |index: usize| {
            bytes
                .get(index)
                .is_some_and(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        };
        let first_short = matches!(bytes.first(), Some(b'O' | b'P' | b'Q'));
        let first_long = bytes
            .first()
            .is_some_and(|byte| matches!(byte, b'A'..=b'N' | b'R'..=b'Z'));
        let six = bytes.len() == 6
            && digit(1)
            && alnum(3)
            && alnum(4)
            && digit(5)
            && ((first_short && alnum(2)) || (first_long && uppercase(2)));
        let ten = bytes.len() == 10
            && first_long
            && digit(1)
            && uppercase(2)
            && alnum(3)
            && alnum(4)
            && digit(5)
            && uppercase(6)
            && alnum(7)
            && alnum(8)
            && digit(9);
        let suffix = isoform.is_none_or(|suffix| {
            !suffix.is_empty()
                && !suffix.starts_with('0')
                && suffix.bytes().all(|byte| byte.is_ascii_digit())
        });
        (six || ten).then_some(())?;
        suffix.then_some(Self(value))
    }
    /// Publisher accession.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for UniProtIsoform {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
impl std::str::FromStr for UniProtIsoform {
    type Err = crate::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value).ok_or(crate::Error::UniProtIsoformInvalid)
    }
}
impl TryFrom<String> for UniProtIsoform {
    type Error = UniProtIsoformError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(UniProtIsoformError)
    }
}
impl From<UniProtIsoform> for String {
    fn from(value: UniProtIsoform) -> Self {
        value.0.into()
    }
}
/// Invalid UniProt accession or isoform text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UniProtIsoformError;
impl UniProtIsoformError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        "id.uniprot_isoform.invalid"
    }
}
impl std::fmt::Display for UniProtIsoformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for UniProtIsoformError {}

/// A nonempty string of one-letter standard amino-acid codes plus U, O and X.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct AminoAcids(Box<str>);
impl AminoAcids {
    /// Check the stated residue text without changing it.
    pub fn new(value: &str) -> Result<Self, AminoAcidsError> {
        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| b"ACDEFGHIKLMNPQRSTVWYUOX".contains(&byte))
        {
            return Err(AminoAcidsError);
        }
        Ok(Self(value.into()))
    }
    /// Original publisher text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::str::FromStr for AminoAcids {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value).map_err(|_| crate::Error::ProteinResidueInvalid)
    }
}
impl std::fmt::Display for AminoAcids {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
impl TryFrom<String> for AminoAcids {
    type Error = AminoAcidsError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}
impl From<AminoAcids> for String {
    fn from(value: AminoAcids) -> Self {
        value.0.into()
    }
}
/// Invalid protein residue text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AminoAcidsError;
impl AminoAcidsError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        "protein.residue.invalid"
    }
}
impl std::fmt::Display for AminoAcidsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for AminoAcidsError {}

/// One source feature with its exact protein context.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniProtFeature {
    /// Protein sequence declared for this feature.
    pub protein: UniProtIsoform,
    /// Source location, including uncertain bounds.
    pub span: UncertainSpan0,
    /// Publisher feature-type term.
    pub kind: Term,
    /// Publisher description.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<crate::Text>,
    /// Publisher feature identifier.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<crate::Text>,
    /// Source-stated original residues.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub original: Option<AminoAcids>,
    /// Source-stated alternative residues in order.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub alternative: Vec<AminoAcids>,
    /// Publisher evidence references.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub evidence: Vec<crate::Text>,
}
