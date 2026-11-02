//! Gene-qualified protein residues and residue ranges.
use crate::{Accession, AccessionBase, GeneRef, Position1, Text};

/// A transcript accession with its source-stated version status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TranscriptReference {
    /// Versioned transcript.
    Versioned(Accession),
    /// Unversioned transcript base.
    Base(AccessionBase),
}

/// A residue or residue interval, without a variant assertion.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "ProteinSiteFields"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(!try_from))]
pub struct ProteinSite {
    /// Source gene identity.
    pub gene: GeneRef,
    /// Source transcript, with version status kept.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub transcript: Option<TranscriptReference>,
    /// Source protein accession.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub protein: Option<Accession>,
    /// First one-based residue.
    position: Position1,
    /// Last one-based residue, if a range.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    end: Option<Position1>,
    /// Source residue label, if stated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub residue: Option<Text>,
}

impl ProteinSite {
    /// Refuse a residue range ending before its first residue.
    pub fn new(
        gene: GeneRef,
        transcript: Option<TranscriptReference>,
        protein: Option<Accession>,
        position: Position1,
        end: Option<Position1>,
        residue: Option<Text>,
    ) -> Result<Self, ProteinSiteError> {
        if !Self::ordered(position, end) {
            return Err(ProteinSiteError);
        }
        Ok(Self {
            gene,
            transcript,
            protein,
            position,
            end,
            residue,
        })
    }

    /// First one-based residue.
    pub fn position(&self) -> Position1 {
        self.position
    }

    /// Last one-based residue, if a range.
    pub fn end(&self) -> Option<Position1> {
        self.end
    }

    /// Change the last residue if the resulting range remains ordered.
    pub fn set_end(&mut self, end: Option<Position1>) -> Result<(), ProteinSiteError> {
        if !Self::ordered(self.position, end) {
            return Err(ProteinSiteError);
        }
        self.end = end;
        Ok(())
    }

    /// Change the first residue if the resulting range remains ordered.
    pub fn set_position(&mut self, position: Position1) -> Result<(), ProteinSiteError> {
        if !Self::ordered(position, self.end) {
            return Err(ProteinSiteError);
        }
        self.position = position;
        Ok(())
    }

    fn ordered(position: Position1, end: Option<Position1>) -> bool {
        end.is_none_or(|last| last >= position)
    }
}

/// An ordered protein site was invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProteinSiteError;

impl std::fmt::Display for ProteinSiteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("protein.site.order")
    }
}

impl std::error::Error for ProteinSiteError {}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct ProteinSiteFields {
    gene: GeneRef,
    transcript: Option<TranscriptReference>,
    protein: Option<Accession>,
    position: Position1,
    end: Option<Position1>,
    residue: Option<Text>,
}

#[cfg(feature = "serde")]
impl TryFrom<ProteinSiteFields> for ProteinSite {
    type Error = ProteinSiteError;

    fn try_from(fields: ProteinSiteFields) -> Result<Self, Self::Error> {
        Self::new(
            fields.gene,
            fields.transcript,
            fields.protein,
            fields.position,
            fields.end,
            fields.residue,
        )
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(ProteinSite, ProteinSiteFields);
