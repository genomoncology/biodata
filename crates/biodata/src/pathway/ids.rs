use crate::{Text, text_value};
#[cfg(feature = "serde")]
use serde::Serialize;
/// A checked human Reactome stable pathway identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReactomeId(Box<str>);
impl ReactomeId {
    /// Check the human namespace and canonical positive decimal identifier.
    pub fn new(value: &str) -> Result<Self, crate::Error> {
        let suffix = value
            .strip_prefix("R-HSA-")
            .ok_or(crate::Error::ReactomeIdInvalid)?;
        if suffix.is_empty()
            || suffix.starts_with('0')
            || !suffix.bytes().all(|ch| ch.is_ascii_digit())
        {
            return Err(crate::Error::ReactomeIdInvalid);
        }
        Ok(Self(value.into()))
    }
    /// Borrow the stable identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::str::FromStr for ReactomeId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}
impl std::fmt::Display for ReactomeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    ReactomeId,
    "ReactomeId",
    "^R-HSA-[1-9][0-9]*$",
    "A checked human Reactome stable pathway identifier."
);
crate::serde_object::object_record! {
    /// A pathway reference without membership or hierarchy data.
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct PathwayRef {
        /// Human stable identifier.
        pub id: ReactomeId,
        /// Whole source pathway name.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub name: Option<Text>,
    }
}

/// A checked WikiPathways identity without its revision.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WikiPathwaysId(Box<str>);
impl WikiPathwaysId {
    /// Check the canonical positive WP number.
    pub fn new(value: &str) -> Result<Self, crate::Error> {
        let suffix = value
            .strip_prefix("WP")
            .ok_or(crate::Error::WikiPathwaysIdInvalid)?;
        if suffix.is_empty()
            || suffix.starts_with('0')
            || !suffix.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(crate::Error::WikiPathwaysIdInvalid);
        }
        Ok(Self(value.into()))
    }
    /// Borrow the identity without a revision suffix.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::str::FromStr for WikiPathwaysId {
    type Err = crate::Error;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        Self::new(v)
    }
}
impl std::fmt::Display for WikiPathwaysId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    WikiPathwaysId,
    "WikiPathwaysId",
    "^WP[1-9][0-9]*$",
    "A checked WikiPathways identity without its revision."
);
crate::serde_object::object_enum! {
PathwayFields, "namespace", "id", "lowercase";
/// Source-owned pathway identity; equal names never merge sources.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "namespace", content = "id", rename_all = "lowercase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub enum PathwayId {
    /// Human Reactome stable identity without a version suffix.
    Reactome(ReactomeId),
    /// WikiPathways identity without a revision suffix.
    Wikipathways(WikiPathwaysId),
}
}

impl std::str::FromStr for PathwayId {
    type Err = crate::Error;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if v.starts_with("R-HSA-") {
            v.parse()
                .map(Self::Reactome)
                .map_err(|_| crate::Error::PathwayIdInvalid)
        } else {
            v.parse().map(Self::Wikipathways)
        }
    }
}
impl std::fmt::Display for PathwayId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reactome(v) => v.fmt(f),
            Self::Wikipathways(v) => v.fmt(f),
        }
    }
}
