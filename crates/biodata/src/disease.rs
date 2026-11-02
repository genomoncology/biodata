use crate::{Attribution, Curie, FiniteNumber, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// A native source association without clinical interpretation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseAssociation {
    /// The exact requested concept, including a queried parent.
    pub requested_id: Curie,
    /// Native returned subject.
    pub subject: DiseaseEndpoint,
    /// Native subject taxon, only as supplied by the publisher.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub subject_taxon: Option<Curie>,
    /// Native source predicate.
    pub predicate: Curie,
    /// Native returned object.
    pub object: DiseaseEndpoint,
    /// Original source subject before source normalization.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub original_subject: Option<Curie>,
    /// Original source object before source normalization.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub original_object: Option<Curie>,
    /// Native row provenance and version.
    pub attribution: Attribution,
    /// Native primary knowledge source.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub primary_knowledge_source: Option<Curie>,
    /// Native aggregator knowledge sources; missing, null and lists remain distinct.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            deserialize_with = "aggregators",
            skip_serializing_if = "omit_aggregators"
        )
    )]
    pub aggregator_knowledge_source: Option<Option<Box<[Curie]>>>,
    /// Native providing collection.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub provided_by: Option<Text>,
    /// Source evidence terms; unavailable evidence stays absent.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::absent_or_empty")
    )]
    pub evidence: Option<Box<[DiseaseAssociationReference]>>,
    /// Native qualifiers, including frequency or onset if supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub qualifiers: Box<[DiseaseAssociationQualifier]>,
    /// Native publication references, without inferred links or labels.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub publications: Box<[DiseaseAssociationReference]>,
    /// Native pre-normalization predicate, without interpretation.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub original_predicate: Option<Text>,
    /// Native association category.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub category: Option<Curie>,
    /// Native agent type.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub agent_type: Option<Text>,
    /// Native source knowledge level.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub knowledge_level: Option<Text>,
    /// Native evidence count, distinct from returned reference count.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub evidence_count: Option<u64>,
    /// Source negation; absent is not false.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub negated: Option<bool>,
    /// Native annotation count, without a computed denominator.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub has_count: Option<u64>,
    /// Native annotation total, without inferring an assayed denominator.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub has_total: Option<u64>,
    /// Native percentage, never computed from another field.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub has_percentage: Option<FiniteNumber>,
    /// Native quotient, never computed from another field.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub has_quotient: Option<FiniteNumber>,
    /// Native subject closure.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub subject_closure: Box<[Curie]>,
    /// Native object closure.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub object_closure: Box<[Curie]>,
}
/// A publisher relation with distinct endpoints and predicate.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseRelation {
    /// Unchanged native cross-reference description when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub description: Option<Box<str>>,
    /// Native subject.
    pub subject: DiseaseEndpoint,
    /// Publisher predicate, without equivalence promotion.
    pub predicate: Curie,
    /// Native object.
    pub object: DiseaseEndpoint,
    /// Publisher record and version.
    pub attribution: Attribution,
}
/// A publisher synonym with its scope and language.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseSynonym {
    /// Complete synonym text.
    pub value: Text,
    /// Publisher scope predicate.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub scope: Option<Curie>,
    /// Source language tag.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub language: Option<Text>,
    /// Publisher record and version.
    pub attribution: Attribution,
}
/// One source-native selector without a local interpretation.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StatisticSelector {
    /// Source field name.
    pub name: Text,
    /// Source selector code.
    pub code: Text,
    /// Publisher label when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<Text>,
}
/// One native statistics row with requested and returned dimensions.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseStatistic {
    /// Explicit selected native site ID.
    pub requested_site: Text,
    /// Settings sent to the publisher.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub requested_selectors: Box<[StatisticSelector]>,
    /// Settings actually returned, never filled from request.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub returned_selectors: Box<[StatisticSelector]>,
    /// Native annual row year.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub year: Option<u32>,
    /// Native measure name.
    pub measure: Text,
    /// Observed estimate; missing stays absent.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub observed: Option<StatisticEstimate>,
    /// Modeled estimate kept separately.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub modeled: Option<StatisticEstimate>,
    /// Publisher record and dataset version when supplied.
    pub attribution: Attribution,
}
/// Source estimate and uncertainty without computed quantities.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct StatisticEstimate {
    /// Native estimate.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub value: Option<crate::WithUnit<FiniteNumber>>,
    /// Native standard error.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub standard_error: Option<crate::WithUnit<FiniteNumber>>,
    /// Native lower confidence bound.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub lower_ci: Option<crate::WithUnit<FiniteNumber>>,
    /// Native upper confidence bound.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub upper_ci: Option<crate::WithUnit<FiniteNumber>>,
    /// Native count; no inferred denominator.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub count: Option<u64>,
}

/// One native endpoint with an identifier and an optional source label.
/// Native associations use this reference because a source may omit a display label.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseEndpoint {
    /// Exact source identifier.
    pub id: Curie,
    /// Source label when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<Text>,
}

#[cfg(feature = "serde")]
type Aggregators = Option<Option<Box<[Curie]>>>;
#[cfg(feature = "serde")]
fn omit_aggregators(value: &Aggregators) -> bool {
    value.is_none()
        || value
            .as_ref()
            .and_then(Option::as_ref)
            .is_some_and(|items| items.is_empty())
}
#[cfg(feature = "serde")]
fn aggregators<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Aggregators, D::Error> {
    Option::<Box<[Curie]>>::deserialize(d).map(Some)
}

/// One native association reference, with no invented display label or link.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseAssociationReference {
    /// Native evidence or publication identity.
    pub id: Curie,
    /// Native label when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<Text>,
    /// Unchanged native link when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    pub url: Option<Text>,
}
/// A named native qualifier without an inferred role.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DiseaseAssociationQualifier {
    /// Original native qualifier field.
    pub field: Text,
    /// Native qualifier identity.
    pub id: Curie,
    /// Native label when supplied.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub label: Option<Text>,
}
