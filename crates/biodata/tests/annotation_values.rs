//! Public annotation values retain source distinctions across callers.
use biodata::{
    Accession, AlleleKey, AlleleOrigin, AminoAcids, Attribution, Base, Bound, Build, Chromosome,
    ClassificationType, ClinVarValueError, Coding, Consequence, ConsequenceError, FrequencyError,
    GeneExtent, GeneRef, GeneSymbol, GenomicInterval, NmdEscape, NmdEscapeReason, NmdRuleId,
    PopulationFrequency, Residue, ReviewStatus, SoCode, SoTerm, Span0, Strand, Term, Text,
    TranscriptAccession, TranscriptConsequence, UncertainSpan0, UniProtFeature, UniProtIsoform,
    VariantRef,
    hgvs::ProteinChange,
    peptide::{Frame, PeptideError, difference, replacement},
};

#[test]
fn legacy_consequence_keeps_allele_identity_and_deduplicates_so_terms() {
    let transcript = TranscriptAccession::new("NM_004333.4".parse().unwrap()).unwrap();
    let allele = AlleleKey::new(
        GenomicInterval::new(
            Build::GRCh38,
            Chromosome::new(7).unwrap(),
            Span0::new(10, 11).unwrap(),
        ),
        vec![Base::A],
        vec![Base::T],
    )
    .unwrap();
    let result = Consequence::new(
        allele.clone(),
        transcript,
        vec![SoTerm::Missense, SoTerm::SpliceRegion, SoTerm::Missense],
    );
    assert_eq!(result.allele, allele);
    assert_eq!(result.transcript, transcript);
    assert_eq!(result.terms().len(), 2);
}

#[test]
fn consequence_retains_distinct_source_terms_on_one_transcript() {
    let transcript = TranscriptAccession::new("NM_004333.4".parse().unwrap()).unwrap();
    let terms = vec![
        Coding::So(SoCode::Consequence(SoTerm::Missense)),
        Coding::So(SoCode::Consequence(SoTerm::SpliceRegion)),
    ];
    let source =
        serde_json::from_value::<Attribution>(serde_json::json!({"source":"VEP"})).unwrap();
    let result = TranscriptConsequence::new(
        transcript.accession(),
        None,
        terms.clone(),
        None,
        None,
        None,
        source.clone(),
    )
    .unwrap();
    assert_eq!(result.transcript(), transcript.accession());
    assert_eq!(result.consequences(), terms);
    assert_eq!(result.attribution(), &source);
    assert_eq!(
        TranscriptConsequence::new(
            transcript.accession(),
            None,
            vec![terms[0], terms[0]],
            None,
            None,
            None,
            source
        )
        .unwrap_err(),
        ConsequenceError::Duplicate
    );
    assert_eq!(
        (SoTerm::Missense.system(), SoTerm::Missense.code()),
        ("so", "SO:0001583")
    );
    let rule = NmdRuleId::new("local-policy").unwrap();
    let escape = NmdEscape::new(rule, NmdEscapeReason::LastExon);
    assert_eq!(escape.rule().as_str(), "local-policy");
    assert_eq!(escape.reason().code(), "last_exon");
    assert!(NmdRuleId::new("").is_none());
}

#[test]
fn sequence_ontology_codes_keep_their_published_identity() {
    for (term, code) in [
        (SoTerm::Synonymous, "SO:0001819"),
        (SoTerm::ThreePrimeUtr, "SO:0001624"),
        (SoTerm::SpliceAcceptor, "SO:0001574"),
        (SoTerm::SpliceDonor, "SO:0001575"),
        (SoTerm::SpliceRegion, "SO:0001630"),
        (SoTerm::Intron, "SO:0001627"),
        (SoTerm::FivePrimeUtr, "SO:0001623"),
        (SoTerm::StopGained, "SO:0001587"),
        (SoTerm::Missense, "SO:0001583"),
        (SoTerm::InframeDeletion, "SO:0001822"),
        (SoTerm::InframeInsertion, "SO:0001821"),
        (SoTerm::Frameshift, "SO:0001589"),
        (SoTerm::StopLost, "SO:0001578"),
        (SoTerm::NoncodingTranscript, "SO:0001619"),
    ] {
        assert_eq!((term.system(), term.code()), ("so", code));
    }
}

#[test]
fn consequence_refuses_nontranscript_accessions() {
    for accepted in [
        "NM_004333.4",
        "NR_123456.1",
        "XM_123456.1",
        "XR_123456.1",
        "ENST00000288602.11",
        "LRG_1t1",
    ] {
        let accession: Accession = accepted.parse().unwrap();
        assert!(TranscriptAccession::new(accession).is_some(), "{accepted}");
    }
    for refused in [
        "NC_000007.14",
        "NP_004324.2",
        "ENSG00000157764.14",
        "ENSP00000288602.6",
        "LRG_1",
        "LRG_1p1",
    ] {
        let accession: Accession = refused.parse().unwrap();
        assert!(TranscriptAccession::new(accession).is_none(), "{refused}");
    }
}

#[test]
fn nmd_reason_codes_match_serialized_values() {
    for reason in [
        NmdEscapeReason::LastExon,
        NmdEscapeReason::NearLastExonJunction,
        NmdEscapeReason::NearCodingStartSite,
        NmdEscapeReason::LongExon,
    ] {
        assert_eq!(
            serde_json::to_string(&reason).unwrap(),
            format!("\"{}\"", reason.code())
        );
    }
}

#[test]
fn gene_extent_keeps_gene_identity_with_its_reference_bounds() {
    let gene = GeneRef::new(None, None, None, "TEST1".parse::<GeneSymbol>().unwrap()).unwrap();
    let interval = GenomicInterval::new(
        Build::GRCh38,
        Chromosome::new(7).unwrap(),
        Span0::new(100, 200).unwrap(),
    )
    .with_strand(Strand::Minus);
    let extent = GeneExtent { gene, interval };
    assert_eq!(extent.gene.label().as_str(), "TEST1");
    assert_eq!(extent.interval.build(), Ok(Build::GRCh38));
    assert_eq!(extent.interval.strand, Some(Strand::Minus));
}

#[test]
fn frequency_keeps_source_strata_and_checks_published_numbers() {
    let mut wire = serde_json::json!({"subject":VariantRef::allele_hgvs("NC_000007.14:g.140753336A>T".parse().unwrap()),"build":"GRCh38","dataset":"gnomad","dataset_version":"4.1","attribution":{"source":"gnomad"},"data_type":"exomes","ancestry_group":"afr","sex":"xx","subset":"non_ukb","allele_count":2,"allele_number":100,"hemizygote_count":0,"allele_frequency":0.02,"faf95":0.01});
    assert!(
        serde_json::from_value::<PopulationFrequency>(wire.clone()).is_ok(),
        "checked frequency accepts the source stratum"
    );
    wire["allele_count"] = serde_json::json!(101);
    assert!(
        serde_json::from_value::<PopulationFrequency>(wire.clone())
            .unwrap_err()
            .to_string()
            .contains("population.count.exceeds_number"),
        "counts cannot exceed their source denominator"
    );
    wire["allele_count"] = serde_json::json!(2);
    wire["faf95"] = serde_json::json!(1.01);
    assert!(
        serde_json::from_value::<PopulationFrequency>(wire)
            .unwrap_err()
            .to_string()
            .contains("population.frequency.range"),
        "source fractions stay within the unit interval"
    );
    assert_eq!(
        FrequencyError::InvalidFraction.code(),
        "population.frequency.range",
        "the retained frequency error code stays stable"
    );
}

#[test]
fn clinvar_vocabulary_keeps_aggregate_and_submission_review_statuses() {
    assert_eq!(
        "criteria provided, multiple submitters, no conflicts".parse::<ReviewStatus>(),
        Ok(ReviewStatus::CriteriaMultipleSubmittersNoConflicts)
    );
    assert_eq!(
        "criteria provided, single submitter".parse::<ReviewStatus>(),
        Ok(ReviewStatus::CriteriaSingleSubmitter)
    );
    assert!(
        "criteria provided, three submitters"
            .parse::<ReviewStatus>()
            .is_err()
    );
    assert_ne!(ClassificationType::Somatic, ClassificationType::Germline);
    assert_ne!(AlleleOrigin::Somatic, AlleleOrigin::Germline);
    assert_eq!(
        "uniparental".parse::<AlleleOrigin>(),
        Ok(AlleleOrigin::Uniparental)
    );
    for (text, origin) in [
        ("not provided", AlleleOrigin::NotProvided),
        ("unknown", AlleleOrigin::Unknown),
    ] {
        assert_eq!(text.parse::<AlleleOrigin>(), Ok(origin));
    }
    assert_eq!(
        "criteria provided, three submitters"
            .parse::<ReviewStatus>()
            .unwrap_err()
            .to_string(),
        ClinVarValueError::ReviewStatus.code()
    );
    assert_eq!(
        "other".parse::<AlleleOrigin>().unwrap_err().to_string(),
        ClinVarValueError::AlleleOrigin.code()
    );
}

#[test]
fn uniprot_feature_retains_sequence_and_uncertain_bounds() {
    let protein = UniProtIsoform::new("P15056-2").unwrap();
    let feature = UniProtFeature {
        protein: protein.clone(),
        span: UncertainSpan0::new(
            Bound::new(None, Some(10)).unwrap(),
            Bound::new(Some(10), None).unwrap(),
        )
        .unwrap(),
        kind: Term {
            label: Text::new("domain").unwrap(),
            codings: Box::new([]),
        },
        description: Some(Text::new("kinase").unwrap()),
        id: Some(Text::new("PRO_001").unwrap()),
        original: Some(AminoAcids::new("V").unwrap()),
        alternative: vec![AminoAcids::new("E").unwrap()],
        evidence: vec![Text::new("ECO:0000269").unwrap()],
    };
    assert_eq!(feature.span.start().max(), Some(10));
    assert_eq!(feature.kind.label.as_str(), "domain");
    assert_eq!(feature.evidence[0].as_str(), "ECO:0000269");
    assert_eq!(feature.protein.as_str(), "P15056-2");
    assert_eq!(feature.original.as_ref().unwrap().as_str(), "V");
    assert_eq!(feature.alternative[0].as_str(), "E");
    let wire = serde_json::to_value(&feature).unwrap();
    assert_eq!(wire["original"], "V");
    assert_eq!(wire["alternative"], serde_json::json!(["E"]));
    assert_eq!(
        serde_json::from_value::<UniProtFeature>(wire).unwrap(),
        feature
    );
    for invalid in ["", "v", "B", "*"] {
        assert_eq!(
            AminoAcids::new(invalid).unwrap_err().code(),
            "protein.residue.invalid"
        );
    }
    for accession in ["A2BC19", "A0A023GPI8-2"] {
        assert!(UniProtIsoform::new(accession).is_some(), "{accession}");
    }
    for invalid in ["VSP_018632", "P15056-0", "P15056-01", "P15056-2-3"] {
        assert!(UniProtIsoform::new(invalid).is_none(), "{invalid}");
    }
}

#[test]
fn peptide_difference_describes_a_substitution_and_terminal_extension() {
    let original = [Residue::Ala, Residue::Lys, Residue::Ter];
    let substitution = [Residue::Ala, Residue::Arg, Residue::Ter];
    let change = difference(&original, &substitution, 1..2, 3, Frame::Preserved).unwrap();
    assert_eq!(change.to_string(), "Lys2Arg");
    let extension = [Residue::Ala, Residue::Lys, Residue::Gln, Residue::Ter];
    let change = difference(&original, &extension, 2..3, 4, Frame::Preserved).unwrap();
    assert_eq!(change.to_string(), "Ter3GlnextTer1");
}

#[test]
fn peptide_difference_uses_readable_stop_context_beyond_the_coding_window() {
    let original = [Residue::Ala, Residue::Lys, Residue::Ter];
    let extended = [Residue::Ala, Residue::Arg, Residue::Gln, Residue::Ter];
    assert!(matches!(
        difference(&original, &extended, 1..2, 3, Frame::Preserved),
        Ok(ProteinChange::Delins { .. })
    ));
    assert_eq!(
        difference(&original, &extended[..3], 1..2, 3, Frame::Preserved),
        Err(PeptideError::Unsupported)
    );
}

#[test]
fn peptide_replacement_refuses_invalid_public_spans() {
    let original = [Residue::Ala, Residue::Lys];
    for (first, end, inserted) in [
        (0, 0, vec![Residue::Lys]),
        (0, 0, Vec::new()),
        (usize::MAX, 0, vec![Residue::Lys]),
        (1, 0, Vec::new()),
        (3, 3, vec![Residue::Lys]),
        (1, 1, Vec::new()),
    ] {
        assert_eq!(
            replacement(&original, first, end, inserted),
            Err(PeptideError::Input),
            "{first}..{end}"
        );
    }
    assert_eq!(PeptideError::Input.to_string(), "peptide.input");
    assert_eq!(
        PeptideError::Unsupported.code(),
        "peptide.context.unsupported"
    );
    assert!(matches!(
        replacement(&original, 1, 2, Vec::new()),
        Ok(ProteinChange::Deletion(_))
    ));
}
