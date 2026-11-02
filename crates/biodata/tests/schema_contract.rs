//! Public parser boundaries agree with the shared value schemas.
#![cfg(feature = "schema")]
use proptest::{
    prelude::*,
    test_runner::{Config, TestRunner},
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

#[expect(
    clippy::expect_used,
    reason = "schema construction and generated contract failures fail the test"
)]
fn string_contract<T: DeserializeOwned + JsonSchema>(seeds: &[&str]) {
    let validator = jsonschema::validator_for(&schemars::schema_for!(T).to_value())
        .expect("the public schema compiles");
    let values: Vec<String> = seeds.iter().map(|value| (*value).to_owned()).collect();
    let strategy = prop_oneof![
        "[A-Za-z0-9:._/@ +?()=-]{0,80}",
        prop::sample::select(values)
    ];
    let mut runner = TestRunner::new(Config {
        cases: 96,
        ..Config::default()
    });
    runner
        .run(&strategy, |input| {
            let wire = Value::String(input);
            let parsed = serde_json::from_value::<T>(wire.clone()).is_ok();
            prop_assert_eq!(
                parsed,
                validator.is_valid(&wire),
                "public parser and schema disagree for {}",
                wire
            );
            Ok(())
        })
        .expect("the public string parser follows its schema");
}

macro_rules! strings {
    ($($test:ident: $ty:ty => [$($sample:literal),+];)+) => {$(
        #[test]
        fn $test() { string_contract::<$ty>(&[$($sample),+]); }
    )+};
}
strings! {
    accession: biodata::Accession => ["NM_000001.1", "ENST00000000001.1", "LRG_0001t01"];
    accession_base: biodata::AccessionBase => ["NM_000001", "ENSG00000000001", "LRG_0001t01"];
    base: biodata::Base => ["A", "N", "U", "a"];
    sequence: biodata::Sequence => ["", "ACGTN", "ACGU"];
    chromosome: biodata::Chromosome => ["1", "X", "MT", "01", "26"];
    build: biodata::Build => ["GRCh37", "GRCh38", "grch38"];
    strand: biodata::Strand => ["+", "-", "1"];
    contig: biodata::Contig => ["chr1", "source_scaffold", "source scaffold"];
    hgvs_text: biodata::HgvsText => ["NM_000001.1:c.1A>T", "NP_000001.1:p.?", "NM_000001.1:c.foo", "NP_000001.1:c.1A>T"];
    curie: biodata::Curie => ["TEST:1", "TEST:a:b", "TEST:/a"];
    direction: biodata::Direction => ["supports", "disputes", "neutral"];
    predicate: biodata::Predicate => ["is_causal_for", "predicts_sensitivity_to"];
    rs: biodata::RsId => ["rs1", "rs18446744073709551615", "rs18446744073709551616"];
    pa: biodata::PaId => ["PA1", "PA18446744073709551615", "PA18446744073709551616"];
    ca: biodata::CaId => ["CA000001", "CA18446744073709551615", "CA18446744073709551616"];
    hgnc: biodata::HgncId => ["HGNC:1", "HGNC:4294967295", "HGNC:4294967296"];
    ncbi_gene: biodata::NcbiGeneId => ["1", "4294967295", "4294967296"];
    pmid: biodata::Pmid => ["1", "4294967295", "4294967296"];
    omim: biodata::OmimId => ["100000", "699999", "099999", "700000", "999999"];
    variation: biodata::ClinVarVariationId => ["clinvar:1", "clinvar:4294967295", "clinvar:4294967296"];
    allele_id: biodata::ClinVarAlleleId => ["1", "4294967295", "4294967296", "clinvar-allele:1"];
    vcv: biodata::VcvId => ["VCV000000001", "VCV999999999.4294967295", "VCV000000001.4294967296", "VCV000000000", "VCV1000000000"];
    rcv: biodata::RcvId => ["RCV000000001", "RCV999999999.4294967295", "RCV000000001.4294967296"];
    scv: biodata::ScvId => ["SCV000000001", "SCV999999999.4294967295", "SCV000000001.4294967296"];
    cosmic: biodata::CosmicId => ["COSV1", "COSM18446744073709551615", "COSN18446744073709551616"];
    variant_id: biodata::VariantId => ["rs1", "CA000001", "PA1", "VCV000000001.1", "clinvar:1", "COSV1", "ga4gh:VA.abcdefghijklmnopqrstuvwxyzABCDEF", "VCV000000001.4294967296", "clinvar:4294967296"];
    uniprot_isoform: biodata::UniProtIsoform => ["P12345", "P12345-2", "P12345-0"];
    amino_acids: biodata::AminoAcids => ["ACDUOX", "B", "Z", "J", ""];
    acmg: biodata::AcmgAmp2015Code => ["pathogenic", "likely_pathogenic", "benign"];
    amp_tier: biodata::AmpAscoCap2017TierCode => ["tier_i", "tier_iv"];
    amp_level: biodata::AmpAscoCap2017LevelCode => ["level_a", "level_d"];
    civic_level: biodata::CivicEvidenceLevelCode => ["a", "e", "A"];
    clingen: biodata::ClingenCgcVicc2022Code => ["oncogenic", "benign"];
    low_penetrance: biodata::ClingenLowPenetrance2024Code => ["pathogenic_low_penetrance", "uncertain_risk_allele"];
    oncokb_level: biodata::OncokbLevelCode => ["level_1", "level_3a", "level_r1"];
    oncokb_oncogenicity: biodata::OncokbOncogenicityCode => ["oncogenic", "resistance", "unknown"];
    review_status: biodata::ReviewStatus => ["practice_guideline", "no_classification_provided", "practice guideline"];
    allele_origin: biodata::AlleleOrigin => ["germline", "de_novo", "de novo"];
    mane_status: biodata::ManeStatus => ["select", "plus_clinical"];
}

#[expect(
    clippy::expect_used,
    reason = "schema construction and generated contract failures fail the test"
)]
fn record_contract<T: DeserializeOwned + JsonSchema>(seed: Value) {
    let validator = jsonschema::validator_for(&schemars::schema_for!(T).to_value())
        .expect("the public schema compiles");
    assert!(
        validator.is_valid(&seed),
        "the contract seed follows the schema"
    );
    assert!(
        serde_json::from_value::<T>(seed.clone()).is_ok(),
        "the public parser accepts the contract seed"
    );
    let keys: Vec<String> = seed
        .as_object()
        .expect("a record seed is an object")
        .keys()
        .cloned()
        .collect();
    let mut runner = TestRunner::new(Config {
        cases: 48,
        ..Config::default()
    });
    runner
        .run(&(prop::sample::select(keys), 0u8..4), |(key, mutation)| {
            let mut wire = seed.clone();
            let fields = wire
                .as_object_mut()
                .expect("the contract seed is an object");
            match mutation {
                0 => {
                    fields.remove(&key);
                }
                1 => {
                    fields.insert(key, Value::Null);
                }
                2 => {
                    fields.insert(key, json!({"unexpected":true}));
                }
                _ => {
                    fields.insert("extra_property".into(), Value::Bool(true));
                }
            }
            prop_assert_eq!(
                serde_json::from_value::<T>(wire.clone()).is_ok(),
                validator.is_valid(&wire),
                "public record parser and schema disagree for {}",
                wire
            );
            Ok(())
        })
        .expect("the public record parser follows its schema");
}

macro_rules! record {
    ($test:ident, $ty:ty, $seed:expr) => {
        #[test]
        fn $test() {
            record_contract::<$ty>($seed);
        }
    };
}
record!(span, biodata::Span0, json!({"start":0,"end":1}));
record!(bound, biodata::Bound, json!({"min":0,"max":1}));
record!(
    uncertain_span,
    biodata::UncertainSpan0,
    json!({"start":{"min":0},"end":{"max":1}})
);
record!(
    sequence_location,
    biodata::SequenceLocation,
    json!({"sequence":"NM_000001.1","span":{"start":0,"end":1}})
);
record!(
    allele,
    biodata::Allele,
    json!({"location":{"sequence":"NM_000001.1","span":{"start":0,"end":1}},"reference":"A","alternate":"T"})
);
record!(
    interval,
    biodata::GenomicInterval,
    json!({"build":"GRCh38","chromosome":"1","span":{"start":0,"end":1}})
);
record!(
    gene_ref,
    biodata::SourceGeneRef,
    json!({"label":"source label","ensembl_gene_id":"ENSG00000000001.1"})
);
record!(
    concept,
    biodata::Concept,
    json!({"label":"source label","id":"TEST:1"})
);
record!(
    term,
    biodata::Term,
    json!({"label":"source label","codings":[]})
);
record!(
    expression,
    biodata::Expression,
    json!({"syntax":"hgvs.c","value":"NM_000001.1:c.1A>T"})
);
record!(
    representation,
    biodata::Representation,
    json!({"source":"source","expression":{"syntax":"hgvs.c","value":"NM_000001.1:c.1A>T"}})
);
record!(
    variant,
    biodata::Variant,
    json!({"ids":["rs1"],"genes":[{"label":"source label"}]})
);
record!(
    mane,
    biodata::Mane,
    json!({"status":"select","refseq":"NM_000001.1","ensembl":"ENST00000000001.1"})
);
record!(
    uniprot_feature,
    biodata::UniProtFeature,
    json!({"protein":"P12345","span":{"start":{"min":0},"end":{"max":1}},"kind":{"label":"source"}})
);

#[test]
fn partial_date_schema_differs_only_for_impossible_calendar_days() {
    let validator =
        jsonschema::validator_for(&schemars::schema_for!(biodata::PartialDate).to_value())
            .expect("the frozen date schema compiles");
    for year in [1_u16, 1900, 2000, 2023, 2024, 9999] {
        for month in 1..=12 {
            for day in 1..=31 {
                let wire = json!(format!("{year:04}-{month:02}-{day:02}"));
                let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
                let limit = match month {
                    2 => {
                        if leap {
                            29
                        } else {
                            28
                        }
                    }
                    4 | 6 | 9 | 11 => 30,
                    _ => 31,
                };
                assert!(
                    validator.is_valid(&wire),
                    "the frozen schema admits calendar-shaped days"
                );
                assert_eq!(
                    serde_json::from_value::<biodata::PartialDate>(wire).is_ok(),
                    day <= limit,
                    "the parser rejects only impossible calendar days"
                );
            }
        }
    }
    for text in ["2023-02-29", "2024-02-30"] {
        assert!(
            validator.is_valid(&json!(text)),
            "the frozen schema keeps the approved calendar exception"
        );
        assert!(
            text.parse::<biodata::PartialDate>().is_err(),
            "the parser rejects the impossible calendar day"
        );
    }
    for text in [
        "2024",
        "2024-02",
        "2024-02-29",
        "0000",
        "2024-00",
        "2024-13",
        "2024-01-00",
        "2024-01-32",
        "2024-1-1",
        "other",
    ] {
        assert_eq!(
            text.parse::<biodata::PartialDate>().is_ok(),
            validator.is_valid(&json!(text)),
            "date shape and precision agree outside the calendar exception"
        );
    }
}

#[expect(
    clippy::expect_used,
    reason = "schema construction and generated contract failures fail the test"
)]
fn numeric_contract<T: DeserializeOwned + JsonSchema>(values: &[Value]) {
    let validator = jsonschema::validator_for(&schemars::schema_for!(T).to_value())
        .expect("the frozen numeric schema compiles");
    for value in values {
        assert_eq!(
            serde_json::from_value::<T>(value.clone()).is_ok(),
            validator.is_valid(value),
            "numeric parser and frozen schema agree at {value}"
        );
    }
}

#[test]
fn numeric_shared_values_match_frozen_boundary_schemas() {
    numeric_contract::<biodata::Position1>(&[
        json!(-1),
        json!(0),
        json!(1),
        json!(4294967295_u64),
        json!(4294967296_u64),
        json!(1.5),
        json!("1"),
        Value::Null,
    ]);
    numeric_contract::<biodata::FiniteNumber>(&[
        json!(-1.25),
        json!(0),
        json!(1.25),
        json!(f64::MIN),
        json!(f64::MAX),
        json!("1"),
        Value::Null,
    ]);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            biodata::FiniteNumber::new(value).is_err(),
            "nonfinite values cannot enter the public numeric wire boundary"
        );
    }
}

#[test]
fn added_gene_shapes_keep_ensembl_identity_optional_and_unversioned() {
    let gene_schema = schemars::schema_for!(biodata::GeneRef).to_value();
    let subject_schema = schemars::schema_for!(biodata::Subject).to_value();
    let gene_validator = jsonschema::validator_for(&gene_schema).expect("the gene schema compiles");
    let subject_validator =
        jsonschema::validator_for(&subject_schema).expect("the subject schema compiles");
    for (id, accepted) in [
        (None, true),
        (Some(Value::Null), true),
        (Some(json!("ENSG00000000001")), true),
        (Some(json!("ENSG00000000001.1")), false),
        (Some(json!("ENST00000000001")), false),
    ] {
        let mut gene = json!({"label":"GENE"});
        if let Some(id) = id {
            gene["ensembl_gene_id"] = id;
        }
        assert_eq!(
            gene_validator.is_valid(&gene),
            accepted,
            "gene schema states its optional identifier"
        );
        let parsed = serde_json::from_value::<biodata::GeneRef>(gene.clone());
        assert_eq!(
            parsed.is_ok(),
            accepted,
            "gene parser agrees with its schema"
        );
        if let Ok(value) = parsed {
            let wire = serde_json::to_value(value).expect("the checked gene serializes");
            assert!(
                gene_validator.is_valid(&wire),
                "omitted identifiers remain valid after serialization"
            );
        }
        gene["level"] = json!("gene");
        assert_eq!(
            subject_validator.is_valid(&gene),
            accepted,
            "gene subject retains the same optional grammar"
        );
        assert_eq!(
            serde_json::from_value::<biodata::Subject>(gene).is_ok(),
            accepted,
            "gene subject parser agrees with its schema"
        );
    }
}

#[test]
fn transcript_references_use_one_string_schema() -> Result<(), Box<dyn std::error::Error>> {
    fn contract<T: DeserializeOwned + serde::Serialize + JsonSchema>()
    -> Result<(), Box<dyn std::error::Error>> {
        let schema = schemars::schema_for!(T).to_value();
        assert_eq!(
            schema["type"], "string",
            "one serialized string has one string schema"
        );
        assert_eq!(
            schema["pattern"],
            format!(
                "({}|{})",
                biodata::Accession::pattern(),
                biodata::AccessionBase::pattern()
            ),
            "both component patterns remain exact"
        );
        let validator = jsonschema::validator_for(&schema)?;
        for text in [
            "NM_000001.1",
            "NM_000001",
            "ENST00000000001.1",
            "ENST00000000001",
            "LRG_1t1",
        ] {
            let wire = json!(text);
            let value: T = serde_json::from_value(wire.clone())?;
            assert!(validator.is_valid(&wire), "valid component form {text}");
            assert_eq!(
                serde_json::to_value(value)?,
                wire,
                "serialized spelling stays unchanged"
            );
        }
        for text in ["NM_000001.", "NM_000001.65536", "ENST1", "LRG_1t", "other"] {
            let wire = json!(text);
            assert!(
                !validator.is_valid(&wire),
                "schema refuses component near miss {text}"
            );
            assert!(
                serde_json::from_value::<T>(wire).is_err(),
                "parser refuses component near miss {text}"
            );
        }
        Ok(())
    }
    contract::<biodata::TranscriptReference>()?;
    contract::<biodata::TranscriptId>()
}
