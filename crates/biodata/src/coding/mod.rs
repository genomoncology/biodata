//! A coding carries one checked code from a named published vocabulary.
mod vocab;
pub use vocab::{
    AcmgAmp2015Code, AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode, CivicEvidenceLevelCode,
    ClingenCgcVicc2022Code, ClingenDosageScoreCode, ClingenGeneValidity2017Code,
    ClingenLowPenetrance2024Code, ClinvarClassificationCode, HpoInheritanceCode, OncokbLevelCode,
    OncokbOncogenicityCode,
};

use crate::{EditKind, SoTerm};
use std::fmt;

/// A checked Sequence Ontology consequence or edit class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoCode {
    /// Transcript consequence already recognized by this crate.
    Consequence(SoTerm),
    /// Nucleotide edit class already recognized by this crate.
    Edit(EditKind),
}

impl SoCode {
    /// Sequence Ontology accession.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Consequence(term) => term.code(),
            Self::Edit(kind) => kind.so_id(),
        }
    }

    fn from_code(code: &str) -> Option<Self> {
        SoTerm::ALL
            .into_iter()
            .find(|term| term.code() == code)
            .map(Self::Consequence)
            .or_else(|| {
                EditKind::ALL
                    .into_iter()
                    .find(|kind| kind.so_id() == code)
                    .map(Self::Edit)
            })
    }
}

/// A published code or coding system is unknown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodingError;

impl fmt::Display for CodingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("coding.value.invalid")
    }
}

impl std::error::Error for CodingError {}

impl crate::Coding {
    /// Exact wire system name.
    pub const fn system(self) -> &'static str {
        match self {
            Self::AcmgAmp2015(_) => "acmg_amp_2015",
            Self::ClingenLowPenetrance2024(_) => "clingen_low_penetrance_2024",
            Self::AmpAscoCap2017Tier(_) => "amp_asco_cap_2017_tier",
            Self::AmpAscoCap2017Level(_) => "amp_asco_cap_2017_level",
            Self::ClingenCgcVicc2022(_) => "clingen_cgc_vicc_2022",
            Self::ClinvarReviewStatus(_) => "clinvar_review_status",
            Self::CivicEvidenceLevel(_) => "civic_evidence_level",
            Self::OncokbLevel(_) => "oncokb_level",
            Self::OncokbOncogenicity(_) => "oncokb_oncogenicity",
            Self::ClinvarClassificationType(_) => "clinvar_classification_type",
            Self::ClinvarAlleleOrigin(_) => "clinvar_allele_origin",
            Self::So(_) => "so",
            Self::Hpo(_) => "hpo",
            Self::ClingenGeneValidity2017(_) => "clingen_gene_validity_2017",
            Self::ClingenDosageScore(_) => "clingen_dosage_score",
        }
    }

    /// Checked wire code in that system.
    pub fn code(self) -> &'static str {
        match self {
            Self::AcmgAmp2015(value) => value.code(),
            Self::ClingenLowPenetrance2024(value) => value.code(),
            Self::AmpAscoCap2017Tier(value) => value.code(),
            Self::AmpAscoCap2017Level(value) => value.code(),
            Self::ClingenCgcVicc2022(value) => value.code(),
            Self::ClinvarReviewStatus(value) => vocab::review_code(value),
            Self::CivicEvidenceLevel(value) => value.code(),
            Self::OncokbLevel(value) => value.code(),
            Self::OncokbOncogenicity(value) => value.code(),
            Self::ClinvarClassificationType(value) => value.code(),
            Self::ClinvarAlleleOrigin(value) => vocab::origin_code(value),
            Self::So(value) => value.code(),
            Self::Hpo(value) => value.code(),
            Self::ClingenGeneValidity2017(value) => value.code(),
            Self::ClingenDosageScore(value) => value.code(),
        }
    }

    /// Parse a code through its closed system vocabulary.
    pub fn from_system_code(system: &str, code: &str) -> Result<Self, CodingError> {
        let value = match system {
            "acmg_amp_2015" => {
                Self::AcmgAmp2015(AcmgAmp2015Code::from_code(code).ok_or(CodingError)?)
            }
            "clingen_low_penetrance_2024" => Self::ClingenLowPenetrance2024(
                ClingenLowPenetrance2024Code::from_code(code).ok_or(CodingError)?,
            ),
            "amp_asco_cap_2017_tier" => Self::AmpAscoCap2017Tier(
                AmpAscoCap2017TierCode::from_code(code).ok_or(CodingError)?,
            ),
            "amp_asco_cap_2017_level" => Self::AmpAscoCap2017Level(
                AmpAscoCap2017LevelCode::from_code(code).ok_or(CodingError)?,
            ),
            "clingen_cgc_vicc_2022" => Self::ClingenCgcVicc2022(
                ClingenCgcVicc2022Code::from_code(code).ok_or(CodingError)?,
            ),
            "clinvar_review_status" => {
                Self::ClinvarReviewStatus(vocab::review_from_code(code).ok_or(CodingError)?)
            }
            "civic_evidence_level" => Self::CivicEvidenceLevel(
                CivicEvidenceLevelCode::from_code(code).ok_or(CodingError)?,
            ),
            "oncokb_level" => {
                Self::OncokbLevel(OncokbLevelCode::from_code(code).ok_or(CodingError)?)
            }
            "oncokb_oncogenicity" => Self::OncokbOncogenicity(
                OncokbOncogenicityCode::from_code(code).ok_or(CodingError)?,
            ),
            "clinvar_classification_type" => Self::ClinvarClassificationType(
                ClinvarClassificationCode::from_code(code).ok_or(CodingError)?,
            ),
            "clinvar_allele_origin" => {
                Self::ClinvarAlleleOrigin(vocab::origin_from_code(code).ok_or(CodingError)?)
            }
            "so" => Self::So(SoCode::from_code(code).ok_or(CodingError)?),
            "hpo" => Self::Hpo(HpoInheritanceCode::from_code(code).ok_or(CodingError)?),
            "clingen_gene_validity_2017" => Self::ClingenGeneValidity2017(
                ClingenGeneValidity2017Code::from_code(code).ok_or(CodingError)?,
            ),
            "clingen_dosage_score" => Self::ClingenDosageScore(
                ClingenDosageScoreCode::from_code(code).ok_or(CodingError)?,
            ),
            _ => return Err(CodingError),
        };
        Ok(value)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SoCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SoCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_code(&<String as serde::Deserialize>::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom(CodingError))
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for SoCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SoCode".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string", "enum":SoTerm::ALL.into_iter().map(SoTerm::code).chain(EditKind::ALL.into_iter().map(EditKind::so_id)).collect::<Vec<_>>()})
    }
}
