//! Neutral study records preserve source metadata without I/O or clinical inference.
mod id;
use crate::{Pmid, Text};
pub use id::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Compact source study identity; source codes and counts retain their own scope.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyHeader {
    /// Exact case-sensitive source identity.
    pub study_id: StudyId,
    /// Native study name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Text>,
    /// cBioPortal namespace only; no ontology inference.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub cancer_type_id: Option<CbioportalCancerTypeId>,
    /// Native allSampleCount, never a patient or profiled-gene denominator.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub all_sample_count: Option<u64>,
    /// Every validated source PMID in native order, including repetitions; absent remains absent.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::absent_or_empty")
    )]
    pub pmids: Option<Box<[Pmid]>>,
    /// Native citation, absent for unpublished records.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub citation: Option<Box<str>>,
    /// Native study metadata label, never a coordinate assembly.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub reference_genome: Option<Text>,
}
/// Source-specific study card metadata; no inferred denominator or gene aggregate.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Study {
    /// Compact study identity and publication references.
    pub header: StudyHeader,
    /// Native description.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
    /// Native sequencedSampleCount, not gene coverage.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sequenced_sample_count: Option<u64>,
    /// Native cnaSampleCount.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub cna_sample_count: Option<u64>,
    /// Native structuralVariantCount; its name does not imply sample units.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub structural_variant_count: Option<u64>,
    /// Native treatmentCount without a unit conversion.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub treatment_count: Option<u64>,
    /// Only an explicitly stated card patientCount; captured cards omit it, distinct from zero.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub patient_count: Option<u64>,
    /// Independent resource-scoped counts, never summed into study totals.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub resource_counts: Box<[StudyResourceCount]>,
    /// Native timezone-unstated import time.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub import_date: Option<StudyImportTime>,
    /// Native access groups, possibly empty; not a reuse license.
    pub groups: Box<str>,
    /// Native access metadata; not a reuse license.
    pub read_permission: bool,
    /// Native public-study access metadata.
    pub public_study: bool,
}
/// One native resource scope; counts remain attached to its study and resource.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyResourceCount {
    /// Native resource ID.
    pub resource_id: StudyResourceId,
    /// Native resource display name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub display_name: Option<Box<str>>,
    /// Native resource description.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
    /// Extensible native resource type, not a derived count unit.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub resource_type: Option<Text>,
    /// Native priority string, without numeric reinterpretation.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub priority: Option<Text>,
    /// Native access/display setting, not a reuse license.
    pub open_by_default: bool,
    /// Exact resource study scope.
    pub cancer_study_identifier: StudyId,
    /// Native count within this resource only.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sample_count: Option<u64>,
    /// Native patient count within this resource only.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub patient_count: Option<u64>,
}
mod sections;
pub use sections::*;
