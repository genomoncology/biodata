use super::{ProteinCoordinateConvention, ProteinIsoformStatus, ProteinSequenceAvailability};
use crate::UniProtFeature;
use crate::{Attribution, Curie, Text, UniProtAcc, UniProtIsoform};
#[cfg(feature = "serde")]
use serde::Serialize;
crate::serde_object::object_record! {
    /// A reviewed human protein listed by the authoritative gene record.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    #[cfg_attr(feature = "schema", schemars(rename = "Protein_{A}"))]
    pub struct Protein<A = UniProtAcc> {
        /// Checked primary accession.
        pub accession: A,
        /// Recommended protein name.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub name: Option<Text>,
        /// Sequence length in amino acids.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub length: Option<u32>,
        /// Whole source function paragraphs, omitted when absent.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub function: Box<[Text]>,
        /// Source subcellular location names.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub subcellular_locations: Box<[Text]>,
    }
}

crate::serde_object::object_record! {
    /// Native entry metadata wrapping the same Protein identity used by the gene slice.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct ProteinHeader {
        /// Shared gene/protein identity fields; never independently restated.
        pub protein: Protein<UniProtIsoform>,
        /// Native source entry name.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub entry_id: Option<Text>,
        /// Display names only; never identifier joins.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub gene_names: Box<[Text]>,
        /// Only identifiers explicitly supplied by the source.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub gene_ids: Box<[Curie]>,
        /// Native source taxonomy identifier.
        pub taxon: u32,
        /// Native reviewed status.
        pub reviewed: bool,
        /// Source entry URL.
        #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
        pub url: Text,
    }
}
crate::serde_object::object_record! {
    /// UniProt feature with source attribution and coordinate provenance.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct ProteinFeature {
        /// Shared feature, including native label and residues.
        pub feature: UniProtFeature,
        /// Source record and release facts.
        pub attribution: Attribution,
        /// Convention before mathematical conversion.
        pub source_coordinates: ProteinCoordinateConvention,
    }
}
crate::serde_object::object_record! {
    /// One source alternative-product row; all stated identifiers stay together.
    #[derive(Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
    pub struct ProteinIsoform {
        /// All source-stated isoform identifiers, including External base identities.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub ids: Box<[UniProtIsoform]>,
        /// Source isoform name.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub name: Option<Text>,
        /// Every source synonym.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub synonyms: Box<[Text]>,
        /// Native sequence status; no sequence is inferred from it.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub sequence_status: Option<ProteinIsoformStatus>,
        /// Source VSP sequence identifiers.
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
        )]
        pub sequence_ids: Box<[Text]>,
        /// Explicit direct-sequence retrieval state, independent of descriptive status.
        pub direct_sequence: ProteinSequenceAvailability,
    }
}
