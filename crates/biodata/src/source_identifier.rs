use crate::Text;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

crate::serde_object::object_record! {
    /// An authority and its unchanged native identifier; unknown means the authority was not stated.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(rename = "Identifier_{T}", extend("x-identifier" = true)))]
    pub struct Identifier<T> {
        /// Native authority spelling; the literal unknown carries an unstated authority.
        pub namespace: IdentifierNamespace,
        /// Native identifier value, with its original scalar type and precision.
        pub id: T,
    }
}

impl<T> Identifier<T> {
    /// Pair a native value with a stated authority without parsing or rewriting the value.
    pub fn new(namespace: Text, id: T) -> Self {
        Self {
            namespace: namespace.into(),
            id,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.namespace.as_deref() {
            Some(namespace) => write!(f, "{namespace}:{}", self.id),
            None => write!(f, "unknown:{}", self.id),
        }
    }
}

/// A source-stated authority or an explicit unknown authority, represented by the string unknown.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Text"))]
pub enum IdentifierNamespace {
    /// The complete authority spelling supplied for this identifier.
    Stated(Text),
    /// The source supplied no authority; serialize unknown rather than omit the property.
    Unknown,
}
impl IdentifierNamespace {
    /// Borrow the native authority when one was stated.
    pub fn as_deref(&self) -> Option<&str> {
        match self {
            Self::Stated(value) => Some(value.as_str()),
            Self::Unknown => None,
        }
    }
}
impl From<Text> for IdentifierNamespace {
    fn from(value: Text) -> Self {
        if value.as_str() == "unknown" {
            Self::Unknown
        } else {
            Self::Stated(value)
        }
    }
}
impl From<Option<Text>> for IdentifierNamespace {
    fn from(value: Option<Text>) -> Self {
        value.map_or(Self::Unknown, Self::from)
    }
}

#[cfg(feature = "serde")]
impl Serialize for IdentifierNamespace {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_deref().unwrap_or("unknown"))
    }
}
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for IdentifierNamespace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Text::deserialize(deserializer).map(Self::from)
    }
}
