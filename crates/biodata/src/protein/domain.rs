use crate::{Attribution, InterProId, Text, UncertainSpan0, UniProtIsoform};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// One native integrated InterPro match; not every entry type is a domain.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomainMatch {
    /// Integrated entry accession, distinct from member signatures.
    pub id: InterProId,
    /// Native integrated name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Text>,
    /// Literal source entry type, including family, site and repeat.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub entry_type: Option<Text>,
    /// Exact native protein identity.
    pub protein: UniProtIsoform,
    /// Source protein length when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub protein_length: Option<u32>,
    /// Source taxon when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub taxon: Option<u32>,
    /// Every native location group in source order.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub locations: Box<[ProteinDomainLocation]>,
    /// Source member signatures remain distinct from integrated entries.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub member_signatures: Box<[ProteinMemberSignature]>,
    /// Source record and release facts.
    pub attribution: Attribution,
}
/// One native InterPro location group retaining all discontinuous fragments.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomainLocation {
    /// All native fragments; never reduced to a min/max hull.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub fragments: Box<[ProteinDomainFragment]>,
}
/// One native match fragment after coordinate conversion.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProteinDomainFragment {
    /// Converted interbase span with uncertainty retained.
    pub span: UncertainSpan0,
    /// Literal discontinuity status supplied by InterPro.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub dc_status: Option<Text>,
}
/// A member database signature, not an integrated IPR entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct ProteinMemberSignature {
    /// Native member database name.
    pub namespace: Text,
    /// Native signature accession.
    pub id: Text,
    /// Native member signature name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Text>,
}
