//! Owned transcript and source alignment values.
use crate::{
    Accession, AccessionBase, EnsemblFeature, GeneSymbol, HgncId, IdError, NcbiGeneId,
    RefSeqPrefix, Span0, Strand,
};
use std::{fmt, str::FromStr};

/// One gene label and the publisher identifiers known for it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "GeneRefFields"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(rename = "SymbolGeneRef", !try_from))]
pub struct GeneRef {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    hgnc_id: Option<HgncId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    ncbi_gene_id: Option<NcbiGeneId>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "schema",
        schemars(default, schema_with = "symbol_gene_id_schema")
    )]
    ensembl_gene_id: Option<AccessionBase>,
    label: GeneSymbol,
}

#[cfg(feature = "schema")]
pub(crate) fn symbol_gene_id_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"anyOf":[{"type":"string", "pattern":"^ENSG[0-9]{11}$"},{"type":"null"}]})
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
struct GeneRefFields {
    hgnc_id: Option<HgncId>,
    ncbi_gene_id: Option<NcbiGeneId>,
    ensembl_gene_id: Option<AccessionBase>,
    label: GeneSymbol,
}

#[cfg(feature = "serde")]
impl TryFrom<GeneRefFields> for GeneRef {
    type Error = IdError;

    fn try_from(fields: GeneRefFields) -> Result<Self, Self::Error> {
        Self::new(
            fields.hgnc_id,
            fields.ncbi_gene_id,
            fields.ensembl_gene_id,
            fields.label,
        )
    }
}
impl GeneRef {
    /// Bind a validated label to the identifiers supplied by known sources.
    pub fn new(
        hgnc_id: Option<HgncId>,
        ncbi_gene_id: Option<NcbiGeneId>,
        ensembl_gene_id: Option<AccessionBase>,
        label: GeneSymbol,
    ) -> Result<Self, IdError> {
        if ensembl_gene_id.is_some_and(|id| id.ensembl_feature() != Some(EnsemblFeature::Gene)) {
            return Err(IdError);
        }
        Ok(Self {
            hgnc_id,
            ncbi_gene_id,
            ensembl_gene_id,
            label,
        })
    }
    /// HGNC identity when supplied.
    pub fn hgnc_id(&self) -> Option<HgncId> {
        self.hgnc_id
    }
    /// NCBI Gene identity when supplied.
    pub fn ncbi_gene_id(&self) -> Option<NcbiGeneId> {
        self.ncbi_gene_id
    }
    /// Ensembl gene identity when supplied.
    pub fn ensembl_gene_id(&self) -> Option<AccessionBase> {
        self.ensembl_gene_id
    }
    /// Source gene label.
    pub fn label(&self) -> &GeneSymbol {
        &self.label
    }
}

/// Transcript facts independent of a genomic alignment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transcript {
    accession: Accession,
    gene: GeneRef,
    protein: Option<Accession>,
    coding: Option<Span0>,
    length: u32,
    mane: Option<Mane>,
}
impl Transcript {
    /// Check coding bounds against the full transcript length.
    pub fn new(
        accession: Accession,
        gene: GeneRef,
        protein: Option<Accession>,
        coding: Option<Span0>,
        length: u32,
        mane: Option<Mane>,
    ) -> Result<Self, TranscriptError> {
        if length == 0 || coding.is_some_and(|span| span.is_empty() || span.end() > length) {
            return Err(TranscriptError::CodingBounds);
        }
        if mane
            .as_ref()
            .is_some_and(|pair| accession != pair.refseq() && accession != pair.ensembl())
        {
            return Err(TranscriptError::ManeAccession);
        }
        Ok(Self {
            accession,
            gene,
            protein,
            coding,
            length,
            mane,
        })
    }
    /// Versioned transcript accession.
    pub fn accession(&self) -> Accession {
        self.accession
    }
    /// Gene identity and label.
    pub fn gene(&self) -> &GeneRef {
        &self.gene
    }
    /// Protein accession when supplied.
    pub fn protein(&self) -> Option<Accession> {
        self.protein
    }
    /// Coding span on the transcript.
    pub fn coding(&self) -> Option<Span0> {
        self.coding
    }
    /// Full transcript length.
    pub fn length(&self) -> u32 {
        self.length
    }
    /// Exact MANE pair when published.
    pub fn mane(&self) -> Option<&Mane> {
        self.mane.as_ref()
    }
}

/// A MANE pair's published status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ManeStatus {
    /// The selected representative transcript pair.
    Select,
    /// A further clinically relevant transcript pair.
    PlusClinical,
}

/// Why transcript facts cannot form an owned value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranscriptError {
    /// Coding span is empty or outside the transcript.
    CodingBounds,
    /// A MANE pair has the wrong transcript accession families.
    ManePair,
    /// The transcript does not name either accession of its stated MANE pair.
    ManeAccession,
}
impl TranscriptError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::CodingBounds => "transcript.coding_bounds",
            Self::ManePair => "transcript.mane_pair",
            Self::ManeAccession => "transcript.mane_accession",
        }
    }
}
impl fmt::Display for TranscriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for TranscriptError {}

/// One paired MANE transcript record.
///
/// NCBI and Ensembl publish versioned RefSeq/Ensembl transcript pairs:
/// https://www.ncbi.nlm.nih.gov/refseq/MANE/ and
/// https://static.ensembl.org/info/genome/genebuild/transcript_quality_tags.html.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "ManeFields"))]
pub struct Mane {
    /// Published status.
    pub status: ManeStatus,
    /// Versioned RefSeq transcript.
    refseq: Accession,
    /// Versioned Ensembl transcript.
    ensembl: Accession,
}
impl Mane {
    /// Check RefSeq NM_/NR_ and Ensembl ENST accession families.
    pub fn new(
        status: ManeStatus,
        refseq: Accession,
        ensembl: Accession,
    ) -> Result<Self, TranscriptError> {
        if !matches!(
            refseq.refseq_parts(),
            Some((RefSeqPrefix::Nm | RefSeqPrefix::Nr, _, _))
        ) || !matches!(
            ensembl.ensembl_parts(),
            Some((EnsemblFeature::Transcript, _, _))
        ) {
            return Err(TranscriptError::ManePair);
        }
        Ok(Self {
            status,
            refseq,
            ensembl,
        })
    }
    /// Published pair status.
    pub fn status(&self) -> ManeStatus {
        self.status
    }
    /// Versioned RefSeq transcript accession.
    pub fn refseq(&self) -> Accession {
        self.refseq
    }
    /// Versioned Ensembl transcript accession.
    pub fn ensembl(&self) -> Accession {
        self.ensembl
    }
}

/// An operation in a SAM CIGAR with the genome as reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CigarOp {
    /// Number of bases consumed.
    pub count: u32,
    /// SAM operation code.
    pub code: CigarCode,
}

/// SAM alignment operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CigarCode {
    /// Aligned bases with unspecified equality.
    Match,
    /// Equal bases.
    Equal,
    /// Unequal bases.
    Mismatch,
    /// Transcript bases absent from the genome.
    Insertion,
    /// Genome bases absent from the transcript.
    Deletion,
}
impl CigarCode {
    fn letter(self) -> char {
        match self {
            Self::Match => 'M',
            Self::Equal => '=',
            Self::Mismatch => 'X',
            Self::Insertion => 'I',
            Self::Deletion => 'D',
        }
    }
    fn parse(letter: char) -> Option<Self> {
        match letter {
            'M' => Some(Self::Match),
            '=' => Some(Self::Equal),
            'X' => Some(Self::Mismatch),
            'I' => Some(Self::Insertion),
            'D' => Some(Self::Deletion),
            _ => None,
        }
    }
}

/// Why an alignment value could not be constructed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignmentError {
    /// The genome exon span is empty or reversed.
    GenomeSpan,
    /// The transcript exon span is empty or reversed.
    TranscriptSpan,
    /// Exons overlap or reverse their genomic order.
    ExonOrder,
    /// An operation count or letter is malformed.
    OperationSyntax,
    /// An operation count exceeds a span or integer bound.
    OperationBounds,
    /// Operations do not consume both stated exon spans.
    OperationConsumption,
}
impl AlignmentError {
    /// Stable refusal code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::GenomeSpan => "alignment.genome_span",
            Self::TranscriptSpan => "alignment.transcript_span",
            Self::ExonOrder => "alignment.exon_order",
            Self::OperationSyntax => "alignment.operation_syntax",
            Self::OperationBounds => "alignment.operation_bounds",
            Self::OperationConsumption => "alignment.operation_consumption",
        }
    }
}
impl fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for AlignmentError {}

/// Parsed CIGAR operations with the genome as reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cigar(Box<[CigarOp]>);
impl Cigar {
    /// Construct a checked genome-reference CIGAR from parsed operations.
    pub fn new(operations: impl Into<Box<[CigarOp]>>) -> Result<Self, AlignmentError> {
        let operations = operations.into();
        if operations.is_empty() || operations.iter().any(|op| op.count == 0) {
            return Err(AlignmentError::OperationSyntax);
        }
        Ok(Self(operations))
    }
    /// Operations in source order.
    pub fn operations(&self) -> &[CigarOp] {
        &self.0
    }
    /// Check that operations consume exactly the stated genomic and transcript spans.
    pub fn check_spans(&self, transcript: Span0, reference: Span0) -> Result<(), AlignmentError> {
        let (mut tx, mut genome) = (0_u32, 0_u32);
        for op in &self.0 {
            if op.code != CigarCode::Deletion {
                tx = tx
                    .checked_add(op.count)
                    .ok_or(AlignmentError::OperationBounds)?;
            }
            if op.code != CigarCode::Insertion {
                genome = genome
                    .checked_add(op.count)
                    .ok_or(AlignmentError::OperationBounds)?;
            }
            if tx > transcript.len() as u32 || genome > reference.len() as u32 {
                return Err(AlignmentError::OperationBounds);
            }
        }
        if tx != transcript.len() as u32 || genome != reference.len() as u32 {
            return Err(AlignmentError::OperationConsumption);
        }
        Ok(())
    }
    /// Parse a transcript-reference CIGAR into genome-reference SAM operations.
    pub fn from_transcript_reference(text: &str) -> Result<Self, AlignmentError> {
        let mut cigar: Self = text.parse()?;
        for op in &mut cigar.0 {
            op.code = match op.code {
                CigarCode::Insertion => CigarCode::Deletion,
                CigarCode::Deletion => CigarCode::Insertion,
                code => code,
            };
        }
        Ok(cigar)
    }
}
impl FromStr for Cigar {
    type Err = AlignmentError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut ops = Vec::new();
        let mut count = 0_u32;
        let mut digits = false;
        for letter in text.chars() {
            if let Some(digit) = letter.to_digit(10).filter(|_| letter.is_ascii_digit()) {
                count = count
                    .checked_mul(10)
                    .and_then(|n| n.checked_add(digit))
                    .ok_or(AlignmentError::OperationBounds)?;
                digits = true;
                continue;
            }
            let code = CigarCode::parse(letter).ok_or(AlignmentError::OperationSyntax)?;
            if !digits || count == 0 {
                return Err(AlignmentError::OperationSyntax);
            }
            ops.push(CigarOp { count, code });
            count = 0;
            digits = false;
        }
        if ops.is_empty() || digits {
            return Err(AlignmentError::OperationSyntax);
        }
        Self::new(ops)
    }
}
impl fmt::Display for Cigar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for op in &self.0 {
            write!(f, "{}{}", op.count, op.code.letter())?;
        }
        Ok(())
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl serde::Serialize for Cigar {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
#[cfg(feature = "serde")]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Cigar {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

/// One aligned exon in transcript order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlignedExon {
    /// Zero-based half-open transcript bounds.
    transcript: Span0,
    /// Zero-based half-open genome bounds.
    reference: Span0,
    /// Parsed genome-reference operations.
    cigar: Cigar,
}
impl AlignedExon {
    /// Check both exon spans and their CIGAR consumption.
    pub fn new(transcript: Span0, reference: Span0, cigar: Cigar) -> Result<Self, AlignmentError> {
        if transcript.is_empty() {
            return Err(AlignmentError::TranscriptSpan);
        }
        if reference.is_empty() {
            return Err(AlignmentError::GenomeSpan);
        }
        cigar.check_spans(transcript, reference)?;
        Ok(Self {
            transcript,
            reference,
            cigar,
        })
    }
    /// Transcript bounds.
    pub fn transcript(&self) -> Span0 {
        self.transcript
    }
    /// Genomic reference bounds.
    pub fn reference(&self) -> Span0 {
        self.reference
    }
    /// Genome-reference operations.
    pub fn cigar(&self) -> &Cigar {
        &self.cigar
    }
}

/// One complete transcript alignment on a genomic reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranscriptAlignment {
    /// Exact transcript accession.
    transcript: Accession,
    /// Exact genomic reference accession.
    reference: Accession,
    /// Transcript direction on the genome.
    strand: Strand,
    /// Publisher alignment method.
    method: Box<str>,
    /// Exons in transcript order.
    exons: Box<[AlignedExon]>,
}
impl TranscriptAlignment {
    /// Refuse missing, overlapping or out-of-order exons.
    pub fn new(
        transcript: Accession,
        reference: Accession,
        strand: Strand,
        method: impl Into<Box<str>>,
        exons: impl Into<Box<[AlignedExon]>>,
    ) -> Result<Self, AlignmentError> {
        let exons = exons.into();
        if exons.is_empty() {
            return Err(AlignmentError::TranscriptSpan);
        }
        for (first, second) in exons.iter().zip(exons.iter().skip(1)) {
            if first.transcript.end() > second.transcript.start() {
                return Err(AlignmentError::TranscriptSpan);
            }
            let wrong_order = match strand {
                Strand::Plus => first.reference.end() > second.reference.start(),
                Strand::Minus => second.reference.end() > first.reference.start(),
            };
            if wrong_order {
                return Err(AlignmentError::ExonOrder);
            }
        }
        Ok(Self {
            transcript,
            reference,
            strand,
            method: method.into(),
            exons,
        })
    }
    /// Exact transcript accession.
    pub fn transcript(&self) -> Accession {
        self.transcript
    }
    /// Exact genomic reference accession.
    pub fn reference(&self) -> Accession {
        self.reference
    }
    /// Transcript direction on the genome.
    pub fn strand(&self) -> Strand {
        self.strand
    }
    /// Publisher alignment method.
    pub fn method(&self) -> &str {
        &self.method
    }
    /// Checked exons in transcript order.
    pub fn exons(&self) -> &[AlignedExon] {
        &self.exons
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct ManeFields {
    status: ManeStatus,
    refseq: Accession,
    ensembl: Accession,
}
#[cfg(feature = "serde")]
impl TryFrom<ManeFields> for Mane {
    type Error = TranscriptError;
    fn try_from(fields: ManeFields) -> Result<Self, Self::Error> {
        Self::new(fields.status, fields.refseq, fields.ensembl)
    }
}

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(GeneRef, GeneRefFields);

#[cfg(feature = "serde")]
crate::serde_object::object_deserialize!(Mane, ManeFields);
