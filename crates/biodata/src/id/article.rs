use crate::{Error, text_value};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU32,
    str::FromStr,
};
/// Canonical PMC identifier, with an explicit namespace.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pmcid(Pmid);
impl FromStr for Pmcid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        s.strip_prefix("PMC")
            .ok_or(Error::ArticleIdInvalid)?
            .parse()
            .map_err(|_| Error::ArticleIdInvalid)
            .map(Self)
    }
}
impl fmt::Display for Pmcid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PMC{}", self.0)
    }
}
text_value!(
    Pmcid,
    "Pmcid",
    format!("^PMC{}$", crate::schema_pattern::decimal(u32::MAX.into())),
    "Canonical PMC identifier, with an explicit namespace."
);
/// DOI preserving source spelling with ASCII-insensitive equality.
#[derive(Clone, Debug)]
pub struct Doi(Box<str>);
impl Doi {
    /// Borrow the unchanged source DOI spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for Doi {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let (registrant, suffix) = s
            .strip_prefix("10.")
            .and_then(|s| s.split_once('/'))
            .ok_or(Error::ArticleIdInvalid)?;
        if registrant.is_empty()
            || !registrant.bytes().all(|c| c.is_ascii_digit())
            || suffix.is_empty()
            || s.chars().any(char::is_whitespace)
            || s.chars().any(char::is_control)
        {
            return Err(Error::ArticleIdInvalid);
        }
        Ok(Self(s.into()))
    }
}
impl PartialEq for Doi {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}
impl Eq for Doi {}
impl Hash for Doi {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.to_ascii_lowercase().hash(h);
    }
}
impl fmt::Display for Doi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    Doi,
    "Doi",
    format!(
        r"^10\.[0-9]+/[^{}\x00-\x1f\x7f-\x9f]+$",
        crate::schema_pattern::WHITESPACE
    ),
    "DOI preserving source spelling with ASCII-insensitive equality."
);

/// Europe PMC source prefix followed by its positive numeric local identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EuropePmcId(Box<str>);
impl FromStr for EuropePmcId {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let prefix = value.bytes().take_while(u8::is_ascii_uppercase).count();
        if !(1..=8).contains(&prefix) || value.len() > 64 {
            return Err(Error::ArticleIdInvalid);
        }
        let digits = value.get(prefix..).ok_or(Error::ArticleIdInvalid)?;
        if digits.is_empty()
            || !digits.bytes().all(|b| b.is_ascii_digit())
            || digits.bytes().all(|b| b == b'0')
        {
            return Err(Error::ArticleIdInvalid);
        }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for EuropePmcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    EuropePmcId,
    "EuropePmcId",
    r"^(?=.{1,64}$)[A-Z]{1,8}[0-9]*[1-9][0-9]*$",
    "Europe PMC source prefix followed by its positive numeric local identifier."
);

/// Semantic Scholar's stable 40-character hexadecimal paper identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SemanticScholarId(Box<str>);
impl FromStr for SemanticScholarId {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        if value.len() != 40 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::ArticleIdInvalid);
        }
        Ok(Self(value.to_ascii_lowercase().into()))
    }
}
impl fmt::Display for SemanticScholarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    SemanticScholarId,
    "SemanticScholarId",
    r"^[A-Fa-f0-9]{40}$",
    "Semantic Scholar's stable 40-character hexadecimal paper identifier."
);

/// Positive PubMed identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pmid(NonZeroU32);
impl Pmid {
    /// Check the namespace-local number.
    pub fn new(value: u32) -> Result<Self, crate::SourceShapeError> {
        NonZeroU32::new(value)
            .map(Self)
            .ok_or(crate::SourceShapeError::ArticleIdInvalid)
    }
    /// Namespace-local number.
    pub fn get(self) -> u32 {
        self.0.get()
    }
}
impl FromStr for Pmid {
    type Err = crate::SourceShapeError;
    fn from_str(s: &str) -> Result<Self, crate::SourceShapeError> {
        if s.starts_with('0') || !s.bytes().all(|c| c.is_ascii_digit()) {
            return Err(crate::SourceShapeError::ArticleIdInvalid);
        }
        Self::new(
            s.parse()
                .map_err(|_| crate::SourceShapeError::ArticleIdInvalid)?,
        )
    }
}
impl fmt::Display for Pmid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Pmid {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Pmid {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
