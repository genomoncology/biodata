//! Public transcript model and allele behavior.
use biodata::{
    Accession, AlignedExon, AlignmentError, Allele, Build, Chromosome, Cigar, CigarCode, CigarOp,
    EditKind, GeneRef, GenomicInterval, LiftFailure, LiftOutcome, Lifted, Mane, ManeStatus,
    Sequence, SequenceLocation, Span0, Strand, Transcript, TranscriptAlignment,
    vcf::{Alternate, Site, SiteError},
};

#[test]
fn cigar_uses_genome_as_reference_and_round_trips_sam_text() {
    let cigar: Cigar = "3=2I4X1D".parse().unwrap();
    assert_eq!(cigar.to_string(), "3=2I4X1D");
    assert_eq!("01M".parse::<Cigar>().unwrap().to_string(), "1M");
    assert_eq!(serde_json::to_string(&cigar).unwrap(), "\"3=2I4X1D\"");
    assert_eq!(
        serde_json::from_str::<Cigar>("\"3=2I4X1D\"").unwrap(),
        cigar
    );
    let tx = Span0::new(0, 9).unwrap();
    let genome = Span0::new(100, 108).unwrap();
    assert!(AlignedExon::new(tx, genome, cigar).is_ok());
    assert_eq!(
        Cigar::from_transcript_reference("3=2D4X1I")
            .unwrap()
            .to_string(),
        "3=2I4X1D"
    );
}

#[test]
fn cigar_rejects_syntax_bounds_and_incomplete_consumption() {
    assert_eq!(
        Cigar::new(Vec::<CigarOp>::new()),
        Err(AlignmentError::OperationSyntax)
    );
    assert_eq!(
        Cigar::new([CigarOp {
            count: 0,
            code: CigarCode::Match
        }]),
        Err(AlignmentError::OperationSyntax)
    );
    for source in ["", "0M", "M", "1Q", "1M2", "4294967296M"] {
        assert!(source.parse::<Cigar>().is_err(), "{source}");
    }
    let tx = Span0::new(0, 3).unwrap();
    let genome = Span0::new(100, 103).unwrap();
    assert_eq!(
        Cigar::from_transcript_reference("4M")
            .unwrap()
            .check_spans(tx, genome),
        Err(AlignmentError::OperationBounds)
    );
    assert_eq!(
        Cigar::from_transcript_reference("2M")
            .unwrap()
            .check_spans(tx, genome),
        Err(AlignmentError::OperationConsumption)
    );
    for (error, code) in [
        (AlignmentError::GenomeSpan, "alignment.genome_span"),
        (AlignmentError::TranscriptSpan, "alignment.transcript_span"),
        (AlignmentError::ExonOrder, "alignment.exon_order"),
        (
            AlignmentError::OperationSyntax,
            "alignment.operation_syntax",
        ),
        (
            AlignmentError::OperationBounds,
            "alignment.operation_bounds",
        ),
        (
            AlignmentError::OperationConsumption,
            "alignment.operation_consumption",
        ),
    ] {
        assert_eq!(error.code(), code);
        assert_eq!(error.to_string(), code);
    }
}

#[test]
fn alignment_checks_exon_order_in_both_directions() {
    let tx: Accession = "NM_004333.4".parse().unwrap();
    let reference: Accession = "NC_000007.14".parse().unwrap();
    let exon = |tx_start, genome_start| {
        AlignedExon::new(
            Span0::new(tx_start, tx_start + 3).unwrap(),
            Span0::new(genome_start, genome_start + 3).unwrap(),
            "3M".parse().unwrap(),
        )
        .unwrap()
    };
    let plus = TranscriptAlignment::new(
        tx,
        reference,
        Strand::Plus,
        "splign",
        vec![exon(0, 100), exon(3, 200)],
    );
    let plus = plus.unwrap();
    assert_eq!(plus.transcript(), tx);
    assert_eq!(plus.reference(), reference);
    assert_eq!(plus.strand(), Strand::Plus);
    assert_eq!(plus.method(), "splign");
    assert_eq!(plus.exons().len(), 2);
    assert_eq!(plus.exons()[0].transcript(), Span0::new(0, 3).unwrap());
    assert_eq!(plus.exons()[0].reference(), Span0::new(100, 103).unwrap());
    assert_eq!(plus.exons()[0].cigar().to_string(), "3M");
    let minus = TranscriptAlignment::new(
        tx,
        reference,
        Strand::Minus,
        "splign",
        vec![exon(0, 200), exon(3, 100)],
    );
    assert!(minus.is_ok());
    let wrong = TranscriptAlignment::new(
        tx,
        reference,
        Strand::Minus,
        "splign",
        vec![exon(0, 100), exon(3, 200)],
    );
    assert_eq!(wrong.err(), Some(AlignmentError::ExonOrder));
    assert_eq!(
        TranscriptAlignment::new(tx, reference, Strand::Plus, "splign", Vec::new()),
        Err(AlignmentError::TranscriptSpan)
    );
    assert_eq!(
        TranscriptAlignment::new(
            tx,
            reference,
            Strand::Plus,
            "splign",
            vec![exon(0, 100), exon(2, 200)]
        ),
        Err(AlignmentError::TranscriptSpan)
    );
    assert_eq!(
        AlignedExon::new(
            Span0::new(0, 0).unwrap(),
            Span0::new(100, 103).unwrap(),
            "3M".parse().unwrap()
        ),
        Err(AlignmentError::TranscriptSpan)
    );
    assert_eq!(
        AlignedExon::new(
            Span0::new(0, 3).unwrap(),
            Span0::new(100, 100).unwrap(),
            "3M".parse().unwrap()
        ),
        Err(AlignmentError::GenomeSpan)
    );
}

#[test]
fn allele_trim_preserves_source_and_minimal_replacement() {
    let accession: Accession = "NC_000007.14".parse().unwrap();
    let allele = Allele::new(
        SequenceLocation::new(accession, Span0::new(100, 104).unwrap()),
        "ACGT".parse::<Sequence>().unwrap(),
        "ACTT".parse::<Sequence>().unwrap(),
    )
    .unwrap();
    let trimmed = allele.trimmed();
    assert_eq!(trimmed.location().span(), Span0::new(102, 103).unwrap());
    assert_eq!(trimmed.reference().to_string(), "G");
    assert_eq!(trimmed.alternate().to_string(), "T");
    assert_eq!(trimmed.location().sequence(), accession);
    let unchanged = Allele::new(
        SequenceLocation::new(accession, Span0::new(100, 104).unwrap()),
        "ACGT".parse().unwrap(),
        "ACGT".parse().unwrap(),
    )
    .unwrap();
    assert_eq!(unchanged.trimmed(), unchanged);
    assert_eq!(
        biodata::trim_flanks(
            unchanged.reference().as_slice(),
            unchanged.alternate().as_slice()
        ),
        (0, 0)
    );
}

#[test]
fn vcf_site_retains_source_alt_classes_and_padded_reference() {
    let site = Site::new(
        "chr7".parse().unwrap(),
        biodata::Position1::new(140_453_136).unwrap(),
        Some("rs1".into()),
        "AT".parse().unwrap(),
        [
            Alternate::parse("A").unwrap(),
            Alternate::parse("*").unwrap(),
            Alternate::parse("<DEL>").unwrap(),
            Alternate::parse("A]chr9:42]").unwrap(),
            Alternate::parse(".").unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(site.chromosome().as_str(), "chr7");
    assert_eq!(site.position().get(), 140_453_136);
    assert_eq!(site.id(), Some("rs1"));
    assert_eq!(site.reference().to_string(), "AT");
    assert!(matches!(site.alternates()[0], Alternate::Sequence(_)));
    assert!(matches!(site.alternates()[1], Alternate::Star));
    assert!(matches!(site.alternates()[2], Alternate::Symbolic(_)));
    assert!(matches!(site.alternates()[3], Alternate::Breakend(_)));
    assert!(matches!(site.alternates()[4], Alternate::Missing));
}

#[test]
fn vcf_site_rejects_publicly_constructed_invalid_alternates() {
    let chromosome: biodata::Contig = "chr7".parse().unwrap();
    let position = biodata::Position1::new(42).unwrap();
    let reference: Sequence = "A".parse().unwrap();
    assert_eq!(
        Site::new(
            chromosome.clone(),
            position,
            None,
            "".parse().unwrap(),
            [Alternate::Star]
        ),
        Err(SiteError::EmptyReference)
    );
    for alternate in [
        Alternate::Sequence("".parse().unwrap()),
        Alternate::Symbolic("x".into()),
        Alternate::Breakend("x".into()),
    ] {
        assert_eq!(
            Site::new(
                chromosome.clone(),
                position,
                None,
                reference.clone(),
                [alternate]
            ),
            Err(SiteError::InvalidAlternate)
        );
    }
    for (error, code) in [
        (SiteError::EmptyReference, "vcf.reference.empty"),
        (SiteError::InvalidAlternate, "vcf.alternate.invalid"),
    ] {
        assert_eq!(error.code(), code);
        assert_eq!(error.to_string(), code);
    }
}

#[test]
fn edit_classes_hold_reviewed_so_terms() {
    let cases = [
        (EditKind::Substitution, "SO:1000002", "substitution"),
        (EditKind::Deletion, "SO:0000159", "deletion"),
        (EditKind::Insertion, "SO:0000667", "insertion"),
        (EditKind::Duplication, "SO:1000035", "duplication"),
        (EditKind::Inversion, "SO:1000036", "inversion"),
        (EditKind::Delins, "SO:1000032", "delins"),
        (EditKind::Same, "SO:0002073", "no_sequence_alteration"),
    ];
    for (kind, id, term) in cases {
        assert_eq!((kind.so_id(), kind.so_term()), (id, term));
    }
}

#[test]
fn mane_pair_carries_status_and_both_accessions() {
    let pair = Mane::new(
        ManeStatus::Select,
        "NM_004333.4".parse().unwrap(),
        "ENST00000288602.11".parse().unwrap(),
    )
    .unwrap();
    assert_eq!(pair.status(), ManeStatus::Select);
    assert_eq!(pair.refseq().to_string(), "NM_004333.4");
    assert_eq!(pair.ensembl().to_string(), "ENST00000288602.11");
    assert!(
        Mane::new(
            ManeStatus::Select,
            "NP_004324.2".parse().unwrap(),
            pair.ensembl()
        )
        .is_err()
    );
    assert!(
        Mane::new(
            ManeStatus::Select,
            pair.refseq(),
            "ENSG00000157764.1".parse().unwrap()
        )
        .is_err()
    );
    assert!(
        Mane::new(
            ManeStatus::PlusClinical,
            "NR_123456.1".parse().unwrap(),
            pair.ensembl()
        )
        .is_ok()
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "keep the reviewed transcript validation cases together"
)]
fn transcript_checks_coding_bounds_and_gene_namespace() {
    let gene = GeneRef::new(
        Some("HGNC:1097".parse().unwrap()),
        Some("673".parse().unwrap()),
        Some("ENSG00000157764".parse().unwrap()),
        "BRAF".parse().unwrap(),
    )
    .unwrap();
    assert_eq!(
        gene.ensembl_gene_id().unwrap().to_string(),
        "ENSG00000157764"
    );
    assert_eq!(gene.hgnc_id().unwrap().to_string(), "HGNC:1097");
    assert_eq!(gene.ncbi_gene_id().unwrap().to_string(), "673");
    assert!(
        GeneRef::new(
            None,
            None,
            Some("ENST00000288602".parse().unwrap()),
            "BRAF".parse().unwrap()
        )
        .is_err()
    );
    let accession = "NM_004333.4".parse().unwrap();
    assert!(
        Transcript::new(
            accession,
            gene.clone(),
            None,
            Some(Span0::new(10, 101).unwrap()),
            100,
            None,
        )
        .is_err()
    );
    let transcript = Transcript::new(
        accession,
        gene,
        Some("NP_004324.2".parse().unwrap()),
        Some(Span0::new(10, 100).unwrap()),
        100,
        None,
    )
    .unwrap();
    assert_eq!(transcript.gene().label().as_str(), "BRAF");
    assert_eq!(transcript.accession(), accession);
    assert_eq!(transcript.protein().unwrap().to_string(), "NP_004324.2");
    assert_eq!(transcript.coding(), Some(Span0::new(10, 100).unwrap()));
    assert_eq!(transcript.length(), 100);
    assert_eq!(transcript.mane(), None);
    let pair = Mane::new(
        ManeStatus::Select,
        accession,
        "ENST00000288602.11".parse().unwrap(),
    )
    .unwrap();
    assert!(
        Transcript::new(
            "NM_004333.5".parse().unwrap(),
            transcript.gene().clone(),
            None,
            None,
            100,
            Some(pair.clone()),
        )
        .is_err()
    );
    let with_mane = Transcript::new(
        pair.ensembl(),
        transcript.gene().clone(),
        None,
        None,
        100,
        Some(pair),
    )
    .unwrap();
    assert_eq!(with_mane.mane().unwrap().status(), ManeStatus::Select);
    for (error, code) in [
        (
            biodata::TranscriptError::CodingBounds,
            "transcript.coding_bounds",
        ),
        (biodata::TranscriptError::ManePair, "transcript.mane_pair"),
        (
            biodata::TranscriptError::ManeAccession,
            "transcript.mane_accession",
        ),
    ] {
        assert_eq!(error.code(), code);
        assert_eq!(error.to_string(), code);
    }
}

#[test]
fn lift_result_binds_target_allele_and_optional_source_release() {
    let chromosome = Chromosome::new(7).unwrap();
    let source = GenomicInterval::new(Build::GRCh37, chromosome, Span0::new(100, 101).unwrap())
        .with_strand(Strand::Plus);
    let target = GenomicInterval::new(Build::GRCh38, chromosome, Span0::new(200, 201).unwrap())
        .with_strand(Strand::Minus);
    let allele = Allele::new(
        SequenceLocation::new(
            chromosome.accession(Build::GRCh38),
            Span0::new(200, 201).unwrap(),
        ),
        "A".parse().unwrap(),
        "T".parse().unwrap(),
    )
    .unwrap();
    let mapped =
        Lifted::mapped(source, target, allele.clone(), Some("chain-release".into())).unwrap();
    assert_eq!(mapped.target_build(), Build::GRCh38);
    assert_eq!(mapped.source(), source);
    assert_eq!(mapped.chain_release(), Some("chain-release"));
    assert!(matches!(mapped.outcome(), LiftOutcome::Mapped { .. }));
    let missing =
        Lifted::unmapped(source, Build::GRCh38, LiftFailure::NoContiguousBlock, None).unwrap();
    assert_eq!(missing.chain_release(), None);
    assert!(matches!(missing.outcome(), LiftOutcome::Unmapped(_)));
    let without_strand =
        GenomicInterval::new(Build::GRCh37, chromosome, Span0::new(100, 101).unwrap());
    assert_eq!(
        Lifted::unmapped(
            without_strand,
            Build::GRCh38,
            LiftFailure::NoContiguousBlock,
            None
        ),
        Err(biodata::CoordinateError::StrandAbsent)
    );
    assert_eq!(
        biodata::CoordinateError::StrandAbsent.code(),
        "coord.interval.strand_absent"
    );
    assert_eq!(
        Lifted::mapped(without_strand, target, allele.clone(), None),
        Err(biodata::CoordinateError::StrandAbsent)
    );
    let other_chromosome_allele = Allele::new(
        SequenceLocation::new(
            Chromosome::new(8).unwrap().accession(Build::GRCh38),
            Span0::new(200, 201).unwrap(),
        ),
        "A".parse().unwrap(),
        "T".parse().unwrap(),
    )
    .unwrap();
    let mismatch = Lifted::mapped(source, target, other_chromosome_allele, None).unwrap_err();
    assert_eq!(
        mismatch,
        biodata::CoordinateError::AccessionChromosomeMismatch
    );
    assert_eq!(mismatch.code(), "coord.chromosome.mismatch");
    let span_mismatch = Lifted::mapped(
        source,
        GenomicInterval::new(Build::GRCh38, chromosome, Span0::new(201, 202).unwrap())
            .with_strand(Strand::Minus),
        allele,
        None,
    )
    .unwrap_err();
    assert_eq!(span_mismatch, biodata::CoordinateError::SpanMismatch);
    assert_eq!(span_mismatch.code(), "coord.span.mismatch");
}
