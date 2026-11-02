use crate::Text;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt, str::FromStr};

/// Exact normalized scientific p value; no floating-point conversion is used.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Parts"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Parts"))]
pub struct ScientificPValue(Parts);
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Parts {
    /// Source mantissa spelling, including significant trailing zeros.
    #[cfg_attr(feature = "schema", schemars(with = "String", extend("pattern" = r"^[1-9](\.[0-9]+)?$")))]
    mantissa: Text,
    /// Source base-ten exponent; values below the double range remain exact.
    #[cfg_attr(feature = "schema", schemars(range(max = 0)))]
    exponent: i64,
}
impl TryFrom<Parts> for ScientificPValue {
    type Error = crate::Error;
    fn try_from(parts: Parts) -> Result<Self, Self::Error> {
        let text = parts.mantissa.as_str();
        let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
        if whole.len() != 1
            || !whole.bytes().all(|b| (b'1'..=b'9').contains(&b))
            || !fraction.bytes().all(|b| b.is_ascii_digit())
            || text.ends_with('.')
            || parts.exponent > 0
            || parts.exponent == 0 && (whole != "1" || fraction.bytes().any(|b| b != b'0'))
        {
            return Err(crate::Error::GwasValueInvalid);
        }
        Ok(Self(parts))
    }
}
impl ScientificPValue {
    fn identity_mantissa(&self) -> &str {
        self.mantissa().trim_end_matches('0').trim_end_matches('.')
    }

    /// Unchanged decimal mantissa.
    pub fn mantissa(&self) -> &str {
        self.0.mantissa.as_str()
    }
    /// Base-ten exponent, without a double-range limit.
    pub fn exponent(&self) -> i64 {
        self.0.exponent
    }
}
impl FromStr for ScientificPValue {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (mantissa, exponent) = value
            .split_once(['e', 'E'])
            .ok_or(crate::Error::GwasValueInvalid)?;
        Self::try_from(Parts {
            mantissa: Text::new(mantissa).map_err(|_| crate::Error::GwasValueInvalid)?,
            exponent: exponent
                .parse()
                .map_err(|_| crate::Error::GwasValueInvalid)?,
        })
    }
}
impl fmt::Display for ScientificPValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}e{}", self.mantissa(), self.exponent())
    }
}
impl Ord for ScientificPValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.exponent()
            .cmp(&other.exponent())
            .then_with(|| self.identity_mantissa().cmp(other.identity_mantissa()))
    }
}
impl PartialOrd for ScientificPValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ScientificPValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for ScientificPValue {}

impl std::hash::Hash for ScientificPValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.exponent().hash(state);
        self.identity_mantissa().hash(state);
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(ScientificPValue, Parts);
