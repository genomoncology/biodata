use super::SummarySource;
use crate::{PartialDate, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// A whole source-provided summary with attribution and stated date precision.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Summary {
    /// Entire summary text.
    pub text: Text,
    /// The summary's attribution, independent of the fetching provider.
    pub source: SummarySource,
    /// Date stated by the source, if any.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dated: Option<PartialDate>,
}
