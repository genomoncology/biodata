//! Source categorical variant details.
use std::fmt;

/// A source gene-level category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(transform = crate::schema_compat::closed_string_enum))]
pub enum Category {
    /// A stated gene amplification.
    Amplification,
    /// A stated gene deletion.
    Deletion,
    /// A stated gene fusion.
    Fusion,
    /// An exon-scoped molecular change.
    ExonMutation,
    /// A source category of oncogenic mutations, without classifying a member.
    OncogenicMutation,
    /// Every variant at a stated protein site.
    Residue,
}

/// A source reference has missing identity or misplaced category detail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariantRefError {
    /// A leaf has neither source ID nor label.
    IdentityEmpty,
    /// A profile contains another profile or category.
    MemberInvalid,
    /// A partner appears outside fusion.
    PartnerMisplaced,
    /// Region or edit appears outside exon mutation, or edit is not SO.
    RegionMisplaced,
    /// A protein site appears outside residue category or is missing there.
    SiteMisplaced,
    /// The category's site names another gene.
    SiteGeneMismatch,
}

impl VariantRefError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::IdentityEmpty => "variant_ref.identity.empty",
            Self::MemberInvalid => "variant_ref.member.invalid",
            Self::PartnerMisplaced => "variant_ref.partner.misplaced",
            Self::RegionMisplaced => "variant_ref.region.misplaced",
            Self::SiteMisplaced => "variant_ref.site.misplaced",
            Self::SiteGeneMismatch => "variant_ref.site.gene_mismatch",
        }
    }
}

impl fmt::Display for VariantRefError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for VariantRefError {}
