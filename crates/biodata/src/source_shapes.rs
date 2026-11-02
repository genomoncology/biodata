//! Source assertions and measurements without clinical interpretation.
use crate::{
    Accession, AccessionBase, Attribution, Concept, GeneRef, GenomicInterval, Text, VariantRef,
};
#[cfg(feature = "serde")]
use crate::{GeneSymbol, HgncId, NcbiGeneId};
use std::fmt;

/// A variant, gene or build-qualified region named by a source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subject(SubjectFields);

#[derive(Clone, Debug, PartialEq, Eq)]
enum SubjectFields {
    /// One of the four variant reference levels.
    Variant(VariantRef),
    /// A source gene reference.
    Gene(GeneRef),
    /// A source region with explicit reference build.
    Region {
        /// Stated interval.
        interval: GenomicInterval,
        /// Source label.
        label: Text,
        /// Source region identifier.
        record: Option<Text>,
    },
}

/// A region subject needs an explicit reference build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubjectError;
impl SubjectError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        "coord.interval.build_absent"
    }
}
impl std::fmt::Display for SubjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for SubjectError {}

impl From<VariantRef> for Subject {
    fn from(value: VariantRef) -> Self {
        Self(SubjectFields::Variant(value))
    }
}
impl Subject {
    /// Keep a source gene reference as a subject.
    pub fn gene(gene: GeneRef) -> Self {
        Self(SubjectFields::Gene(gene))
    }
    /// Require the region's stated reference build.
    pub fn region(
        interval: GenomicInterval,
        label: Text,
        record: Option<Text>,
    ) -> Result<Self, SubjectError> {
        interval.build().map_err(|_| SubjectError)?;
        Ok(Self(SubjectFields::Region {
            interval,
            label,
            record,
        }))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct GeneSubjectWire {
    level: GeneLevel,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    hgnc_id: Option<HgncId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ncbi_gene_id: Option<NcbiGeneId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "schema",
        schemars(default, schema_with = "crate::transcript::symbol_gene_id_schema")
    )]
    ensembl_gene_id: Option<AccessionBase>,
    label: GeneSymbol,
}
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum GeneLevel {
    Gene,
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct RegionSubjectWire {
    level: RegionLevel,
    interval: GenomicInterval,
    label: Text,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    record: Option<Text>,
}
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum RegionLevel {
    Region,
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum SubjectWire {
    Variant(VariantRef),
    Gene(GeneSubjectWire),
    Region(RegionSubjectWire),
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Subject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let wire = match &self.0 {
            SubjectFields::Variant(value) => SubjectWire::Variant(value.clone()),
            SubjectFields::Gene(value) => SubjectWire::Gene(GeneSubjectWire {
                level: GeneLevel::Gene,
                hgnc_id: value.hgnc_id(),
                ncbi_gene_id: value.ncbi_gene_id(),
                ensembl_gene_id: value.ensembl_gene_id(),
                label: value.label().clone(),
            }),
            SubjectFields::Region {
                interval,
                label,
                record,
            } => SubjectWire::Region(RegionSubjectWire {
                level: RegionLevel::Region,
                interval: *interval,
                label: label.clone(),
                record: record.clone(),
            }),
        };
        wire.serialize(serializer)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Subject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[cfg(feature = "serde")]
        use serde::de::Error;
        match crate::serde_object::deserialize::<SubjectWire, D>(deserializer)? {
            SubjectWire::Variant(value) => Ok(Self::from(value)),
            SubjectWire::Gene(wire) => GeneRef::new(
                wire.hgnc_id,
                wire.ncbi_gene_id,
                wire.ensembl_gene_id,
                wire.label,
            )
            .map(Self::gene)
            .map_err(D::Error::custom),
            SubjectWire::Region(wire) => {
                Self::region(wire.interval, wire.label, wire.record).map_err(D::Error::custom)
            }
        }
    }
}

/// Source-stated method or guideline.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Method {
    /// Source method label.
    pub label: Text,
    /// Source method version.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<Text>,
}

/// A versioned or unversioned source transcript reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TranscriptId {
    /// Versioned accession.
    Versioned(Accession),
    /// Unversioned accession base.
    Base(AccessionBase),
}

/// A finite source score without interpretation.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "f64", into = "f64"))]
pub struct FiniteNumber(f64);
impl FiniteNumber {
    /// Refuse a NaN or infinity.
    pub fn new(value: f64) -> Result<Self, NumberError> {
        value.is_finite().then_some(Self(value)).ok_or(NumberError)
    }
    /// Source number.
    pub fn get(self) -> f64 {
        self.0
    }
}
impl TryFrom<f64> for FiniteNumber {
    type Error = NumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl From<FiniteNumber> for f64 {
    fn from(value: FiniteNumber) -> Self {
        value.0
    }
}
/// A source number was nonfinite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberError;
impl NumberError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        "number.value.non_finite"
    }
}
impl std::fmt::Display for NumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for NumberError {}

/// Units a source counts in a cohort.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CohortUnit {
    /// Count of samples.
    Samples,
    /// Count of patients.
    Patients,
    /// Count of tumors.
    Tumors,
}

/// Source-stated count for one condition.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct ConditionCount {
    /// Source condition.
    pub condition: Concept,
    /// Count of the source's selected units.
    pub count: u64,
    /// Source-stated count of units profiled at the locus.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub profiled_sample_count: Option<u64>,
}

/// One source cohort count without a derived denominator or frequency.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct CohortObservation {
    /// Source variant or category.
    pub subject: VariantRef,
    /// Dataset family.
    pub dataset: Text,
    /// Source dataset release.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dataset_version: Option<Text>,
    /// Kind of counted unit.
    pub unit: CohortUnit,
    /// Count of units with the subject.
    pub sample_count: u64,
    /// Source-stated units whose assay covered the locus.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub profiled_sample_count: Option<u64>,
    /// Source-stated cohort size, never used as a denominator here.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cohort_sample_count: Option<u64>,
    /// Source-stated condition counts in order.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub by_condition: Vec<ConditionCount>,
    /// Source record provenance.
    pub attribution: Attribution,
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for Subject {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Subject".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Register the unchanged canonical payload definitions before composing the tag union.
        let _ = <SubjectWire as schemars::JsonSchema>::json_schema(generator);
        let variant = <VariantRef as schemars::JsonSchema>::json_schema(generator);
        let mut schema = schemars::json_schema!({"oneOf": variant.get("oneOf")});
        if let Some(branches) = schema
            .get_mut("oneOf")
            .and_then(|value| value.as_array_mut())
        {
            for (mut branch, level) in [
                (
                    <GeneSubjectWire as schemars::JsonSchema>::json_schema(generator),
                    "gene",
                ),
                (
                    <RegionSubjectWire as schemars::JsonSchema>::json_schema(generator),
                    "region",
                ),
            ] {
                if let Some(properties) = branch
                    .get_mut("properties")
                    .and_then(|value| value.as_object_mut())
                {
                    properties.insert(
                        "level".into(),
                        schemars::json_schema!({"const": level, "type": "string"}).to_value(),
                    );
                }
                branches.push(branch.to_value());
            }
        }
        schema
    }
}

/// Rejection of a source concept or article identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceShapeError {
    /// Concept text lacks a prefix or local part.
    CurieInvalid,
    /// An article identifier is not a positive decimal integer.
    ArticleIdInvalid,
}
impl SourceShapeError {
    /// Stable code shared with the source value owner.
    pub const fn code(self) -> &'static str {
        match self {
            Self::CurieInvalid => "concept.curie.invalid",
            Self::ArticleIdInvalid => "id.article.invalid",
        }
    }
}
impl fmt::Display for SourceShapeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}
impl std::error::Error for SourceShapeError {}
