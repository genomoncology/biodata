//! Closed code vocabularies used by shared codings.
use crate::{AlleleOrigin, ClassificationType, ReviewStatus};

macro_rules! codes {
    (frozen $name:ident { $($variant:ident => $code:literal),+ $(,)? }) => {
        codes!(@define [Hash] $name { $($variant => $code),+ });
        impl $name {
            /// Canonical wire spelling from the closed vocabulary.
            pub const fn as_str(self) -> &'static str { self.code() }
        }
        impl std::str::FromStr for $name {
            type Err = crate::Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::from_code(value).ok_or(crate::Error::WireEnumUnknown)
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.as_str()) }
        }
    };

    ($name:ident { $($variant:ident => $code:literal),+ $(,)? }) => {
        codes!(@define [] $name { $($variant => $code),+ });
    };
    (@define [$($extra:path),*] $name:ident { $($variant:ident => $code:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq $(, $extra)*)]
        #[expect(missing_docs, reason = "each variant spells its documented code")]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name { $(#[cfg_attr(feature = "serde", serde(rename = $code))] $variant),+ }

        impl $name {
            /// Published code text.
            pub const fn code(self) -> &'static str {
                match self { $(Self::$variant => $code),+ }
            }

            pub(super) fn from_code(code: &str) -> Option<Self> {
                [$(Self::$variant),+].into_iter().find(|value| value.code() == code)
            }
        }
    };
}

codes!(frozen AcmgAmp2015Code {
    Pathogenic => "pathogenic", LikelyPathogenic => "likely_pathogenic",
    UncertainSignificance => "uncertain_significance", LikelyBenign => "likely_benign",
    Benign => "benign",
});
codes!(frozen ClingenLowPenetrance2024Code {
    PathogenicLowPenetrance => "pathogenic_low_penetrance",
    LikelyPathogenicLowPenetrance => "likely_pathogenic_low_penetrance",
    EstablishedRiskAllele => "established_risk_allele",
    LikelyRiskAllele => "likely_risk_allele",
    UncertainRiskAllele => "uncertain_risk_allele",
});
codes!(frozen AmpAscoCap2017TierCode {
    TierI => "tier_i", TierIi => "tier_ii", TierIii => "tier_iii", TierIv => "tier_iv",
});
codes!(frozen AmpAscoCap2017LevelCode {
    LevelA => "level_a", LevelB => "level_b", LevelC => "level_c", LevelD => "level_d",
});
codes!(frozen ClingenCgcVicc2022Code {
    Oncogenic => "oncogenic", LikelyOncogenic => "likely_oncogenic",
    UncertainSignificance => "uncertain_significance", LikelyBenign => "likely_benign",
    Benign => "benign",
});
codes!(frozen CivicEvidenceLevelCode { A => "a", B => "b", C => "c", D => "d", E => "e" });
codes!(frozen OncokbLevelCode {
    Level1 => "level_1", Level2 => "level_2", Level3a => "level_3a",
    Level3b => "level_3b", Level4 => "level_4", LevelR1 => "level_r1",
    LevelR2 => "level_r2", LevelDx1 => "level_dx1", LevelDx2 => "level_dx2",
    LevelDx3 => "level_dx3", LevelPx1 => "level_px1", LevelPx2 => "level_px2",
    LevelPx3 => "level_px3",
});
codes!(frozen OncokbOncogenicityCode {
    Oncogenic => "oncogenic", LikelyOncogenic => "likely_oncogenic",
    LikelyNeutral => "likely_neutral", Inconclusive => "inconclusive",
    Resistance => "resistance", Unknown => "unknown",
});
/// A ClinVar classification type that carries an actual classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClinvarClassificationCode(ClassificationType);

impl ClinvarClassificationCode {
    /// Germline disease classification.
    pub const GERMLINE: Self = Self(ClassificationType::Germline);
    /// Somatic clinical impact classification.
    pub const SOMATIC_CLINICAL_IMPACT: Self = Self(ClassificationType::Somatic);
    /// Oncogenicity classification.
    pub const ONCOGENICITY: Self = Self(ClassificationType::Oncogenicity);

    /// Published code text.
    pub const fn code(self) -> &'static str {
        match self.0 {
            ClassificationType::Germline => "germline",
            ClassificationType::Somatic => "somatic_clinical_impact",
            ClassificationType::Oncogenicity => "oncogenicity",
            ClassificationType::NoClassification => unreachable!(),
        }
    }

    pub(super) fn from_code(code: &str) -> Option<Self> {
        match code {
            "germline" => Some(Self::GERMLINE),
            "somatic_clinical_impact" => Some(Self::SOMATIC_CLINICAL_IMPACT),
            "oncogenicity" => Some(Self::ONCOGENICITY),
            _ => None,
        }
    }

    /// Existing ClinVar classification type.
    pub const fn classification_type(self) -> ClassificationType {
        self.0
    }
}
codes!(HpoInheritanceCode {
    AutosomalDominant => "HP:0000006", AutosomalRecessive => "HP:0000007",
    XLinked => "HP:0001417", Mitochondrial => "HP:0001427",
    Semidominant => "HP:0032113",
});
codes!(ClingenGeneValidity2017Code {
    Definitive => "definitive", Strong => "strong", Moderate => "moderate",
    Limited => "limited", Disputed => "disputed", Refuted => "refuted",
    NoKnownDiseaseRelationship => "no_known_disease_relationship",
});
codes!(ClingenDosageScoreCode {
    Zero => "0", One => "1", Two => "2", Three => "3",
    Thirty => "30", Forty => "40",
});

pub(super) fn review_code(value: ReviewStatus) -> &'static str {
    value.code()
}

pub(super) fn review_from_code(code: &str) -> Option<ReviewStatus> {
    ReviewStatus::ALL
        .into_iter()
        .find(|value| value.code() == code)
}

pub(super) fn origin_code(value: AlleleOrigin) -> &'static str {
    value.code()
}

pub(super) fn origin_from_code(code: &str) -> Option<AlleleOrigin> {
    AlleleOrigin::ALL
        .into_iter()
        .find(|value| value.code() == code)
}

impl TryFrom<ClassificationType> for ClinvarClassificationCode {
    type Error = super::CodingError;

    fn try_from(value: ClassificationType) -> Result<Self, Self::Error> {
        match value {
            ClassificationType::Germline => Ok(Self::GERMLINE),
            ClassificationType::Somatic => Ok(Self::SOMATIC_CLINICAL_IMPACT),
            ClassificationType::Oncogenicity => Ok(Self::ONCOGENICITY),
            ClassificationType::NoClassification => Err(super::CodingError),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ClinvarClassificationCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ClinvarClassificationCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_code(&<String as serde::Deserialize>::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom(super::CodingError))
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for ClinvarClassificationCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClinvarClassificationCode".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string","enum":["germline","somatic_clinical_impact","oncogenicity"]})
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for HpoInheritanceCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HpoInheritanceCode".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string", "enum":["HP:0000006", "HP:0000007", "HP:0001417", "HP:0001427", "HP:0032113"]})
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for ClingenGeneValidity2017Code {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClingenGeneValidity2017Code".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string", "enum":["definitive", "strong", "moderate", "limited", "disputed", "refuted", "no_known_disease_relationship"]})
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for ClingenDosageScoreCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClingenDosageScoreCode".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string", "enum":["0", "1", "2", "3", "30", "40"]})
    }
}
