use crate::{HgncId, Identifier, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
mod ids;
pub use ids::{PathwayId, PathwayRef, ReactomeId, WikiPathwaysId};
crate::wire_enum! {
    /// Source-stated human species; no species inference from a title.
    PathwaySpecies { #[doc = "Source states human species."] Human => "Homo sapiens" }
}
/// Neutral pathway facts, without display sections or source calls.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Pathway {
    /// Checked source-owned identity.
    pub id: PathwayId,
    /// Whole native pathway name.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Text>,
    /// Explicit source-stated human species.
    pub species: PathwaySpecies,
    /// Complete native description when present; empty is distinct from absent.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
    /// Native Reactome summation objects in source order; not concatenated.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::absent_or_empty")
    )]
    pub summations: Option<Box<[PathwaySummation]>>,
    /// Native pathway-level literature references, separate from each summation.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::absent_or_empty")
    )]
    pub references: Option<Box<[PathwayLiteratureRef]>>,
    /// WikiPathways catalog citedIn cell, including a native empty cell.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub cited_in: Option<Box<str>>,
    /// Whole native Reactome stIdVersion, separate from stable identity.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub version: Option<Text>,
    /// WikiPathways native revision, separate from identity.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub revision: Option<Text>,
}
/// An open source namespace paired with its complete native identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct PathwaySourceRef {
    /// Native namespace; never discarded during a gene join.
    pub namespace: crate::IdentifierNamespace,
    /// Complete namespace-local identifier, including native isoform suffixes.
    pub id: Text,
}
/// One source membership reference, without name-based gene inference.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayMember {
    /// Native data-node identity; repeated references remain separate nodes.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub node_id: Option<Text>,
    /// Namespaced native reference.
    pub reference: PathwaySourceRef,
    /// Native label, never interpreted as a gene symbol.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Text>,
    /// Native reference URL, including therapeutic references without a drug action.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub source_url: Option<Text>,
    /// Exact resolved gene identifier, absent when no checked join exists.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub gene: Option<HgncId>,
    /// Native reference schema class.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub schema_class: Option<Text>,
    /// Number of source physical entity contexts carrying this exact reference.
    pub physical_context_count: u64,
    /// Complete physical contexts, disclosed only by the explicit details view.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::absent_or_empty")
    )]
    pub participants: Option<Box<[PathwayParticipant]>>,
}
/// An event object or unresolved integer reference; never the parent's inferred ID.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PathwayEventRef {
    /// Native unresolved integer database reference.
    Database {
        /// Positive native database reference.
        db_id: Identifier<NonZeroU64>,
    },
    /// Native event object; fields are absent only when omitted by the source.
    Object {
        /// Native object database identity.
        db_id: Identifier<NonZeroU64>,
        /// Native stable identity when present, never copied from the pathway.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        id: Option<ReactomeId>,
        /// Publisher record URL for the event's own stable identity, never the parent pathway.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        source_url: Option<Text>,
        /// Whole native stIdVersion, separate from the event stable identity.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        version: Option<Text>,
        /// Whole native display name.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        name: Option<Text>,
        /// Native class; a reaction is not automatically a pathway.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        schema_class: Option<Text>,
    },
}

/// One native summation, with its own database identity and whole text.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwaySummation {
    /// Native summation database identity, not the pathway identity.
    pub db_id: Identifier<NonZeroU64>,
    /// Complete native text; no sentence clipping or reference reassignment.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub text: Option<Box<str>>,
}
/// A native pathway-level literature reference; never inferred from prose.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayLiteratureRef {
    /// Native literature database reference.
    pub db_id: Identifier<NonZeroU64>,
    /// Native PubMed identifier when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub pubmed: Option<crate::Pmid>,
    /// Native title when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub title: Option<Text>,
    /// Source record URL when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
}
/// One native physical entity carrying a membership reference.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PathwayParticipant {
    /// Native physical entity database identity, distinct from reference identity.
    pub db_id: Identifier<NonZeroU64>,
    /// Whole physical entity label, including source-stated modification context.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Text>,
    /// Native physical entity schema class, separate from reference schema class.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub schema_class: Option<Text>,
}
