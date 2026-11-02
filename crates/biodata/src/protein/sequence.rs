use crate::{AminoAcids, PartialDate, Text, UniProtIsoform};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Native source audit metadata, independent from local retrieval metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinAudit {
    /// Native entry version, never an isoform version.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub entry_version: Option<u32>,
    /// Native sequence version when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sequence_version: Option<u32>,
    /// Native first-public date.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub first_public: Option<PartialDate>,
    /// Native annotation update date.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub last_annotation_update: Option<PartialDate>,
    /// Native sequence update date.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub last_sequence_update: Option<PartialDate>,
    /// Native release when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub release: Option<Text>,
}
/// A directly returned sequence with exact returned identity and audit facts.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinSequence {
    /// Exact returned accession; never replaced by the requested base entry.
    pub accession: UniProtIsoform,
    /// Source residues, never constructed from another entry.
    pub sequence: AminoAcids,
    /// Native sequence length.
    pub length: u32,
    /// Native checksum when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub crc64: Option<Text>,
    /// Native checksum when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub md5: Option<Text>,
    /// Native audit facts for this record.
    pub audit: ProteinAudit,
}
/// One whole source function paragraph and its own evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinFunction {
    /// Native molecule qualifier; never attributed to another isoform.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub molecule: Option<Text>,
    /// Whole native source text, without clinical interpretation.
    pub text: Text,
    /// All evidence references supplied for this text.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub evidence: Box<[Text]>,
}
/// A source structure cross-reference link; no residue coverage claim.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct ProteinStructureRef {
    /// Native cross-reference database.
    pub namespace: Text,
    /// Native cross-reference identifier.
    pub id: Text,
    /// Publisher link; no structure file is retrieved.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub url: Option<Text>,
    /// All native property pairs, including method, resolution and chains.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub properties: Box<[ProteinStructureProperty]>,
}
/// A native cross-reference property pair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinStructureProperty {
    /// Native property key.
    pub key: Text,
    /// Native property value, without interpreting residue coverage.
    pub value: Text,
}
