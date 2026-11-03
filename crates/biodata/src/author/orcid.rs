use super::{OrcidClientId, OrcidId};
use crate::{PartialDate, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// Only public source visibility is admitted; legacy uppercase is an alias.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = ["public", "PUBLIC"])))]
pub enum OrcidPublicVisibility {
    /// A public source assertion.
    #[cfg_attr(feature = "serde", serde(rename = "public", alias = "PUBLIC"))]
    Public,
}
crate::serde_object::object_record! {
    /// Native ORCID external identifier before any article pivot.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
    pub struct OrcidExternalId {
        /// Open native external-id-type.
        pub namespace: crate::IdentifierNamespace,
        /// Unchanged native identifier value.
        pub id: Text,
        /// Open native relationship, including self and part-of.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(with = "Text"))]
        pub relationship: Option<Text>,
        /// Native identifier URL.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
    }
}
crate::serde_object::object_record! {
    /// Source and assertion origin remain distinct provider claims.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct OrcidAssertionSource {
        /// Native source ORCID iD.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub source_orcid: Option<OrcidId>,
        /// Native ORCID client ID; no person identity.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub source_client_id: Option<OrcidClientId>,
        /// Native source display name.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub source_name: Option<Text>,
        /// Native assertion origin ORCID iD.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub assertion_origin_orcid: Option<OrcidId>,
        /// Native origin client ID.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub assertion_origin_client_id: Option<OrcidClientId>,
        /// Native origin display name.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub assertion_origin_name: Option<Text>,
    }
}
crate::serde_object::object_record! {
    /// One public summary, retained in native group order without representative selection.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct OrcidWorkAssertion {
        /// Source assertion identifier scoped to this ORCID record.
        pub put_code: std::num::NonZeroU64,
        /// Native title; some summaries have none.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub title: Option<Text>,
        /// Native subtitle.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub subtitle: Option<Text>,
        /// Native translated title.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub translated_title: Option<Text>,
        /// Open native work type.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        #[cfg_attr(feature = "schema", schemars(with = "Text"))]
        pub work_type: Option<Text>,
        /// Native partial publication date.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub publication_date: Option<PartialDate>,
        /// Native journal title.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub journal_title: Option<Text>,
        /// Native work URL.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Option<Text>,
        /// Every native assertion identifier in order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub external_ids: Box<[OrcidExternalId]>,
        /// Native source and origin provenance.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub source: Option<OrcidAssertionSource>,
        /// Only public assertions belong to this product.
        pub visibility: OrcidPublicVisibility,
        /// Native display index retained without sorting.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub display_index: Option<Text>,
    }
}
crate::serde_object::object_record! {
    /// One native work group is one group, regardless of assertion multiplicity.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct OrcidWorkGroup {
        /// Native group identifiers in source order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub external_ids: Box<[OrcidExternalId]>,
        /// Every public summary in native source order.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub assertions: Box<[OrcidWorkAssertion]>,
    }
}
impl std::str::FromStr for OrcidPublicVisibility {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "public" | "PUBLIC" => Ok(Self::Public),
            _ => Err(crate::Error::AuthorIdSyntax),
        }
    }
}
impl std::fmt::Display for OrcidPublicVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("public")
    }
}
