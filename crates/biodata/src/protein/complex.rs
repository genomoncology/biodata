use crate::{ComplexPortalId, Curie, Text, UniProtIsoform};
#[cfg(feature = "serde")]
use serde::Serialize;
crate::serde_object::object_record! {
    /// Exact source complex header admitted by a native participant identity.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct ProteinComplexHeader {
        /// Native complex accession.
        pub id: ComplexPortalId,
        /// Native source name.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub name: Option<Text>,
        /// Source taxonomy identifier.
        pub taxon: u32,
        /// Literal native predictedComplex flag.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub predicted: Option<bool>,
        /// Actual participants in this exact complex, never a complex search total.
        pub participant_total: u64,
        /// Exact native participant identifier supporting admission.
        pub matched_participant: Text,
        /// Native complex record URL.
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Text,
    }
}
crate::serde_object::object_record! {
    /// A native participant; names never establish a protein identity.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct ProteinComplexMember {
        /// Native participant identifier, including non-protein identifiers.
        pub id: Text,
        /// Literal native participant type; non-protein members remain source facts.
        pub participant_type: Text,
        /// Native taxonomy identifier when supplied.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub taxon: Option<u32>,
        /// Literal supplied organism text, including non-taxonomic source sentinels.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub organism: Option<Text>,
        /// Native display name.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub name: Option<Text>,
        /// Native stoichiometry text; no numeric imputation.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub stoichiometry: Option<Text>,
        /// Source-supplied cross-references only.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub xrefs: Box<[Curie]>,
        /// Only an exact source-supplied protein identifier; never inferred from a name.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        pub protein: Option<UniProtIsoform>,
    }
}
