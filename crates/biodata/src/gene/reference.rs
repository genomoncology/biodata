use crate::{EnsemblGeneId, Error, HgncId, NcbiGeneId, Text};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// A source-stated gene reference; its label never implies an identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Fields"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
#[cfg_attr(feature = "schema", schemars(rename = "GeneRef"))]
pub struct SourceGeneRef(Fields);
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    hgnc_id: Option<HgncId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ncbi_gene_id: Option<NcbiGeneId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ensembl_gene_id: Option<EnsemblGeneId>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "label"))]
    label: Text,
}
impl SourceGeneRef {
    /// Preserve exactly the identifiers and label the source supplies.
    pub fn new(
        label: Text,
        hgnc_id: Option<HgncId>,
        ncbi_gene_id: Option<NcbiGeneId>,
        ensembl_gene_id: Option<EnsemblGeneId>,
    ) -> Result<Self, Error> {
        Self::try_from(Fields {
            label,
            hgnc_id,
            ncbi_gene_id,
            ensembl_gene_id,
        })
    }
    /// Borrow the complete source label.
    pub fn label(&self) -> &Text {
        &self.0.label
    }
    /// Source-stated HGNC identity.
    pub fn hgnc_id(&self) -> Option<HgncId> {
        self.0.hgnc_id
    }
    /// Source-stated NCBI Gene identity.
    pub fn ncbi_gene_id(&self) -> Option<NcbiGeneId> {
        self.0.ncbi_gene_id
    }
    /// Source-stated Ensembl Gene identity.
    pub fn ensembl_gene_id(&self) -> Option<&EnsemblGeneId> {
        self.0.ensembl_gene_id.as_ref()
    }
}
impl TryFrom<Fields> for SourceGeneRef {
    type Error = Error;
    fn try_from(value: Fields) -> Result<Self, Error> {
        if value.label.as_str().trim().is_empty() {
            return Err(Error::GeneRefLabelEmpty);
        }
        Ok(Self(value))
    }
}

#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
fn label<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Text, D::Error> {
    Text::new(&String::deserialize(d)?)
        .map_err(|_| serde::de::Error::custom(Error::GeneRefLabelEmpty))
}

impl TryFrom<crate::GeneSymbol> for SourceGeneRef {
    type Error = Error;
    fn try_from(symbol: crate::GeneSymbol) -> Result<Self, Error> {
        Self::new(Text::new(symbol.as_str())?, None, None, None)
    }
}

impl TryFrom<&crate::GeneRef> for SourceGeneRef {
    type Error = Error;
    fn try_from(value: &crate::GeneRef) -> Result<Self, Error> {
        Self::new(
            value.label().as_str().parse()?,
            value.hgnc_id(),
            value.ncbi_gene_id(),
            value
                .ensembl_gene_id()
                .map(|id| id.to_string().parse())
                .transpose()?,
        )
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(SourceGeneRef, Fields);
