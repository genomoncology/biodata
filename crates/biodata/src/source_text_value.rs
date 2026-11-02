//! Free source text keeps whitespace and empty values exactly as supplied.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Unchanged free source text, including empty values and surrounding whitespace.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(transparent))]
pub struct SourceText(Box<str>);

impl SourceText {
    /// Borrow the unchanged source value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl From<String> for SourceText {
    fn from(value: String) -> Self {
        Self(value.into_boxed_str())
    }
}
impl From<&str> for SourceText {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl std::fmt::Display for SourceText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
