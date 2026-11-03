use crate::{Attribution, Curie, FiniteNumber};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Exact Human Phenotype Ontology identifier with seven decimal digits.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HpId(Box<str>);
crate::text_value!(
    HpId,
    "HpId",
    "^HP:[0-9]{7}$",
    "Exact Human Phenotype Ontology identifier with seven decimal digits."
);
impl FromStr for HpId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() == 10
            && value
                .strip_prefix("HP:")
                .is_some_and(|s| s.bytes().all(|b| b.is_ascii_digit()))
        {
            Ok(Self(value.into()))
        } else {
            Err(crate::Error::HpIdInvalid)
        }
    }
}
impl fmt::Display for HpId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl HpId {
    /// Exact checked identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
crate::serde_object::object_record! {
    /// One native term search candidate, without a selected identity.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PhenotypeHeader {
        /// Source HP identity.
        pub id: HpId,
        /// Exact native label, including an empty value.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Box<str>>,
        /// Native search match score with no local interpretation.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub match_score: Option<FiniteNumber>,
        /// Native matched field name.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub matched_field: Option<Box<str>>,
        /// Native match type label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub match_type: Option<Box<str>>,
        /// Publisher identity and version when supplied.
        pub attribution: Attribution,
    }
}
/// Native source card, with independent HPO and Monarch metadata.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "source", rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PhenotypeCard {
    /// The direct HPO term answer.
    Hpo {
        /// Exact requested identifier.
        requested_id: HpId,
        /// Exact returned identifier.
        returned_id: HpId,
        /// Whole source label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        label: Option<Box<str>>,
        /// Complete definition; empty text remains present.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        definition: Option<Box<str>>,
        /// Complete native comment.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        comment: Option<Box<str>>,
        /// Source publication references, omitted when empty.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "crate::absent_or_empty")
        )]
        publication_references: Option<Box<[Curie]>>,
        /// HPO native descendant count.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        hpo_descendant_count: Option<u64>,
        /// Publisher provenance.
        attribution: Attribution,
    },
    /// The independent Monarch term answer.
    Monarch {
        /// Exact requested identifier.
        requested_id: HpId,
        /// Exact returned identifier.
        returned_id: HpId,
        /// Whole native label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        label: Option<Box<str>>,
        /// Complete native description.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        description: Option<Box<str>>,
        /// Native deprecation fact, when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        deprecated: Option<bool>,
        /// Monarch native descendant count, independent of HPO.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        monarch_descendant_count: Option<u64>,
        /// Native association counts with their original scopes.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "crate::absent_or_empty")
        )]
        association_counts: Option<Box<[PhenotypeAssociationCount]>>,
        /// Publisher provenance.
        attribution: Attribution,
    },
}
crate::serde_object::object_record! {
    /// Native collection counts, without summing scopes or counting patients.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PhenotypeAssociationCount {
        /// Native category key.
        pub key: Curie,
        /// Native category.
        pub category: Curie,
        /// Native scope label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Box<str>>,
        /// Native closure-expanded count.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub count: Option<u64>,
        /// Native direct count.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub count_direct: Option<u64>,
        /// Native count including orthologs, only when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub count_with_orthologs: Option<u64>,
    }
}
crate::serde_object::object_record! {
    /// One native synonym; HPO values remain unclassified.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PhenotypeSynonym {
        /// Whole source text.
        pub value: Box<str>,
        /// Native Monarch relationship class; absent for HPO.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub relationship: Option<PhenotypeSynonymClass>,
        /// Publisher provenance.
        pub attribution: Attribution,
    }
}
crate::wire_enum! { described
    /// Publisher synonym relationship class.
    PhenotypeSynonymClass {
        /// Native exact synonym.
        Exact => "exact",
        /// Native broad synonym.
        Broad => "broad",
        /// Native narrow synonym.
        Narrow => "narrow",
        /// Native related synonym.
        Related => "related",
    }
}
crate::serde_object::object_record! {
    /// One exact native cross reference, without equivalence promotion.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PhenotypeXref {
        /// Exact publisher reference.
        pub id: Curie,
        /// Native predicate when actually supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub predicate: Option<Curie>,
        /// Publisher provenance.
        pub attribution: Attribution,
    }
}
crate::serde_object::object_record! {
    /// One native direct hierarchy link, without traversal or inferred equivalence.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PhenotypeHierarchyLink {
        /// Exact related term identifier; native parents can be outside HP.
        pub id: Curie,
        /// Whole related term label.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Box<str>>,
        /// Original source relation, such as super_classes or sub_classes.
        pub relation: PhenotypeHierarchyRelation,
        /// Publisher provenance.
        pub attribution: Attribution,
    }
}

crate::wire_enum! { described
    /// Native hierarchy collection that supplies the link.
    PhenotypeHierarchyRelation {
        /// A direct parent in the native super_classes collection.
        Parent => "super_classes",
        /// A direct child in the native sub_classes collection.
        Child => "sub_classes",
    }
}
