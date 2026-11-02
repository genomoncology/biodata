use crate::{AlleleOrigin, ReviewStatus};
#[cfg(feature = "serde")]
use serde::Serialize;
crate::serde_object::object_enum! {
CodingFields, "system", "code", "snake_case";
/// A code in one explicitly named source framework.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "system", content = "code"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Coding {
    /// ClinVar publisher review status.
    #[cfg_attr(feature = "serde", serde(rename = "clinvar_review_status"))]
    ClinvarReviewStatus(ReviewStatus),
    /// ClinVar publisher allele origin.
    #[cfg_attr(feature = "serde", serde(rename = "clinvar_allele_origin"))]
    ClinvarAlleleOrigin(AlleleOrigin),
    /// OncoKB oncogenicity vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "oncokb_oncogenicity"))]
    OncokbOncogenicity(OncokbOncogenicityCode),
    /// OncoKB evidence levels.
    #[cfg_attr(feature = "serde", serde(rename = "oncokb_level"))]
    OncokbLevel(OncokbLevelCode),
    /// CIViC evidence level.
    #[cfg_attr(feature = "serde", serde(rename = "civic_evidence_level"))]
    CivicEvidenceLevel(CivicEvidenceLevelCode),
    /// acmg_amp_2015 vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "acmg_amp_2015"))]
    AcmgAmp2015(AcmgAmp2015Code),
    /// clingen_low_penetrance_2024 vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "clingen_low_penetrance_2024"))]
    ClingenLowPenetrance2024(ClingenLowPenetrance2024Code),
    /// amp_asco_cap_2017_tier vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "amp_asco_cap_2017_tier"))]
    AmpAscoCap2017Tier(AmpAscoCap2017TierCode),
    /// amp_asco_cap_2017_level vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "amp_asco_cap_2017_level"))]
    AmpAscoCap2017Level(AmpAscoCap2017LevelCode),
    /// clingen_cgc_vicc_2022 vocabulary.
    #[cfg_attr(feature = "serde", serde(rename = "clingen_cgc_vicc_2022"))]
    ClingenCgcVicc2022(ClingenCgcVicc2022Code),
    /// ClinVar classification type.
    #[cfg_attr(feature = "serde", serde(rename = "clinvar_classification_type"))]
    ClinvarClassificationType(crate::ClinvarClassificationCode),
    /// Sequence Ontology consequence or edit.
    #[cfg_attr(feature = "serde", serde(rename = "so"))]
    So(crate::SoCode),
    /// HPO mode of inheritance.
    #[cfg_attr(feature = "serde", serde(rename = "hpo"))]
    Hpo(crate::HpoInheritanceCode),
    /// ClinGen gene validity.
    #[cfg_attr(feature = "serde", serde(rename = "clingen_gene_validity_2017"))]
    ClingenGeneValidity2017(crate::ClingenGeneValidity2017Code),
    /// ClinGen dosage score.
    #[cfg_attr(feature = "serde", serde(rename = "clingen_dosage_score"))]
    ClingenDosageScore(crate::ClingenDosageScoreCode),
}
}

pub use crate::coding::{
    AcmgAmp2015Code, AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode, CivicEvidenceLevelCode,
    ClingenCgcVicc2022Code, ClingenLowPenetrance2024Code, OncokbLevelCode, OncokbOncogenicityCode,
};
