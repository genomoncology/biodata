use crate::Text;
#[cfg(feature = "serde")]
use serde::Serialize;

crate::serde_object::object_record! {
    /// One source-native target identity, without inventing a gene label.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
    pub struct DrugNativeTarget {
        /// Source responsible for the native identifier.
        pub namespace: Text,
        /// Exact published target identifier.
        pub id: Text,
        /// Native target label, only when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Text>,
        /// Native target link, only when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
    }
}

crate::serde_object::object_record! {
    /// One native disease identifier and its own source label.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
    pub struct DrugDiseaseReference {
        /// Source namespace spelling, such as EFO or MeSH.
        pub namespace: Text,
        /// Exact identifier spelling, without CURIE conversion.
        pub id: Text,
        /// Label attached to this identifier by the source.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub label: Option<Text>,
    }
}

crate::serde_object::object_record! {
    /// One source development-stage field and its unchanged value.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugStage {
        /// Exact native field name.
        pub field: Box<str>,
        /// Exact source string, without an authorization interpretation.
        pub value: Box<str>,
    }
}

crate::serde_object::object_record! {
    /// Typed source reference shared by mechanism and indication records.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugNativeReference {
        /// Native reference type.
        pub ref_type: Text,
        /// Source reference identifier, when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub ref_id: Option<Text>,
        /// Source reference link, when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub ref_url: Option<Text>,
    }
}
