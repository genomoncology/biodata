use crate::{ClinVarAlleleId, ClinVarVariationId, Error, Text, VcvId};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// One source-stated member of a native ClinVar haplotype.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClinvarHaplotypeMember {
    /// Native ClinVar variation identity.
    pub id: ClinVarVariationId,
    /// Native ClinVar allele identity.
    pub allele_id: ClinVarAlleleId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
enum ClinvarHaplotypeKind {
    Haplotype,
}

/// A native ClinVar haplotype, without inferred allele identity or phase.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(inline))]
pub struct ClinvarHaplotype {
    kind: ClinvarHaplotypeKind,
    /// Native ClinVar variation identity.
    pub id: ClinVarVariationId,
    /// Native unversioned VCV accession.
    pub accession: VcvId,
    /// Explicit native accession version.
    pub version: Text,
    /// Literal NumberOfChromosomes supplied by ClinVar.
    pub number_of_chromosomes: Text,
    /// Complete member identities in native order.
    pub members: Box<[ClinvarHaplotypeMember]>,
}
impl ClinvarHaplotype {
    /// Preserve source grouping and identities without interpreting their biology.
    pub fn new(
        id: ClinVarVariationId,
        accession: VcvId,
        version: Text,
        number_of_chromosomes: Text,
        members: Box<[ClinvarHaplotypeMember]>,
    ) -> Result<Self, Error> {
        let value = Self {
            kind: ClinvarHaplotypeKind::Haplotype,
            id,
            accession,
            version,
            number_of_chromosomes,
            members,
        };
        value.check()?;
        Ok(value)
    }
    pub(super) fn check(&self) -> Result<(), Error> {
        if self.accession.version().is_some() || self.members.is_empty() {
            return Err(Error::VariantRefInvalid);
        }
        format!("{}.{}", self.accession, self.version)
            .parse::<VcvId>()
            .map_err(|_| Error::VariantRefInvalid)?;
        Ok(())
    }
}
