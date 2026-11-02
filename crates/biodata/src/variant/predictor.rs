//! Source-owned measurements and component provenance, without score interpretation.
use crate::{Attribution, FiniteNumber, Text, VariantRef};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// One unmodified source score, never a locally blended assessment.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PredictorScore {
    /// Source gene qualifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub gene: Option<crate::SourceGeneRef>,
    /// Source transcript qualifier.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub transcript: Option<crate::TranscriptId>,
    /// Exact source-qualified subject.
    pub subject: VariantRef,
    /// Published component field path.
    pub method: Text,
    /// Source number, including zero.
    pub score: FiniteNumber,
    /// Source label without code expansion or interpretation.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<Text>,
    /// Source-owned record and dataset release.
    pub attribution: Attribution,
}
