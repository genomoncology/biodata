use super::{AuthorId, SemanticScholarAuthorId};
use crate::{Curie, Doi, PartialDate, Pmcid, Pmid, SemanticScholarId, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
crate::wire_enum! {
    /// Scope of source-reported bibliographic counts.
    AuthorCountScope {
        /// Only the requested provider record's indexed papers and citations.
        ProviderRecord => "provider_record"
    }
}
/// Counts describe only this provider record, never a global publication corpus.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorCounts {
    /// Counts apply only to this provider record.
    pub scope: AuthorCountScope,
    /// Native indexed paper count.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub paper_count: Option<u64>,
    /// Native citations to this record’s indexed papers.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub citation_count: Option<u64>,
    /// Native h-index for this record’s indexed papers.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub h_index: Option<u32>,
}
/// One source candidate; absent affiliations differ from an empty source collection.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorHeader {
    /// Exact qualified provider record.
    pub id: AuthorId,
    /// Source display name, never a join key.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "schema", schemars(with = "Text"))]
    pub name: Option<Text>,
    /// Native affiliation labels; no employment claim.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "crate::absent_or_empty")
    )]
    pub affiliations: Option<Box<[Text]>>,
    /// Source record counts when supplied.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub counts: Option<AuthorCounts>,
}
/// Exact source author metadata without identity reconciliation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorCard {
    /// Exact source candidate.
    pub header: AuthorHeader,
    /// Provider record link.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
}
/// One indexed paper in source order; identifiers come only from that source.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorPaperHeader {
    /// Native source paper ID.
    pub paper_id: SemanticScholarId,
    /// Exact native PMID.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pmid: Option<Pmid>,
    /// Exact native PMCID.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pmcid: Option<Pmcid>,
    /// Exact native DOI.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub doi: Option<Doi>,
    /// Every other native paper identifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub external_ids: Box<[AuthorPaperExternalId]>,
    /// Whole source title.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<Text>,
    /// Source publication year.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub year: Option<u32>,
    /// Source venue when supplied and nonempty.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub venue: Option<Text>,
}
/// An open native identifier namespace and value; no identity inference.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct AuthorPaperExternalId {
    /// Explicit native namespace supplied with the external identifier.
    pub namespace: Text,
    /// Native namespace and identifier.
    pub id: Curie,
}
/// One source byline row; repeated display names remain separate.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorByline {
    /// Source author ID when supplied.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<SemanticScholarAuthorId>,
    /// Native display name.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Text>,
}
/// Native access metadata; access does not establish reuse rights.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorPaperAccess {
    /// Native PDF URL.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
    /// Open native access status.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub status: Option<Text>,
    /// Native license label.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub license: Option<Text>,
    /// Native source disclaimer.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub disclaimer: Option<Text>,
}
/// Detailed indexed paper metadata for the same native paper page.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AuthorPaperDetail {
    /// Same compact paper identity.
    pub header: AuthorPaperHeader,
    /// Source abstract when available.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub abstract_text: Option<Text>,
    /// Native publication date.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub publication_date: Option<PartialDate>,
    /// Native references on this paper.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub reference_count: Option<u64>,
    /// Native citations to this paper.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub citation_count: Option<u64>,
    /// Native influential citations to this paper.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub influential_citation_count: Option<u64>,
    /// Native open-access flag.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub is_open_access: Option<bool>,
    /// Native access metadata.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub open_access_pdf: Option<AuthorPaperAccess>,
    /// Native study labels.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "crate::absent_or_empty")
    )]
    pub fields_of_study: Option<Box<[Text]>>,
    /// Native publication type labels.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "crate::absent_or_empty")
    )]
    pub publication_types: Option<Box<[Text]>>,
    /// Native ordered byline; completeness is unknown.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "crate::absent_or_empty")
    )]
    pub authors: Option<Box<[AuthorByline]>>,
}
