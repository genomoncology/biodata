use crate::ClinPgxAccession;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Source-native PgxObjectReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxObjectReference {
    /// Native `objCls` when supplied.
    #[cfg_attr(feature = "serde", serde(rename = "objCls"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub obj_cls: Option<Box<str>>,
    /// Native `id`; literal source value.
    pub id: ClinPgxAccession,
    /// Native `name` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Box<str>>,
    /// Native `symbol` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub symbol: Option<Box<str>>,
}
/// Source-native PgxCrossReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxCrossReference {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `resource`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resource: Option<Box<str>>,
    /// Native `resourceId`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "resourceId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resource_id: Option<Box<str>>,
    /// Native `_url`; literal source value.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub _url: Option<Box<str>>,
    /// Native `name` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Box<str>>,
}
/// Native location diplotype reference; its numeric ID is not a ClinPGx accession.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxDiplotypeReference {
    /// Native `id`; no expression, phase or phenotype is inferred.
    pub id: crate::Identifier<i64>,
}
/// Source-native PgxLocation fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxLocation {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `buildVersion` when supplied.
    #[cfg_attr(feature = "serde", serde(rename = "buildVersion"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub build_version: Option<Box<str>>,
    /// Native `chromosomeId` when supplied.
    #[cfg_attr(feature = "serde", serde(rename = "chromosomeId"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub chromosome_id: Option<Box<str>>,
    /// Native `chromosomeName` when supplied.
    #[cfg_attr(feature = "serde", serde(rename = "chromosomeName"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub chromosome_name: Option<Box<str>>,
    /// Native `copyNumber`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "copyNumber"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub copy_number: Option<Box<str>>,
    /// Native `diplotypes`; literal source value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub diplotypes: Box<[PgxDiplotypeReference]>,
    /// Native `displayName`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "displayName"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<Box<str>>,
    /// Native `genes`; literal source value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub genes: Box<[PgxObjectReference]>,
    /// Native `gpPosition`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "gpPosition"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gp_position: Option<i64>,
    /// Native `haplotypes`; literal source value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub haplotypes: Box<[PgxObjectReference]>,
    /// Native `linkedObjects`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "linkedObjects"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub linked_objects: Box<[PgxObjectReference]>,
    /// Native `refSeqCrossReference` when supplied.
    #[cfg_attr(feature = "serde", serde(rename = "refSeqCrossReference"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub ref_seq_cross_reference: Option<PgxCrossReference>,
    /// Native `refSeqPosition`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "refSeqPosition"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ref_seq_position: Option<i64>,
    /// Native `rsid` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub rsid: Option<Box<str>>,
    /// Native `tagGene`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "tagGene"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub tag_gene: Option<bool>,
    /// Native `type`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub kind: Option<Box<str>>,
    /// Native `variant` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub variant: Option<PgxObjectReference>,
}
/// Source-native PgxEvidenceTerm fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxEvidenceTerm {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `resource`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resource: Option<Box<str>>,
    /// Native `term`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub term: Option<Box<str>>,
    /// Native `termId`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "termId"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub term_id: Option<Box<str>>,
    /// Native `synonyms` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::absent_or_empty")
    )]
    pub synonyms: Option<Box<[PgxUnobserved]>>,
    /// Native `valid` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub valid: Option<bool>,
}
/// Source-native PgxAnnotationHeader fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationHeader {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `accessionId`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "accessionId"))]
    pub accession_id: ClinPgxAccession,
    /// Native `levelOfEvidence`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "levelOfEvidence"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub level_of_evidence: Option<PgxEvidenceTerm>,
    /// Native `location`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub location: Option<PgxLocation>,
    /// Native `name`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Box<str>>,
    /// Native `relatedChemicals`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "relatedChemicals"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub related_chemicals: Box<[PgxObjectReference]>,
    /// Native `relatedChemicalsLogic`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "relatedChemicalsLogic"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub related_chemicals_logic: Option<Box<str>>,
    /// Native `relatedGuidelines`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "relatedGuidelines"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub related_guidelines: Box<[PgxObjectReference]>,
    /// Native `relatedLabels`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "relatedLabels"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub related_labels: Box<[PgxObjectReference]>,
    /// Native `types`; literal source value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub types: Box<[Box<str>]>,
}
/// Source-native PgxAnnotationReference fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationReference {
    /// Native `objCls`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "objCls"))]
    pub obj_cls: Box<str>,
    /// Native `id`; literal source value.
    pub id: ClinPgxAccession,
    /// Native `name`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Box<str>>,
    /// Native `crossReferences`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "crossReferences"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub cross_references: Box<[PgxUnobserved]>,
}
/// No element shape was captured for these empty arrays. Populated values require new source proof.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PgxUnobserved {}
