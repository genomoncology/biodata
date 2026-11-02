//! Population construction refusal.

/// Invalid published population fact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrequencyError {
    /// Allele count exceeds allele number.
    CountExceedsNumber,
    /// A published fraction is nonfinite or outside zero to one.
    InvalidFraction,
    /// A mitochondrial field appears on a known non-mitochondrial allele.
    MitochondrialFieldMisplaced,
    /// A grpmax group appears on a named ancestry stratum.
    GrpmaxMisplaced,
}
impl FrequencyError {
    /// Stable shared refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::CountExceedsNumber => "population.count.exceeds_number",
            Self::InvalidFraction => "population.frequency.range",
            Self::MitochondrialFieldMisplaced => "population.mt_field.misplaced",
            Self::GrpmaxMisplaced => "population.grpmax.misplaced",
        }
    }
}
impl std::fmt::Display for FrequencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for FrequencyError {}
