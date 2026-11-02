use super::{StudyBlobSha, StudyId, StudyProfileId, StudySampleListId};
use crate::Text;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Molecular profile reference with exact study membership.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyProfile {
    /// Exact native profile ID; never constructed from a study ID.
    pub molecular_profile_id: StudyProfileId,
    /// Native profile study scope.
    pub study_id: StudyId,
    /// Extensible source alteration-type label.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub molecular_alteration_type: Option<Text>,
    /// Extensible native data-type label.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub datatype: Option<Text>,
    /// Source profile name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Box<str>>,
    /// Source description.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
    /// Native generic assay type, absent outside those profiles.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub generic_assay_type: Option<Text>,
    /// Native patient-level flag, not a unit inferred for other records.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub patient_level: Option<bool>,
    /// Native profile display setting.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub show_profile_in_analysis_tab: Option<bool>,
}
/// Sample-list metadata without a representation for member sample IDs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudySampleList {
    /// Exact native list ID.
    pub sample_list_id: StudySampleListId,
    /// Native study scope.
    pub study_id: StudyId,
    /// Extensible native list category.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub category: Option<Text>,
    /// Native list name.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub name: Option<Box<str>>,
    /// Native list description.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
    /// Stated source sampleCount when supplied, never computed from member IDs.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub sample_count: Option<u64>,
}
crate::wire_enum! {
    /// Native GitHub contents entry kind.
    StudyFileKind {
        /// Repository file; size counts blob bytes, including any LFS pointer.
        File => "file",
        /// Repository directory; no data download is implied.
        Directory => "dir"
    }
}
/// Publisher file reference; repository blob bytes never mean data-content bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StudyFile {
    /// Native repository entry name.
    pub name: Text,
    /// Exact native repository path.
    pub path: Text,
    /// Publisher browser link.
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub html_url: Text,
    /// Native download link, absent for directory entries.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub download_url: Option<Text>,
    /// Repository blob or tree revision, not a data-content checksum.
    pub sha: StudyBlobSha,
    /// GitHub size in repository blob bytes; LFS content size is unknown.
    pub repository_blob_size: u64,
    /// Native file or directory kind.
    pub kind: StudyFileKind,
}
