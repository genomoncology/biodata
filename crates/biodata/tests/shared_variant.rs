//! Shared variant values enforce their canonical public construction boundary.
use biodata::{Accession, Allele, Chromosome, Sequence, SequenceLocation, Span0, Strand};

#[test]
fn uncertain_boundaries_keep_partial_ranges_and_refuse_reversed_spans() {
    use biodata::{
        Bound, CopyChange, CopyNumberChange, CopyNumberCount, UncertainLocation, UncertainSpan0,
    };

    let start = Bound::new(Some(9), Some(11)).unwrap();
    let end = Bound::new(Some(15), None).unwrap();
    let span = UncertainSpan0::new(start, end).unwrap();
    assert_eq!(start.min(), Some(9));
    assert_eq!(start.max(), Some(11));
    assert_eq!(
        serde_json::to_value(span).unwrap(),
        serde_json::json!({
            "start": {"min": 9, "max": 11},
            "end": {"min": 15}
        })
    );
    assert_eq!(
        Bound::new(None, None).unwrap_err().code(),
        "coord.bound.empty"
    );
    assert_eq!(
        Bound::new(Some(12), Some(11)).unwrap_err().code(),
        "coord.bound.order"
    );
    assert_eq!(
        UncertainSpan0::new(
            Bound::new(Some(20), None).unwrap(),
            Bound::new(None, Some(15)).unwrap(),
        )
        .unwrap_err()
        .code(),
        "coord.span.order"
    );
    let count = CopyNumberCount {
        location: UncertainLocation {
            sequence: "NC_000007.14".parse().unwrap(),
            span,
        },
        copies: Bound::new(Some(3), Some(5)).unwrap(),
    };
    assert_eq!(
        serde_json::to_value(count).unwrap(),
        serde_json::json!({
            "location": {
                "sequence": "NC_000007.14",
                "span": {"start": {"min": 9, "max": 11}, "end": {"min": 15}}
            },
            "copies": {"min": 3, "max": 5}
        })
    );
    let change = CopyNumberChange {
        location: count.location,
        copy_change: CopyChange::Gain,
    };
    assert_eq!(change.copy_change.efo_code(), "EFO:0030070");
    assert_eq!(serde_json::to_value(change).unwrap()["copy_change"], "gain");
}

#[test]
fn canonical_chromosomes_and_source_names_have_distinct_boundaries() {
    for text in ["1", "22", "X", "Y", "MT"] {
        let chromosome: Chromosome = text.parse().unwrap();
        assert_eq!(chromosome.to_string(), text);
    }
    for text in ["chr1", "M", "23", "01", "0", "26"] {
        assert_eq!(
            text.parse::<Chromosome>().unwrap_err().code(),
            "coord.chromosome.invalid"
        );
    }
    assert_eq!(
        "GRCh39".parse::<biodata::Build>().unwrap_err().code(),
        "coord.build.unknown"
    );
    assert_eq!(
        "".parse::<biodata::Contig>().unwrap_err().code(),
        "coord.contig.invalid"
    );
    for (source, canonical) in [("chr1", "1"), ("chr01", "1"), ("M", "MT"), ("23", "X")] {
        assert_eq!(
            biodata::parse_chromosome(source).unwrap().to_string(),
            canonical
        );
    }
    assert_eq!(serde_json::to_string(&Strand::Plus).unwrap(), "\"+\"");
    assert_eq!(serde_json::to_string(&Strand::Minus).unwrap(), "\"-\"");
}

#[test]
fn shared_alleles_keep_interbase_empty_sides_and_reference_states() {
    let accession: Accession = "NC_000007.14".parse().unwrap();
    for (start, end, reference, alternate, valid) in [
        (10, 10, "", "A", true),
        (10, 11, "A", "", true),
        (10, 11, "A", "A", true),
        (10, 10, "", "", false),
        (10, 12, "A", "T", false),
    ] {
        let location = SequenceLocation::new(accession, Span0::new(start, end).unwrap());
        let allele = Allele::new(
            location,
            reference.parse().unwrap(),
            alternate.parse().unwrap(),
        );
        assert_eq!(allele.is_ok(), valid, "{reference}:{alternate}");
        if let Ok(allele) = allele {
            let wire = serde_json::to_value(&allele).unwrap();
            assert_eq!(wire["location"]["sequence"], "NC_000007.14");
            assert_eq!(wire["reference"], reference);
            assert_eq!(serde_json::from_value::<Allele>(wire).unwrap(), allele);
        }
    }
    assert!("U".parse::<Sequence>().is_err());
    assert!("NC_000007.014".parse::<Accession>().is_err());
    assert!("NC_000007.0".parse::<Accession>().is_err());
    assert!(serde_json::from_str::<Allele>(r#"{"location":{"sequence":"NC_000007.14","span":{"start":0,"end":2}},"reference":"A","alternate":"T"}"#).is_err());
}

#[test]
fn accession_component_construction_uses_the_canonical_grammar() {
    use biodata::{EnsemblFeature, RefSeqPrefix};
    let accession = Accession::refseq(RefSeqPrefix::Nc, 7, 14).unwrap();
    assert_eq!(accession.to_string(), "NC_000007.14");
    assert_eq!(accession.refseq_parts(), Some((RefSeqPrefix::Nc, 7, 14)));
    assert_eq!(accession.ensembl_parts(), None);
    assert_eq!(accession.version(), Some(14));
    for (number, version) in [(999_999, 1), (1_000_000, u16::MAX), (999_999_999, 1)] {
        let text = format!(
            "NC_{number:0width$}.{version}",
            width = if number < 1_000_000 { 6 } else { 9 }
        );
        assert_eq!(
            Accession::refseq(RefSeqPrefix::Nc, number, version),
            text.parse()
        );
    }
    for (number, version) in [(7, 0), (1_000_000_000, 1)] {
        let text = format!(
            "NC_{number:0width$}.{version}",
            width = if number < 1_000_000 { 6 } else { 9 }
        );
        assert_eq!(
            Accession::refseq(RefSeqPrefix::Nc, number, version),
            text.parse()
        );
    }
    let ensembl = Accession::ensembl(EnsemblFeature::Transcript, 288602, 11).unwrap();
    assert_eq!(ensembl.to_string(), "ENST00000288602.11");
    assert_eq!(
        ensembl.ensembl_parts(),
        Some((EnsemblFeature::Transcript, 288602, 11))
    );
    assert_eq!(ensembl.refseq_parts(), None);
    for (number, version) in [
        (0, 1),
        (99_999_999_999, u16::MAX),
        (288602, 0),
        (100_000_000_000, 1),
    ] {
        let text = format!("ENST{number:011}.{version}");
        assert_eq!(
            Accession::ensembl(EnsemblFeature::Transcript, number, version),
            text.parse()
        );
    }
    assert_eq!("LRG_199t1".parse::<Accession>().unwrap().version(), None);
}

#[test]
fn allele_keys_keep_builds_separate_and_refuse_nonprimary_interval_views() {
    use biodata::{AlleleKey, Build};
    let make = |sequence: &str| {
        Allele::new(
            SequenceLocation::new(sequence.parse().unwrap(), Span0::new(3242, 3243).unwrap()),
            "A".parse().unwrap(),
            "G".parse().unwrap(),
        )
        .unwrap()
    };
    let allele = make("NC_012920.1");
    let first = AlleleKey::from_allele(Build::GRCh37, allele.clone());
    let second = AlleleKey::from_allele(Build::GRCh38, allele);
    assert_ne!(first, second);
    assert_eq!(first.allele(), second.allele());
    assert_eq!(first.interval().unwrap().chromosome().to_string(), "MT");
    assert!(
        AlleleKey::from_allele(Build::GRCh38, make("NM_004333.4"))
            .interval()
            .is_err()
    );
}

#[test]
fn interval_containment_requires_both_explicit_builds() {
    use biodata::{Build, CoordinateError, GenomicInterval};
    let interval = GenomicInterval {
        build: None,
        chromosome: "7".parse().unwrap(),
        span: Span0::new(0, 10).unwrap(),
        strand: None,
    };
    let explicit = GenomicInterval {
        build: Some(Build::GRCh38),
        ..interval
    };
    for (outer, inner) in [
        (interval, explicit),
        (explicit, interval),
        (interval, interval),
    ] {
        assert_eq!(outer.contains(inner), Err(CoordinateError::BuildAbsent));
    }
    assert!(explicit.contains(explicit).unwrap());
    assert_eq!(
        serde_json::to_value(interval).unwrap(),
        serde_json::json!({"chromosome":"7","span":{"start":0,"end":10}})
    );
    assert!(
        serde_json::from_value::<GenomicInterval>(
            serde_json::json!({"chromosome":"23","span":{"start":0,"end":10}})
        )
        .is_err()
    );
}

#[test]
fn dna_values_refuse_uracil_while_rna_grammar_keeps_its_base() {
    use biodata::{
        Base, Contig, Outcome,
        hgvs::{Change, NaBody, NaEdit, Nucleotide},
    };
    assert_eq!("U".parse::<Base>().unwrap_err().code(), "sequence.base.rna");
    assert_eq!(
        "x".parse::<Sequence>().unwrap_err().code(),
        "sequence.base.invalid"
    );
    assert_eq!("".parse::<Sequence>().unwrap().len(), 0);
    assert_eq!(Sequence::new(vec![Base::A]).as_slice(), &[Base::A]);
    for text in ["", "AC", "x"] {
        assert!(text.parse::<Base>().is_err());
    }
    assert_eq!(serde_json::from_str::<Base>("\"A\"").unwrap(), Base::A);
    assert!(serde_json::from_str::<Sequence>("\"U\"").is_err());
    let Outcome::Parsed(value) = biodata::hgvs::parse("r.17a>u").outcome else {
        panic!("existing public RNA fixture must parse")
    };
    assert_eq!(value.to_string(), "r.17a>u");
    let Change::Nucleotide(change) = value.change else {
        panic!("expected nucleotide edit")
    };
    let NaBody::Edit {
        edit: NaEdit::Substitution { from, to },
        ..
    } = change.body
    else {
        panic!("expected substitution")
    };
    assert!(matches!(from, Nucleotide::Rna(_)));
    assert!(matches!(to, Nucleotide::Rna(_)));
    assert!(to.dna().is_err());
    for source in ["chr7", "GL000220.1"] {
        let contig: Contig = source.parse().unwrap();
        assert_eq!(contig.as_str(), source);
        assert_eq!(contig.to_string(), source);
        assert_eq!(
            serde_json::from_value::<Contig>(serde_json::to_value(&contig).unwrap()).unwrap(),
            contig
        );
    }
    for source in ["", "chr 7", "x\n", "é"] {
        assert!(source.parse::<Contig>().is_err());
    }
    assert!("x".repeat(256).parse::<Contig>().is_err());
}

#[test]
fn accession_bases_keep_version_identity_without_constructing_a_version() {
    use biodata::{AccessionBase, EnsemblFeature, RefSeqPrefix};
    let old: Accession = "NM_004333.4".parse().unwrap();
    let new: Accession = "NM_004333.6".parse().unwrap();
    assert_eq!(old.base(), new.base());
    assert_eq!(old.base().to_string(), "NM_004333");
    for source in [
        "NM_004333",
        "NC_000007",
        "NC_001000000",
        "ENSG00000141510",
        "LRG_199t1",
    ] {
        let base: AccessionBase = source.parse().unwrap();
        assert_eq!(base.to_string(), source);
    }
    assert_eq!("NM_004333".parse::<AccessionBase>().unwrap(), old.base());
    assert_eq!(
        AccessionBase::refseq(RefSeqPrefix::Nm, 4333).unwrap(),
        old.base()
    );
    assert_eq!(
        AccessionBase::ensembl(EnsemblFeature::Gene, 141_510)
            .unwrap()
            .to_string(),
        "ENSG00000141510"
    );
    assert!(AccessionBase::refseq(RefSeqPrefix::Nc, 1_000_000_000).is_err());
    assert!(AccessionBase::ensembl(EnsemblFeature::Gene, 100_000_000_000).is_err());
    for (source, code) in [
        ("NM_00433", "accession.digits.width"),
        ("NC_1000000", "accession.digits.width"),
        ("ENSG0000014151", "accession.digits.width"),
        ("NM_004333.4", "accession.version.present"),
        ("ENSG00000141510.1", "accession.version.present"),
        ("LRG_1.1", "accession.version.present"),
        ("bad", "accession.prefix.unknown"),
        ("LRG_1tX", "accession.lrg.invalid"),
        ("LRG_1pX", "accession.lrg.invalid"),
        ("LRG_1t65536", "accession.lrg.invalid"),
        ("LRG_1p65536", "accession.lrg.invalid"),
        ("LRG_1x", "accession.lrg.invalid"),
    ] {
        assert_eq!(
            source.parse::<AccessionBase>().unwrap_err().code.name(),
            code
        );
    }
    for (source, code) in [
        ("NM_00433.4", "accession.digits.width"),
        ("NM_004333", "accession.version.missing"),
        ("NM_004333.04", "accession.version.leading_zero"),
        ("NM_004333.999999", "accession.version.overflow"),
        ("LRG_x", "accession.lrg.invalid"),
        ("LRG_1tX", "accession.lrg.invalid"),
        ("LRG_1pX", "accession.lrg.invalid"),
        ("LRG_1t65536", "accession.lrg.invalid"),
        ("LRG_1p65536", "accession.lrg.invalid"),
        ("LRG_1x", "accession.lrg.invalid"),
    ] {
        assert_eq!(source.parse::<Accession>().unwrap_err().code.name(), code);
    }
    assert_eq!(
        "ENST00000288602.11"
            .parse::<Accession>()
            .unwrap()
            .base()
            .to_string(),
        "ENST00000288602"
    );
    assert_eq!(
        "LRG_199t1".parse::<Accession>().unwrap().base().to_string(),
        "LRG_199t1"
    );
    let base: AccessionBase = serde_json::from_str("\"ENSG00000141510\"").unwrap();
    assert_eq!(serde_json::to_string(&base).unwrap(), "\"ENSG00000141510\"");
    assert!(serde_json::from_str::<AccessionBase>("\"ENSG00000141510.1\"").is_err());
}

#[test]
fn shared_public_allele_fixtures_round_trip_without_changing_source_claims() {
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/variant/common.json")).unwrap();
    for case in cases {
        if let Some(wire) = case.get("allele") {
            let allele: Allele = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(
                serde_json::to_value(&allele).unwrap(),
                *wire,
                "{}",
                case["name"]
            );
            for expression in case["expressions"].as_array().unwrap() {
                if expression["syntax"] == "spdi" {
                    let location = allele.location();
                    assert_eq!(
                        expression["value"],
                        format!(
                            "{}:{}:{}:{}",
                            location.sequence(),
                            location.span().start(),
                            allele.reference(),
                            allele.alternate()
                        )
                    );
                }
            }
        }
    }
}
