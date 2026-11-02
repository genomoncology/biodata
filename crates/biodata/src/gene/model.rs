use super::{GeneStatus, LocusGroup, LocusType, Summary};
use crate::Mane;
use crate::{
    EnsemblGeneId, GeneLocation, GeneSymbol, GenomicInterval, HgncId, NcbiGeneId, OmimId, Text,
    UniProtAcc,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Human gene identity and whole source annotations, independent of response cards.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Gene {
    /// HGNC identifier.
    pub hgnc_id: HgncId,
    /// Source-preserved symbol.
    pub symbol: GeneSymbol,
    /// NCBI Gene identifier, if stated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub entrez_id: Option<NcbiGeneId>,
    /// Ensembl gene identifier, if stated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ensembl_id: Option<EnsemblGeneId>,
    /// HGNC record status.
    pub status: GeneStatus,
    /// HGNC locus group.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub locus_group: Option<LocusGroup>,
    /// HGNC locus type, if stated and recognized.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub locus_type: Option<LocusType>,
    /// Source-stated location, including unparsed publisher text.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub location: Option<GeneLocation>,
    /// All stated previous symbols.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub previous_symbols: Box<[GeneSymbol]>,
    /// All stated HGNC aliases.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub aliases: Box<[Text]>,
    /// All stated UniProt base accessions.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub uniprot: Box<[UniProtAcc]>,
    /// All stated OMIM identifiers.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub omim: Box<[OmimId]>,
    /// Versioned MANE Select transcript pair, if stated.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[ _ ]>::is_empty")
    )]
    pub mane: Box<[Mane]>,
    /// Source intervals with explicit assembly and counting rule.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "<[_]>::is_empty")
    )]
    pub spans: Box<[GenomicInterval]>,
    /// Gene name.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Text>,
    /// Whole attributed summary, if stated.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub summary: Option<Summary>,
}
