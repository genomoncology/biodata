//! Synthetic merge regressions through the public value parsers.
use biodata::{
    AlleleKey, Base, Build, Chromosome, GenomicInterval, Outcome, Span0, UncertainLocation,
};
use serde_json::json;

#[test]
fn alteration_prefix_obeys_the_gene_symbol_limit() {
    for (length, accepted) in [(255, true), (256, false), (512, false)] {
        let source = format!("{} amplification", "A".repeat(length));
        let parsed = matches!(
            biodata::alteration::parse(&source).outcome,
            Outcome::Parsed(_)
        );
        assert_eq!(
            parsed, accepted,
            "the public alteration parser keeps the checked symbol bound"
        );
    }
}

#[test]
fn second_only_coordinate_records_refuse_unknown_fields() {
    let interval = GenomicInterval::new(
        Build::GRCh38,
        Chromosome::new(1).expect("a primary chromosome"),
        Span0::new(0, 1).expect("an ordered span"),
    );
    let value =
        AlleleKey::new(interval, vec![Base::A], vec![Base::T]).expect("a stated replacement");
    let mut wire = serde_json::to_value(value).expect("the checked allele serializes");
    wire["unexpected"] = json!(true);
    assert!(
        serde_json::from_value::<AlleleKey>(wire).is_err(),
        "the allele key keeps its closed input shape"
    );
    let wire = json!({"sequence":"NM_000001.1","span":{"start":{"min":0},"end":{"max":1}},"unexpected":true});
    assert!(
        serde_json::from_value::<UncertainLocation>(wire).is_err(),
        "the uncertain location keeps its closed input shape"
    );
}

#[test]
fn compact_accession_bases_canonicalize_only_variable_width_parts() {
    for (source, expected) in [
        ("LRG_0001t01", "LRG_1t1"),
        ("NM_000546", "NM_000546"),
        ("ENSG00000000001", "ENSG00000000001"),
    ] {
        let value: biodata::AccessionBase = source
            .parse()
            .expect("the public accession base grammar accepts the spelling");
        assert_eq!(
            value.to_string(),
            expected,
            "fixed-width accession digits keep their zeros"
        );
    }
}

#[test]
fn clinvar_accessions_offer_version_independent_identity_explicitly() {
    let first: biodata::VcvId = "VCV000000001.1".parse().expect("a checked accession");
    let second: biodata::VcvId = "VCV000000001.2".parse().expect("a checked accession");
    assert_ne!(
        first, second,
        "ordinary equality retains the supplied version"
    );
    assert!(
        first.same_accession(second),
        "explicit accession identity ignores only the version"
    );
}
