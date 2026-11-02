//! Neutral consequence and gene extent values.
use crate::{
    Accession, AlleleKey, Attribution, Coding, GeneRef, GenomicInterval, HgvsText, Region,
};

/// A Sequence Ontology term with the fixed `so` coding system.
///
/// Terms: https://github.com/The-Sequence-Ontology/SO-Ontologies/blob/master/Ontology_Files/so.obo
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoTerm {
    /// Synonymous variant, SO:0001819.
    Synonymous,
    /// 3-prime UTR variant, SO:0001624.
    ThreePrimeUtr,
    /// Splice acceptor variant, SO:0001574.
    SpliceAcceptor,
    /// Splice donor variant, SO:0001575.
    SpliceDonor,
    /// Splice region variant, SO:0001630.
    SpliceRegion,
    /// Intron variant, SO:0001627.
    Intron,
    /// 5-prime UTR variant, SO:0001623.
    FivePrimeUtr,
    /// Stop gained, SO:0001587.
    StopGained,
    /// Missense variant, SO:0001583.
    Missense,
    /// Inframe deletion, SO:0001822.
    InframeDeletion,
    /// Inframe insertion, SO:0001821.
    InframeInsertion,
    /// Frameshift variant, SO:0001589.
    Frameshift,
    /// Stop lost, SO:0001578.
    StopLost,
    /// Noncoding transcript variant, SO:0001619.
    NoncodingTranscript,
}
impl SoTerm {
    pub(crate) const ALL: [Self; 14] = [
        Self::Synonymous,
        Self::ThreePrimeUtr,
        Self::SpliceAcceptor,
        Self::SpliceDonor,
        Self::SpliceRegion,
        Self::Intron,
        Self::FivePrimeUtr,
        Self::StopGained,
        Self::Missense,
        Self::InframeDeletion,
        Self::InframeInsertion,
        Self::Frameshift,
        Self::StopLost,
        Self::NoncodingTranscript,
    ];
    /// Sequence Ontology is the coding system for every value.
    pub const fn system(self) -> &'static str {
        "so"
    }
    /// Stable Sequence Ontology accession.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Synonymous => "SO:0001819",
            Self::ThreePrimeUtr => "SO:0001624",
            Self::SpliceAcceptor => "SO:0001574",
            Self::SpliceDonor => "SO:0001575",
            Self::SpliceRegion => "SO:0001630",
            Self::Intron => "SO:0001627",
            Self::FivePrimeUtr => "SO:0001623",
            Self::StopGained => "SO:0001587",
            Self::Missense => "SO:0001583",
            Self::InframeDeletion => "SO:0001822",
            Self::InframeInsertion => "SO:0001821",
            Self::Frameshift => "SO:0001589",
            Self::StopLost => "SO:0001578",
            Self::NoncodingTranscript => "SO:0001619",
        }
    }
}

/// A checked RefSeq, Ensembl, or LRG transcript accession.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TranscriptAccession(Accession);
impl TranscriptAccession {
    /// Accept only accessions whose parsed kind names a transcript.
    pub fn new(accession: Accession) -> Option<Self> {
        accession.is_transcript().then_some(Self(accession))
    }
    /// The checked source accession, including its version when supplied.
    pub const fn accession(self) -> Accession {
        self.0
    }
}

/// SO consequences observed for one genomic allele on one transcript.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Consequence {
    /// Genomic allele whose consequences were observed.
    pub allele: AlleleKey,
    /// Versioned transcript whose consequence was observed.
    pub transcript: TranscriptAccession,
    terms: Vec<SoTerm>,
}

impl Consequence {
    /// Construct a deterministic set without a severity ranking.
    pub fn new(allele: AlleleKey, transcript: TranscriptAccession, mut terms: Vec<SoTerm>) -> Self {
        terms.sort_unstable();
        terms.dedup();
        Self {
            allele,
            transcript,
            terms,
        }
    }

    /// Distinct SO terms without a severity ranking.
    pub fn terms(&self) -> &[SoTerm] {
        &self.terms
    }
}

/// Source-stated SO consequences for one transcript.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "TranscriptConsequenceFields"))]
pub struct TranscriptConsequence {
    transcript: Accession,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    gene: Option<GeneRef>,
    consequences: Vec<Coding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    coding_hgvs: Option<HgvsText>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    protein_hgvs: Option<HgvsText>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    region: Option<Region>,
    attribution: Attribution,
}
impl TranscriptConsequence {
    /// Retain a nonempty set of exact SO codes for a checked transcript.
    pub fn new(
        transcript: Accession,
        gene: Option<GeneRef>,
        consequences: Vec<Coding>,
        coding_hgvs: Option<HgvsText>,
        protein_hgvs: Option<HgvsText>,
        region: Option<Region>,
        attribution: Attribution,
    ) -> Result<Self, ConsequenceError> {
        if TranscriptAccession::new(transcript).is_none() {
            return Err(ConsequenceError::Transcript);
        }
        if consequences.is_empty() {
            return Err(ConsequenceError::Empty);
        }
        if consequences
            .iter()
            .any(|code| !matches!(code, Coding::So(crate::SoCode::Consequence(_))))
        {
            return Err(ConsequenceError::NotSo);
        }
        if consequences.iter().enumerate().any(|(index, code)| {
            consequences
                .get(..index)
                .is_some_and(|prior| prior.contains(code))
        }) {
            return Err(ConsequenceError::Duplicate);
        }
        Ok(Self {
            transcript,
            gene,
            consequences,
            coding_hgvs,
            protein_hgvs,
            region,
            attribution,
        })
    }

    /// Versioned source transcript.
    pub const fn transcript(&self) -> Accession {
        self.transcript
    }

    /// Source gene identity, if stated.
    pub fn gene(&self) -> Option<&GeneRef> {
        self.gene.as_ref()
    }

    /// Source SO codings, without a severity order.
    pub fn consequences(&self) -> &[Coding] {
        &self.consequences
    }

    /// Source attribution.
    pub fn attribution(&self) -> &Attribution {
        &self.attribution
    }
}

/// A transcript consequence violates its source value contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsequenceError {
    /// The accession does not name a transcript.
    Transcript,
    /// No consequence was supplied.
    Empty,
    /// A consequence is outside Sequence Ontology.
    NotSo,
    /// The same SO code occurs twice.
    Duplicate,
}

impl ConsequenceError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Transcript => "transcript.accession.invalid",
            Self::Empty => "consequence.list.empty",
            Self::NotSo => "consequence.so.unknown",
            Self::Duplicate => "consequence.list.duplicate",
        }
    }
}

impl std::fmt::Display for ConsequenceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ConsequenceError {}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct TranscriptConsequenceFields {
    transcript: Accession,
    gene: Option<GeneRef>,
    consequences: Vec<Coding>,
    coding_hgvs: Option<HgvsText>,
    protein_hgvs: Option<HgvsText>,
    region: Option<Region>,
    attribution: Attribution,
}

#[cfg(feature = "serde")]
impl TryFrom<TranscriptConsequenceFields> for TranscriptConsequence {
    type Error = ConsequenceError;

    fn try_from(fields: TranscriptConsequenceFields) -> Result<Self, Self::Error> {
        Self::new(
            fields.transcript,
            fields.gene,
            fields.consequences,
            fields.coding_hgvs,
            fields.protein_hgvs,
            fields.region,
            fields.attribution,
        )
    }
}

/// A caller's named rule set for an NMD escape prediction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NmdRuleId(&'static str);
impl NmdRuleId {
    /// Refuse an empty rule-set identifier.
    pub const fn new(value: &'static str) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Self(value))
        }
    }
    /// Caller-supplied rule-set identifier.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A reason for predicting escape from nonsense-mediated decay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum NmdEscapeReason {
    /// Premature stop in the last exon.
    #[cfg_attr(feature = "serde", serde(rename = "last_exon"))]
    LastExon,
    /// Premature stop near the last exon junction.
    #[cfg_attr(feature = "serde", serde(rename = "near_last_junction"))]
    NearLastExonJunction,
    /// Premature stop near the coding start.
    #[cfg_attr(feature = "serde", serde(rename = "near_coding_start"))]
    NearCodingStartSite,
    /// Premature stop in a long exon.
    #[cfg_attr(feature = "serde", serde(rename = "long_exon"))]
    LongExon,
}
impl NmdEscapeReason {
    /// Stable reason code, without a threshold.
    pub const fn code(self) -> &'static str {
        match self {
            Self::LastExon => "last_exon",
            Self::NearLastExonJunction => "near_last_junction",
            Self::NearCodingStartSite => "near_coding_start",
            Self::LongExon => "long_exon",
        }
    }
}

/// A predicted NMD escape reason and the exact policy that selected it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NmdEscape {
    rule: NmdRuleId,
    reason: NmdEscapeReason,
}
impl NmdEscape {
    /// Attach caller-selected provenance to a neutral reason.
    pub const fn new(rule: NmdRuleId, reason: NmdEscapeReason) -> Self {
        Self { rule, reason }
    }
    /// The caller's policy identifier.
    pub const fn rule(self) -> NmdRuleId {
        self.rule
    }
    /// The selected reason.
    pub const fn reason(self) -> NmdEscapeReason {
        self.reason
    }
}

/// A gene's identity and genomic extent on a stated reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneExtent {
    /// Source gene identity and label.
    pub gene: GeneRef,
    /// Genomic bounds and optional strand.
    pub interval: GenomicInterval,
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(TranscriptConsequence, TranscriptConsequenceFields);
