//! Registry publication and posted-document metadata, without interpretation.
use crate::{License, Pmid, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// An unchanged registry reference in source order.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialReference {
    /// Registry-supplied PubMed identity, when present.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pmid: Option<Pmid>,
    /// Native reference classification, from the registry vocabulary.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub kind: Option<TrialReferenceKind>,
    /// Original citation text.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub citation: Option<Box<str>>,
}
/// A posted source document; a URL never comes from caller-supplied filename text.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialDocument {
    /// Exact advertised portable source filename.
    pub filename: Text,
    /// Fixed registry CDN URL for this manifest entry.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Text,
    /// Native document type abbreviation.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub kind: Option<Box<str>>,
    /// Source label.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<Box<str>>,
    /// Source document date.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub date: Option<crate::PartialDate>,
    /// Original upload timestamp, without guessing a timezone.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub uploaded: Option<crate::TrialUploadTime>,
    /// Source byte count, when supplied.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub size: Option<u64>,
    /// Source says this contains a protocol.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub has_protocol: Option<bool>,
    /// Source says this contains a statistical analysis plan.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub has_sap: Option<bool>,
    /// Source says this contains an informed consent form.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub has_icf: Option<bool>,
    /// No blanket reuse license is inferred from registry availability.
    pub license: License,
}

crate::wire_enum! {
    /// Native ClinicalTrials.gov reference classification.
    TrialReferenceKind {
        /// Registry background reference.
        Background => "BACKGROUND",
        /// Registry result reference.
        Result => "RESULT",
        /// Registry derived reference.
        Derived => "DERIVED",
    }
}
