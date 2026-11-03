//! Source assertions without ranking, merging or clinical inference.
use crate::{PartialDate, Pmid, SourceGeneRef, Text};
#[cfg(feature = "serde")]
use serde::Serialize;

mod coding;
pub use coding::*;
mod profile;
pub use profile::{ProfileExpression, ProfileNode};
mod haplotype;
pub use haplotype::{ClinvarHaplotype, ClinvarHaplotypeMember};
mod reference;
pub use reference::{Curie, VariantRef};
crate::serde_object::object_record! {
    /// One source claim, kept in its own framework and context.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct Statement {
        /// Source method.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub method: Option<crate::Method>,
        /// Source inheritance mode.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub mode_of_inheritance: Option<Term>,
        /// Criteria assessed and not met.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub criteria_not_met: Box<[Text]>,
        /// Source record status.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub status: Option<Term>,
        /// Source-owned subject reference.
        pub subject: crate::Subject,
        /// Claim domain.
        pub proposition: Proposition,
        /// Source-stated relationship.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub predicate: Option<Predicate>,
        /// Source-stated assessment direction.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub direction: Option<Direction>,
        /// Whole source classification label.
        pub classification: Term,
        /// Source evidence level or review status.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub strength: Option<Term>,
        /// Source disease or tumor context.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub condition: Option<Concept>,
        /// Source therapies in their native order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub therapies: Box<[Concept]>,
        /// Source-stated origin.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub allele_origin: Option<Term>,
        /// Source gene context.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub gene_context: Option<SourceGeneRef>,
        /// Source criterion labels in source order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub criteria: Box<[Text]>,
        /// Source publications.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub citations: Box<[Pmid]>,
        /// One source record and its provenance.
        pub attribution: Attribution,
    }
}
crate::serde_object::object_record! {
    /// A source label with explicitly supported codings in source order.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct Term {
        /// Original label.
        pub label: Text,
        /// Documented framework codings.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub codings: Box<[Coding]>,
    }
}
crate::serde_object::object_record! {
    /// One source concept with its optional ontology identifier.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct Concept {
        /// Source-stated concept identifier.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub id: Option<Curie>,
        /// Original concept label.
        pub label: Text,
    }
}
crate::serde_object::object_record! {
    /// Provenance for one source record.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct Attribution {
        /// Source approval date.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub approved: Option<PartialDate>,
        /// Source publication date.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub published: Option<PartialDate>,
        /// Source responsible for the claim.
        pub source: Text,
        /// Native assertion record.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub record: Option<Text>,
        /// Record version.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub version: Option<Text>,
        /// Dataset release.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub release: Option<Text>,
        /// Source submitter or panel.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub submitter: Option<Text>,
        /// Source evaluation date.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub evaluated: Option<PartialDate>,
        /// Source record link.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
    }
}
crate::wire_enum! {
    /// Domain of a source claim.

    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    Proposition {
        /// Published pathogenicity value.
        Pathogenicity => "pathogenicity",
        /// Published oncogenicity value.
        Oncogenicity => "oncogenicity",
        /// Published clinical_significance value.
        ClinicalSignificance => "clinical_significance",
        /// Published therapeutic_response value.
        TherapeuticResponse => "therapeutic_response",
        /// Published diagnostic value.
        Diagnostic => "diagnostic",
        /// Published prognostic value.
        Prognostic => "prognostic",
        /// Published functional_impact value.
        FunctionalImpact => "functional_impact",
        /// Source gene disease validity.
        GeneDiseaseValidity => "gene_disease_validity",
        /// Source haploinsufficiency.
        Haploinsufficiency => "haploinsufficiency",
        /// Source triplosensitivity.
        Triplosensitivity => "triplosensitivity",
        /// Source gene role.
        GeneRole => "gene_role",
    }
}
crate::wire_enum! {
    /// Assessment direction stated by the source.

    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    Direction {
        /// Published supports value.
        Supports => "supports",
        /// Published disputes value.
        Disputes => "disputes",
        /// Published neutral value.
        Neutral => "neutral",
    }
}
crate::wire_enum! {
    /// VA-Spec 1.1 proposition relationships in native snake_case.

    #[cfg_attr(feature = "schema", schemars(extend("x-enum-open" = true)))]
    Predicate {
        /// Published is_causal_for value.
        IsCausalFor => "is_causal_for",
        /// Published is_oncogenic_for value.
        IsOncogenicFor => "is_oncogenic_for",
        /// Published has_clinical_significance_for value.
        HasClinicalSignificanceFor => "has_clinical_significance_for",
        /// Published predicts_sensitivity_to value.
        PredictsSensitivityTo => "predicts_sensitivity_to",
        /// Published predicts_resistance_to value.
        PredictsResistanceTo => "predicts_resistance_to",
        /// Published is_diagnostic_inclusion_criterion_for value.
        IsDiagnosticInclusionCriterionFor => "is_diagnostic_inclusion_criterion_for",
        /// Published is_diagnostic_exclusion_criterion_for value.
        IsDiagnosticExclusionCriterionFor => "is_diagnostic_exclusion_criterion_for",
        /// Published associated_with_better_outcome_for value.
        AssociatedWithBetterOutcomeFor => "associated_with_better_outcome_for",
        /// Published associated_with_worse_outcome_for value.
        AssociatedWithWorseOutcomeFor => "associated_with_worse_outcome_for",
        /// Published impacts_function_of value.
        ImpactsFunctionOf => "impacts_function_of",
    }
}

impl Statement {
    /// Build one explicit source claim with absent qualifiers and empty source lists.
    pub fn new(
        subject: crate::Subject,
        proposition: Proposition,
        classification: Term,
        attribution: Attribution,
    ) -> Self {
        Self {
            method: None,
            mode_of_inheritance: None,
            criteria_not_met: Box::default(),
            status: None,
            subject,
            proposition,
            predicate: None,
            direction: None,
            classification,
            strength: None,
            condition: None,
            therapies: Box::default(),
            allele_origin: None,
            gene_context: None,
            criteria: Box::default(),
            citations: Box::default(),
            attribution,
        }
    }
}

mod category;
pub use category::{Category, VariantRefError};
