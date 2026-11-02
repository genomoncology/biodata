//! Source-stated copy counts and changes on uncertain locations.
use crate::{Bound, UncertainLocation};

/// An absolute copy count for a source-stated sequence location.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct CopyNumberCount {
    /// The sequence and possibly uncertain span being counted.
    pub location: UncertainLocation,
    /// Absolute number of copies, possibly bounded.
    pub copies: Bound,
}

/// Relative copy change, using the VRS CopyChange EFO terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CopyChange {
    /// Entire genome is absent.
    CompleteGenomicLoss,
    /// Source calls a high-level loss.
    HighLevelLoss,
    /// Source calls a low-level loss.
    LowLevelLoss,
    /// Source calls a loss without a level.
    Loss,
    /// Source states the region's base ploidy.
    RegionalBasePloidy,
    /// Source calls a gain without a level.
    Gain,
    /// Source calls a low-level gain.
    LowLevelGain,
    /// Source calls a high-level gain.
    HighLevelGain,
}

impl CopyChange {
    /// Exact EFO accession named by the VRS CopyChange vocabulary.
    pub const fn efo_code(self) -> &'static str {
        match self {
            Self::CompleteGenomicLoss => "EFO:0030069",
            Self::HighLevelLoss => "EFO:0020073",
            Self::LowLevelLoss => "EFO:0030068",
            Self::Loss => "EFO:0030067",
            Self::RegionalBasePloidy => "EFO:0030064",
            Self::Gain => "EFO:0030070",
            Self::LowLevelGain => "EFO:0030071",
            Self::HighLevelGain => "EFO:0030072",
        }
    }
}

/// A copy change relative to baseline ploidy, as the source states it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct CopyNumberChange {
    /// The sequence and possibly uncertain span.
    pub location: UncertainLocation,
    /// Source-stated relative change.
    pub copy_change: CopyChange,
}
