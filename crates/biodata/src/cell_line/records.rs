use super::CellosaurusAccession;
use crate::{HgncId, PartialDate, Text};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::num::NonZeroU32;
crate::serde_object::object_record! {
    /// One complete native cross-reference; database and accession form the source identity.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
    pub struct CellLineXref {
        /// Native database name; no fixed vocabulary.
        pub namespace: Text,
        /// Native accession, including any internal prefix.
        pub id: Text,
        /// Native display label when supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub label: Option<Text>,
        /// Native source URL when supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
        /// Native source category when supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub category: Option<Text>,
        /// Native IRI when supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub iri: Option<Text>,
        /// Native discontinuation qualifier, including values such as probable.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub discontinued: Option<Text>,
    }
}
crate::serde_object::object_record! {
    /// Entry dates and entry version are independent of dataset release and retrieval time.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineEntryDates {
        /// Source entry creation date.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub created: Option<PartialDate>,
        /// Source entry update date.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub updated: Option<PartialDate>,
        /// Source entry version, never a dataset release.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub version: Option<NonZeroU32>,
    }
}
crate::serde_object::object_record! {
    /// One source cell identity with complete independently scoped species and disease collections.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineIdentity {
        /// Primary source accession.
        pub accession: CellosaurusAccession,
        /// Recommended display name, never a join key.
        pub name: Text,
        /// Native source category.
        pub category: Text,
        /// Complete native species references.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub species: Box<[CellLineXref]>,
        /// Complete native disease references; NCIt and ORDO stay separate.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub diseases: Box<[CellLineXref]>,
        /// Native age expression without conversion.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub age: Option<Text>,
        /// Native sex label without classification.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub sex: Option<super::CellLineSex>,
    }
}
crate::serde_object::object_record! {
    /// Complete source warning with its native scope; no stock classifier is derived.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineWarning {
        /// Native warning category, such as Problematic cell line.
        pub scope: Text,
        /// Complete source text including historical and stock qualifications.
        pub text: Text,
    }
}
crate::serde_object::object_record! {
    /// One native display name entry, never an identity key.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineName {
        /// Native name type, preserving identifier and synonym labels.
        pub kind: Text,
        /// Native name spelling and punctuation.
        pub value: Text,
    }
}
crate::serde_object::object_record! {
    /// A native variation gene member with only a source-supplied HGNC identity.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineGene {
        /// Complete original source gene cross-reference.
        pub reference: CellLineXref,
        /// Present only if this source member explicitly carries the HGNC identifier.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub hgnc: Option<HgncId>,
    }
}
crate::serde_object::object_record! {
    /// One native internal publication reference, without requiring PubMed.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineReference {
        /// Complete native resource reference, for example PubMed=9305600.
        pub resource_internal_ref: Text,
    }
}
crate::serde_object::object_record! {
    /// One complete native source row; reference, institution and database provenance stay independent.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineProvenance {
        /// Native publication reference if supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub reference: Option<CellLineReference>,
        /// Native institution name if supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub institution: Option<Text>,
        /// Native institution value if supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub value: Option<Text>,
        /// Complete native database provenance if supplied.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub xref: Option<CellLineXref>,
    }
}
crate::serde_object::object_record! {
    /// One complete native variation row; descriptions do not become normalized alleles.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct CellLineVariation {
        /// Native open variation label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub variation_type: Option<Text>,
        /// Native open mutation label.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub mutation_type: Option<Text>,
        /// Complete native mutation description including all coordinate spellings.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub description: Option<Text>,
        /// Complete native variation note.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub note: Option<Text>,
        /// Native zygosity label; absence is not inferred.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub zygosity: Option<Text>,
        /// Every native gene member, including both fusion members.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub genes: Box<[CellLineGene]>,
        /// Every other variation cross-reference in native order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub references: Box<[CellLineXref]>,
        /// Every native provenance row in source order; no PMID is required.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub provenance: Box<[CellLineProvenance]>,
    }
}
