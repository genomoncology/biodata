use super::annotation::{PgxEvidenceTerm, PgxLocation, PgxObjectReference};
use crate::ClinPgxAccession;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Source-native PgxAllelePhenotype fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAllelePhenotype {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `allele`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub allele: Option<Box<str>>,
    /// Native `limitedEvidence`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "limitedEvidence"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub limited_evidence: Option<bool>,
    /// Native `phenotype`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub phenotype: Option<Box<str>>,
}
/// Source-native PgxAnnotationHistory fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationHistory {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `date`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub date: Option<Box<str>>,
    /// Native `type`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub kind: Option<Box<str>>,
    /// Native `version`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<i64>,
    /// Native `description` when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
}
/// Source-native PgxAnnotationDetail fields; no clinical interpretation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PgxAnnotationDetail {
    /// Native `id`; literal source value.
    pub id: crate::Identifier<i64>,
    /// Native `accessionId`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "accessionId"))]
    pub accession_id: ClinPgxAccession,
    /// Native `allelePhenotypes`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "allelePhenotypes"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub allele_phenotypes: Box<[PgxAllelePhenotype]>,
    /// Native `conflictingVariantAnnotationIds`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "conflictingVariantAnnotationIds"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub conflicting_variant_annotation_ids: Box<[crate::Identifier<i64>]>,
    /// Native `history`; literal source value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub history: Box<[PgxAnnotationHistory]>,
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
    /// Native `objCls`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "objCls"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub obj_cls: Option<Box<str>>,
    /// Native `overrideLevel`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "overrideLevel"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub override_level: Option<bool>,
    /// Native `pediatric`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pediatric: Option<bool>,
    /// Native `relatedAlleles`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "relatedAlleles"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub related_alleles: Box<[PgxObjectReference]>,
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
    /// Native `relatedDiseases`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "relatedDiseases"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub related_diseases: Box<[PgxObjectReference]>,
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
    /// Native `score`; literal source value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub score: Option<crate::FiniteNumber>,
    /// Native `scoreDetails`; literal source value.
    #[cfg_attr(feature = "serde", serde(rename = "scoreDetails"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub score_details: Option<Box<str>>,
    /// Native `types`; literal source value.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub types: Box<[Box<str>]>,
    /// Native `overrideLevelDescription` when supplied.
    #[cfg_attr(feature = "serde", serde(rename = "overrideLevelDescription"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub override_level_description: Option<Box<str>>,
}
