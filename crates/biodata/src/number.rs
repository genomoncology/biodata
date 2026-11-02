#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A finite source fraction in the closed interval zero to one.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "f64", into = "f64"))]
pub struct Fraction(f64);
impl Fraction {
    const MIN: f64 = 0.0;
    const MAX: f64 = 1.0;
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for Fraction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Fraction".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"number", "minimum":Self::MIN, "maximum":Self::MAX})
    }
}
impl TryFrom<f64> for Fraction {
    type Error = crate::Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() && (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::Error::PopulationFrequencyRange)
        }
    }
}
impl From<Fraction> for f64 {
    fn from(value: Fraction) -> Self {
        value.0
    }
}

/// A source number with its explicitly stated unit; missing units stay absent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "WithUnit_{V}_{U}"))]
pub struct WithUnit<V, U = crate::Text> {
    /// Unchanged numeric source value.
    pub value: V,
    /// Publisher unit or code, without conversion or inference.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unit: Option<U>,
}
