//! Convert only source-stated alteration categories into shared references.
use crate::{
    Category, Coding, EditKind, GeneRef, Region, RegionKind, SoCode, Text, VariantRef,
    VariantRefError,
    alteration::{self, Alteration},
};

/// A parsed alteration does not supply a representable category or required context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlterationReferenceError {
    /// The source did not state a supported category.
    Unsupported,
    /// An exon ordinal cannot be paired without its source transcript total.
    RegionRequired,
    /// Supplied exon context disagrees with the parsed source scope.
    RegionMismatch,
    /// Categorical fields violate their checked shape.
    InvalidCategory(VariantRefError),
}

impl AlterationReferenceError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unsupported => "variant_ref.conversion.unsupported",
            Self::RegionRequired => "variant_ref.region.required",
            Self::RegionMismatch => "variant_ref.region.mismatch",
            Self::InvalidCategory(error) => error.code(),
        }
    }
}

impl std::fmt::Display for AlterationReferenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for AlterationReferenceError {}

impl VariantRef {
    /// Convert an explicitly stated category, retaining caller-supplied source text.
    /// Exon events require a matching region with a source-stated total.
    pub fn from_alteration(
        alteration: &Alteration,
        label: Text,
        region: Option<Region>,
    ) -> Result<Self, AlterationReferenceError> {
        let alteration::Kind::Gene {
            subject,
            change: alteration::Change::Event(event),
        } = &alteration.kind
        else {
            return Err(AlterationReferenceError::Unsupported);
        };
        let (category, region, edit) = match (&event.category, &event.scope) {
            (Some(alteration::Category::Amplification), None) => {
                (Category::Amplification, no_region(region)?, None)
            }
            (Some(alteration::Category::Deletion), None) => {
                (Category::Deletion, no_region(region)?, None)
            }
            (Some(alteration::Category::Fusion), None) => {
                (Category::Fusion, no_region(region)?, None)
            }
            (
                Some(alteration::Category::Deletion),
                Some(alteration::Scope::Exons { first, last: None }),
            ) => {
                let region = region.ok_or(AlterationReferenceError::RegionRequired)?;
                if region.kind() != RegionKind::Exon || region.number().get() != u32::from(*first) {
                    return Err(AlterationReferenceError::RegionMismatch);
                }
                (
                    Category::ExonMutation,
                    Some(region),
                    Some(Coding::So(SoCode::Edit(EditKind::Deletion))),
                )
            }
            _ => return Err(AlterationReferenceError::Unsupported),
        };
        if category == Category::Fusion && subject.as_str().contains('-') {
            return Err(AlterationReferenceError::Unsupported);
        }
        let gene = GeneRef::new(None, None, None, subject.clone())
            .map_err(|_| AlterationReferenceError::Unsupported)?;
        Self::categorical(
            crate::SourceGeneRef::try_from(&gene)
                .map_err(|_| AlterationReferenceError::Unsupported)?,
            category,
            None,
            region,
            edit,
            None,
            label,
        )
        .map_err(AlterationReferenceError::InvalidCategory)
    }
}

fn no_region(region: Option<Region>) -> Result<Option<Region>, AlterationReferenceError> {
    if region.is_some() {
        Err(AlterationReferenceError::RegionMismatch)
    } else {
        Ok(None)
    }
}
