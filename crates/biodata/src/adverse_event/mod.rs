//! Neutral spontaneous report records; codes and arrays carry no causal inference.
mod age;
mod faers;
pub use age::FaersAgeUnit;
mod maude;
pub use faers::{
    FaersDrug, FaersIdentifiers, FaersPatient, FaersReaction, FaersReport, FaersReporter,
    FaersSeriousness,
};
pub use maude::{MaudeDevice, MaudeNarrative, MaudeReport};
/// Opaque published digits with source carried separately, never a prefixed wrapper.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReportId(Box<str>);
impl ReportId {
    /// The original published spelling, including leading zeroes.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::str::FromStr for ReportId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(crate::Error::ReportIdInvalid);
        }
        Ok(Self(value.into()))
    }
}
impl std::fmt::Display for ReportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
crate::text_value!(
    ReportId,
    "ReportId",
    r"^[0-9]+$",
    "Opaque published digits with source carried separately, never a prefixed wrapper."
);
