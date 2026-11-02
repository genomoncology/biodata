// Keep the public variants and their stable codes in one declaration.
macro_rules! errors {
    ($( $(#[$doc:meta])* $variant:ident => $code:literal ),+ $(,)?) => {
        /// A rejected value, without retaining the input text.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Error { $( $(#[$doc])* $variant, )+ }
        impl Error {
            /// Return the stable machine-readable code.
            pub const fn code(self) -> &'static str {
                match self { $(Self::$variant => $code,)+ }
            }
        }
    };
}

errors! {
    /// An article needs at least one checked identity.
    ArticleIdentityRequired => "article identity required",
    /// The checked value was rejected.
    GwasValueInvalid => "gwas.value.invalid",
    /// The checked value was rejected.
    ReportIdInvalid => "report.id.invalid",
    /// The checked value was rejected.
    CellLineIdInvalid => "cell_line.id.invalid",
    /// The checked value was rejected.
    ReactomeIdInvalid => "id.reactome.invalid",
    /// The checked value was rejected.
    HpIdInvalid => "invalid_hp_id",
    /// The checked value was rejected.
    AuthorIdSyntax => "author.id.syntax",
    /// The checked value was rejected.
    AuthorIdChecksum => "author.id.checksum",
    /// The checked value was rejected.
    WikiPathwaysIdInvalid => "id.wikipathways.invalid",
    /// The checked value was rejected.
    PathwayIdInvalid => "id.pathway.invalid",
    /// The checked value was rejected.
    TrialCoordinatesInvalid => "trial.coordinates_invalid",
    /// The checked value was rejected.
    LicenseUnknown => "license.unknown",
    /// The checked value was rejected.
    ArticleMatchKindUnknown => "article.match_kind.unknown",
    /// The checked value was rejected.
    ProteinBoundAbsent => "protein.bound.absent",

    /// A source-native study identity or timestamp failed its grammar.
    StudyValueInvalid => "study.value.invalid",
    /// Invalid amino-acid residue text.
    ProteinResidueInvalid => "protein.residue.invalid",
    /// A protein site ends before its position.
    ProteinSiteOrder => "protein.site.order",
    /// Malformed UniProt accession or isoform suffix.
    UniProtIsoformInvalid => "id.uniprot_isoform.invalid",
    /// Malformed integrated InterPro accession.
    InterProIdInvalid => "id.interpro.invalid",
    /// Malformed Complex Portal accession.
    ComplexPortalIdInvalid => "id.complex_portal.invalid",
    /// Invalid ClinPGx object accession.
    ClinPgxAccession => "id.clinpgx.invalid",
    /// A source-native diagnostic identifier failed its grammar.
    DiagnosticIdInvalid => "diagnostic.id.invalid",
    /// A published native drug identifier was invalid.
    DrugIdInvalid => "drug.id.invalid",
    /// Neither a source gene nor native target identifies the relation.
    DrugTargetAbsent => "drug.target_absent",
    /// Interaction provenance repeats a primary or further attribution.
    DrugAttributionDuplicate => "drug.attribution_duplicate",
    /// The source identity level is outside the closed vocabulary.
    DrugIdentityLevelUnknown => "drug.identity_level.unknown",
    /// A shared numeric value violates its bounds.
    PopulationCountExceedsNumber => "population.count.exceeds_number",
    /// A shared numeric value violates its bounds.
    PopulationFrequencyRange => "population.frequency.range",
    /// A mitochondrial field appears on a known non-mitochondrial allele.
    PopulationMitochondrialFieldMisplaced => "population.mt_field.misplaced",
    /// A source grpmax group appears on a named ancestry stratum.
    PopulationGrpmaxMisplaced => "population.grpmax.misplaced",
    /// A shared numeric value violates its bounds.
    NumberNonFinite => "number.value.non_finite",
    /// A subject reference lacks its source identity or notation.
    VariantRefInvalid => "variant.reference.invalid",
    /// A profile expression has invalid syntax or relations.
    ProfileExpressionInvalid => "variant.profile.expression_invalid",
    /// A profile expression references an absent native member.
    ProfileMemberMissing => "variant.profile.member_missing",
    /// A profile repeats a native member identifier.
    ProfileMemberDuplicate => "variant.profile.member_duplicate",
    /// A concept identifier lacks a checked prefix or local part.
    CurieInvalid => "concept.curie.invalid",
    /// The shared grammar refused the value.
    AccessionDigitsWidth => "accession.digits.width",
    /// The shared grammar refused the value.
    AccessionVersionMissing => "accession.version.missing",
    /// The shared grammar refused the value.
    AccessionVersionLeadingZero => "accession.version.leading_zero",
    /// The shared grammar refused the value.
    AccessionVersionOverflow => "accession.version.overflow",
    /// The shared grammar refused the value.
    AccessionLrgInvalid => "accession.lrg.invalid",
    /// The shared grammar refused the value.
    SequenceRna => "sequence.base.rna",
    /// The shared grammar refused the value.
    ExpressionSyntaxUnknown => "expression.syntax.unknown",
    /// The common value refused this input.
    IntervalBuildAbsent => "coord.interval.build_absent",
    /// The common value refused this input.
    IntervalBuildMismatch => "coord.interval.build_mismatch",
    /// The common value refused this input.
    IntervalChromosomeMismatch => "coord.interval.chromosome_mismatch",
    /// The common value refused this input.
    AccessionVersionPresent => "accession.version.present",
    /// The common value refused this input.
    ManeStatusUnknown => "gene.mane.status_unknown",
    /// The common value refused this input.
    GeneRefLabelEmpty => "gene.ref.label_empty",
    /// The common value refused this input.
    CodingChangeInvalid => "hgvs.coding.invalid",
    /// The common value refused this input.
    HgvsTextEmpty => "hgvs.text.empty",
    /// The common value refused this input.
    HgvsReferenceMissing => "hgvs.reference.missing",
    /// The common value refused this input.
    HgvsReferenceInvalid => "hgvs.reference.invalid",
    /// The common value refused this input.
    HgvsInnerReferenceInvalid => "hgvs.inner_reference.invalid",
    /// The common value refused this input.
    HgvsSyntaxUnknown => "hgvs.syntax.unknown",
    /// The common value refused this input.
    HgvsBodyEmpty => "hgvs.body.empty",
    /// The common value refused this input.
    HgvsBodyUnprintable => "hgvs.body.unprintable",
    /// The common value refused this input.
    RsIdInvalid => "id.rs.invalid",
    /// The common value refused this input.
    CaIdInvalid => "id.ca.invalid",
    /// The common value refused this input.
    PaIdInvalid => "id.pa.invalid",
    /// The common value refused this input.
    VcvIdInvalid => "id.vcv.invalid",
    /// The common value refused this input.
    RcvIdInvalid => "id.rcv.invalid",
    /// The common value refused this input.
    ScvIdInvalid => "id.scv.invalid",
    /// The common value refused this input.
    ClinVarVariationIdInvalid => "id.clinvar_variation.invalid",
    /// The common value refused this input.
    ClinVarAlleleIdInvalid => "id.clinvar_allele.invalid",
    /// The common value refused this input.
    CosmicIdInvalid => "id.cosmic.invalid",
    /// The common value refused this input.
    VrsIdInvalid => "id.vrs.invalid",
    /// A published variant identifier was invalid.
    VariantIdInvalid => "id.variant.unknown",
    /// The source notation did not match its declared form.
    NotationInvalid => "expression.value.invalid",
    /// A versioned sequence accession was invalid.
    AccessionInvalid => "accession.prefix.unknown",
    /// A canonical primary chromosome was invalid.
    ChromosomeInvalid => "coord.chromosome.invalid",
    /// A DNA base or sequence was invalid.
    SequenceInvalid => "sequence.base.invalid",
    /// Allele reference length differs from its span.
    AlleleReferenceLength => "allele.reference.length",
    /// An allele has neither reference nor alternate bases.
    AlleleEmpty => "allele.state.empty",
    /// Input is not a published variant identifier.
    VariantInputInvalid => "variant.input.invalid",
    /// A trial identifier failed its namespace syntax.
    TrialIdInvalid => "id.trial.invalid",
    /// An article identifier failed its namespace syntax.
    ArticleIdInvalid => "id.article.invalid",
    /// The checked NcbiGeneIdInvalid value was rejected.
    NcbiGeneIdInvalid => "id.ncbi_gene.invalid",
    /// The checked GeneSymbolInvalid value was rejected.
    GeneSymbolInvalid => "gene.symbol.invalid",
    /// The checked OmimIdInvalid value was rejected.
    OmimIdInvalid => "id.omim.invalid",
    /// The checked UniProtAccInvalid value was rejected.
    UniProtAccInvalid => "id.uniprot.invalid",
    /// The checked CytobandInvalid value was rejected.
    CytobandInvalid => "cytoband.invalid",
    /// The checked EnsemblGeneIdInvalid value was rejected.
    EnsemblGeneIdInvalid => "id.ensembl_gene.invalid",
    /// The GeneStatusUnknown boundary rejected a value.
    GeneStatusUnknown => "gene.status.unknown",
    /// The LocusGroupUnknown boundary rejected a value.
    LocusGroupUnknown => "gene.locus_group.unknown",
    /// The MatchKindUnknown boundary rejected a value.
    MatchKindUnknown => "gene.match_kind.unknown",
    /// The SummarySourceUnknown boundary rejected a value.
    SummarySourceUnknown => "gene.summary_source.unknown",
    /// The HgncStatusUnknown boundary rejected a value.
    HgncStatusUnknown => "gene.hgnc_status.unknown",
    /// The LocusTypeUnknown boundary rejected a value.
    LocusTypeUnknown => "gene.locus_type.unknown",
    /// The BuildUnknown boundary rejected a value.
    BuildUnknown => "coord.build.unknown",
    /// The StrandUnknown boundary rejected a value.
    StrandUnknown => "coord.strand.unknown",
    /// The ContigInvalid boundary rejected a value.
    ContigInvalid => "coord.contig.invalid",
    /// An HGNC identifier was not canonical positive u32 text.
    HgncIdInvalid => "id.hgnc.invalid",
    /// A date had invalid precision or calendar components.
    DateInvalid => "date.invalid",
    /// A one-based position was zero.
    PositionZero => "coord.position.zero",
    /// A span ended before its start.
    SpanOrder => "coord.span.order",
    /// Text contained unsafe terminal or bidi controls.
    TextControl => "text.control",
    /// Text exceeded its constructor character cap.
    TextTooLong => "text.too_long",
    /// A native document upload timestamp failed its grammar.
    TrialUploadTimeInvalid => "trial.upload_time.invalid",
    /// Text contained no non-whitespace characters.
    TextEmpty => "text.empty",
    /// A source name was not recognized.
    SourceUnknown => "source.unknown",
    /// A closed wire vocabulary rejected an unknown spelling.
    WireEnumUnknown => "wire_enum.unknown",
    /// A reason name was not recognized.
    ReasonUnknown => "reason.unknown",
    /// A problem code was not recognized.
    ProblemCodeUnknown => "problem_code.unknown",
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for Error {}
