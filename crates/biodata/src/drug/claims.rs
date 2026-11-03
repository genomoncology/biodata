use super::{DrugDiseaseReference, DrugNativeReference, DrugStage};
use super::{DrugId, DrugIdentity};
use crate::{Attribution, PartialDate, Text};
#[cfg(feature = "serde")]
use serde::Serialize;
crate::serde_object::object_record! {
    /// One independently identified label set and revision.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugLabelReference {
        /// Published SPL set or native label record ID.
        pub id: DrugId,
        /// Native label revision.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub version: Option<Text>,
        /// Source-stated revision date.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub revised: Option<PartialDate>,
        /// Native label URL.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
        /// Label provenance.
        pub attribution: Attribution,
    }
}
crate::serde_object::object_record! {
    /// Original label text under one exact native heading.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugLabelSection {
        /// Label identity and revision.
        pub label: DrugLabelReference,
        /// Exact native field or heading.
        pub heading: Text,
        /// Original text, preserving whitespace; consumer paging never summarizes it.
        pub text: Box<str>,
    }
}
crate::serde_object::object_record! {
    /// A source indication retaining its full population and combination wording.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugIndicationClaim {
        /// Source drug identity.
        pub drug: DrugIdentity,
        /// Complete native indication wording.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub indication: Option<Text>,
        /// Independently labeled native disease identifiers, without cross-source normalization.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub disease_references: Box<[DrugDiseaseReference]>,
        /// Source-stated development stage, separate from authorization events.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub stage: Option<DrugStage>,
        /// Source-native supporting references.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub references: Box<[DrugNativeReference]>,
        /// Population wording explicitly supplied separately.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub population: Option<Text>,
        /// Complete source-stated combination members, without local joins.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub combination: Box<[DrugIdentity]>,
        /// Native claim provenance.
        pub attribution: Attribution,
    }
}
