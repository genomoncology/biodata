//! ClinVar publisher vocabularies shared by aggregate and submission records.
use std::str::FromStr;

/// A text value is outside a ClinVar publisher vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClinVarValueError {
    /// The text is not a review status.
    ReviewStatus,
    /// The text is not an allele origin.
    AlleleOrigin,
}
impl ClinVarValueError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::ReviewStatus => "clinvar.review_status.invalid",
            Self::AlleleOrigin => "clinvar.allele_origin.invalid",
        }
    }
}
impl std::fmt::Display for ClinVarValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for ClinVarValueError {}

/// The kind of classification reported by ClinVar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ClassificationType {
    /// Germline disease classification.
    Germline,
    /// Somatic clinical impact classification.
    Somatic,
    /// Oncogenicity classification.
    Oncogenicity,
    /// Publisher states that no classification was supplied.
    NoClassification,
}

/// Publisher review status for an aggregate or submitted classification.
///
/// Source: https://www.ncbi.nlm.nih.gov/clinvar/docs/review_status/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReviewStatus {
    /// A practice guideline supplies the classification.
    PracticeGuideline,
    /// An expert panel reviewed the classification.
    ExpertPanel,
    /// Multiple submitters supplied criteria and agree.
    CriteriaMultipleSubmittersNoConflicts,
    /// Multiple submitters supplied criteria but conflict.
    CriteriaConflictingClassifications,
    /// Multiple submitters supplied criteria; consensus is not evaluated.
    CriteriaMultipleSubmitters,
    /// One submitter supplied criteria.
    CriteriaSingleSubmitter,
    /// No assertion criteria were supplied.
    NoAssertionCriteria,
    /// No classification was supplied.
    NoClassification,
    /// No classification directly names this individual variant.
    NoClassificationForIndividualVariant,
}
impl ReviewStatus {
    pub(crate) const ALL: [Self; 9] = [
        Self::PracticeGuideline,
        Self::ExpertPanel,
        Self::CriteriaMultipleSubmittersNoConflicts,
        Self::CriteriaConflictingClassifications,
        Self::CriteriaMultipleSubmitters,
        Self::CriteriaSingleSubmitter,
        Self::NoAssertionCriteria,
        Self::NoClassification,
        Self::NoClassificationForIndividualVariant,
    ];

    /// Snake-case code used in a shared Coding value.
    pub const fn code(self) -> &'static str {
        match self {
            Self::PracticeGuideline => "practice_guideline",
            Self::ExpertPanel => "expert_panel",
            Self::CriteriaMultipleSubmittersNoConflicts => {
                "criteria_provided_multiple_submitters_no_conflicts"
            }
            Self::CriteriaConflictingClassifications => {
                "criteria_provided_conflicting_classifications"
            }
            Self::CriteriaMultipleSubmitters => "criteria_provided_multiple_submitters",
            Self::CriteriaSingleSubmitter => "criteria_provided_single_submitter",
            Self::NoAssertionCriteria => "no_assertion_criteria_provided",
            Self::NoClassification => "no_classification_provided",
            Self::NoClassificationForIndividualVariant => {
                "no_classification_for_the_individual_variant"
            }
        }
    }

    /// Exact publisher text.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PracticeGuideline => "practice guideline",
            Self::ExpertPanel => "reviewed by expert panel",
            Self::CriteriaMultipleSubmittersNoConflicts => {
                "criteria provided, multiple submitters, no conflicts"
            }
            Self::CriteriaConflictingClassifications => {
                "criteria provided, conflicting classifications"
            }
            Self::CriteriaMultipleSubmitters => "criteria provided, multiple submitters",
            Self::CriteriaSingleSubmitter => "criteria provided, single submitter",
            Self::NoAssertionCriteria => "no assertion criteria provided",
            Self::NoClassification => "no classification provided",
            Self::NoClassificationForIndividualVariant => {
                "no classification for the individual variant"
            }
        }
    }
}
impl std::fmt::Display for ReviewStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
impl FromStr for ReviewStatus {
    type Err = ClinVarValueError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|status| status.as_str() == text)
            .ok_or(ClinVarValueError::ReviewStatus)
    }
}

/// The submitted allele origin term, without inferred germline grouping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlleleOrigin {
    /// Germline origin.
    Germline,
    /// Somatic origin.
    Somatic,
    /// De novo origin.
    DeNovo,
    /// Inherited origin.
    Inherited,
    /// Maternal origin.
    Maternal,
    /// Paternal origin.
    Paternal,
    /// Biparental origin.
    Biparental,
    /// Uniparental origin.
    Uniparental,
    /// The submitter did not provide an origin.
    NotProvided,
    /// Unknown origin.
    Unknown,
}
impl AlleleOrigin {
    pub(crate) const ALL: [Self; 10] = [
        Self::Germline,
        Self::Somatic,
        Self::DeNovo,
        Self::Inherited,
        Self::Maternal,
        Self::Paternal,
        Self::Biparental,
        Self::Uniparental,
        Self::NotProvided,
        Self::Unknown,
    ];

    /// Snake-case code used in a shared Coding value.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Germline => "germline",
            Self::Somatic => "somatic",
            Self::DeNovo => "de_novo",
            Self::Inherited => "inherited",
            Self::Maternal => "maternal",
            Self::Paternal => "paternal",
            Self::Biparental => "biparental",
            Self::Uniparental => "uniparental",
            Self::NotProvided => "not_provided",
            Self::Unknown => "unknown",
        }
    }

    /// Exact ClinVar origin term.
    /// Source: https://www.ncbi.nlm.nih.gov/clinvar/docs/properties/
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Germline => "germline",
            Self::Somatic => "somatic",
            Self::DeNovo => "de novo",
            Self::Inherited => "inherited",
            Self::Maternal => "maternal",
            Self::Paternal => "paternal",
            Self::Biparental => "biparental",
            Self::Uniparental => "uniparental",
            Self::NotProvided => "not provided",
            Self::Unknown => "unknown",
        }
    }
}
impl std::fmt::Display for AlleleOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
impl FromStr for AlleleOrigin {
    type Err = ClinVarValueError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|origin| origin.as_str() == text)
            .ok_or(ClinVarValueError::AlleleOrigin)
    }
}

impl ReviewStatus {
    /// Parse an exact source wire code without interpreting the publisher label.
    pub fn from_code(value: &str) -> Result<Self, ClinVarValueError> {
        Self::ALL
            .into_iter()
            .find(|item| item.code() == value)
            .ok_or(ClinVarValueError::ReviewStatus)
    }
    /// Read an exact publisher label when known.
    pub fn from_source_label(value: &str) -> Option<Self> {
        value.parse().ok()
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ReviewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ReviewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_code(&<String as serde::Deserialize>::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for ReviewStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClinvarReviewStatusCode".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string", "description":"Checked ClinVar review status codes.", "enum":["practice_guideline","expert_panel","criteria_provided_multiple_submitters_no_conflicts","criteria_provided_conflicting_classifications","criteria_provided_multiple_submitters","criteria_provided_single_submitter","no_assertion_criteria_provided","no_classification_provided","no_classification_for_the_individual_variant"]})
    }
}

impl AlleleOrigin {
    /// Parse an exact source wire code without interpreting the publisher label.
    pub fn from_code(value: &str) -> Result<Self, ClinVarValueError> {
        Self::ALL
            .into_iter()
            .find(|item| item.code() == value)
            .ok_or(ClinVarValueError::AlleleOrigin)
    }
    /// Read an exact publisher label when known.
    pub fn from_source_label(value: &str) -> Option<Self> {
        value.parse().ok()
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AlleleOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AlleleOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_code(&<String as serde::Deserialize>::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for AlleleOrigin {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClinvarAlleleOriginCode".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string", "description":"Checked ClinVar allele origin codes.", "enum":["germline","somatic","de_novo","inherited","maternal","paternal","biparental","uniparental","not_provided","unknown"]})
    }
}
