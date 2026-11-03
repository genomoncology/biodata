//! Source-qualified variant identity without disclosure policy.
use crate::{Allele, Build, Expression, SourceGeneRef, Text, VariantId};

crate::serde_object::object_record! {
    /// One source's own expression, assembly and optional source-stated allele.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct Representation {
        /// Source asserting this form.
        pub source: Text,
        /// Published notation kept unchanged.
        pub expression: Expression,
        /// Assembly stated by the source.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub build: Option<Build>,
        /// Source-supplied allele, without local equivalence inference.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub allele: Option<Allele>,
    }
}

crate::serde_object::object_record! {
    /// Variant entity with source-stated identities, forms and genes.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct Variant {
        /// All source identifiers, without inferred aliases.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub ids: Box<[VariantId]>,
        /// Every source-qualified representation.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub representations: Box<[Representation]>,
        /// Genes stated by the sources.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[_]>::is_empty")
        )]
        pub genes: Box<[SourceGeneRef]>,
    }
}
