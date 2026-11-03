use super::DrugId;
use crate::{Attribution, Text};
#[cfg(feature = "serde")]
use serde::Serialize;
crate::canonical_enum! {
    /// Identity level established by the source endpoint or record type, never inferred from ID shape.
    DrugIdentityLevel, DrugIdentityLevelUnknown {
        /// Source-stated ingredient identity.
        Ingredient => "ingredient",
        /// Source-stated salt identity.
        Salt => "salt",
        /// Source-stated clinical formulation identity.
        ClinicalFormulation => "clinical_formulation",
        /// Source-stated branded product identity.
        BrandedProduct => "branded_product",
        /// Source-stated spl set identity.
        SplSet => "spl_set",
        /// Source-stated spl version identity.
        SplVersion => "spl_version",
        /// Source-stated ndc package identity.
        NdcPackage => "ndc_package",
        /// Source-stated fda application identity.
        FdaApplication => "fda_application",
        /// Source-stated fda product identity.
        FdaProduct => "fda_product",
        /// Source-stated ema authorization identity.
        EmaAuthorization => "ema_authorization",
        /// Source-stated ema application identity.
        EmaApplication => "ema_application",
        /// Source-stated orphan designation identity.
        OrphanDesignation => "orphan_designation",
        /// Source-stated unknown identity.
        Unknown => "unknown",
    }
}
crate::serde_object::object_record! {
    /// One source record identity, without any equivalence or authorization claim.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
    pub struct DrugIdentity {
        /// Published native record identifier.
        pub id: DrugId,
        /// Source responsible for this identity.
        pub namespace: Text,
        /// Original source name.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(with = "Text"))]
        pub name: Option<Text>,
        /// Level established by the endpoint or record type; unknown stays unknown.
        pub identity_level: DrugIdentityLevel,
        /// Native record type, without local classification.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub r#type: Option<Text>,
        /// Native record URL.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
    }
}
crate::serde_object::object_record! {
    /// One explicitly source-stated identifier, parent, product or formulation relation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugIdentityRelation {
        /// Identity at the other end of the source relation.
        pub identity: DrugIdentity,
        /// Native relationship label; no inferred equivalence.
        pub relation: Text,
        /// Record supplying the relationship.
        pub attribution: Attribution,
    }
}
crate::serde_object::object_record! {
    /// A native product record and its explicit formulation links.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct DrugProduct {
        /// Source-owned product identity.
        pub identity: DrugIdentity,
        /// Only formulation relations the source states.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub formulations: Box<[DrugIdentityRelation]>,
        /// Native product provenance.
        pub attribution: Attribution,
    }
}
