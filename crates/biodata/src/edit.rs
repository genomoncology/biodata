//! Fieldless sequence edit classes from the Sequence Ontology.

/// Minimal edit class, independent of inserted or deleted bases.
/// Terms follow [Sequence Ontology 2026-08-07](https://raw.githubusercontent.com/The-Sequence-Ontology/SO-Ontologies/master/Ontology_Files/so.obo).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EditKind {
    /// A single-base substitution.
    Substitution,
    /// Removed bases.
    Deletion,
    /// Inserted bases.
    Insertion,
    /// Repeated adjacent bases.
    Duplication,
    /// Reverse-complement replacement.
    Inversion,
    /// Deleted bases replaced with inserted bases.
    Delins,
    /// A stated reference with no sequence alteration.
    Same,
}
impl EditKind {
    pub(crate) const ALL: [Self; 7] = [
        Self::Substitution,
        Self::Deletion,
        Self::Insertion,
        Self::Duplication,
        Self::Inversion,
        Self::Delins,
        Self::Same,
    ];
    /// Sequence Ontology class identifier.
    pub const fn so_id(self) -> &'static str {
        match self {
            Self::Substitution => "SO:1000002",
            Self::Deletion => "SO:0000159",
            Self::Insertion => "SO:0000667",
            Self::Duplication => "SO:1000035",
            Self::Inversion => "SO:1000036",
            Self::Delins => "SO:1000032",
            Self::Same => "SO:0002073",
        }
    }
    /// Sequence Ontology term name.
    pub const fn so_term(self) -> &'static str {
        match self {
            Self::Substitution => "substitution",
            Self::Deletion => "deletion",
            Self::Insertion => "insertion",
            Self::Duplication => "duplication",
            Self::Inversion => "inversion",
            Self::Delins => "delins",
            Self::Same => "no_sequence_alteration",
        }
    }
}
