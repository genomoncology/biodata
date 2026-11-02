//! Shared source records keep the publisher's subject, labels and stated counts.
use biodata::{
    Attribution, Build, CaId, Chromosome, CohortObservation, CohortUnit, Curie, FiniteNumber,
    GeneRef, GeneSymbol, GenomicInterval, Method, Pmid, PopulationFrequency, Predicate,
    PredictorScore, Proposition, Span0, Statement, Subject, Term, Text, TranscriptId, VariantId,
    VariantRef,
};

#[expect(
    clippy::unwrap_used,
    reason = "construct a fixed gene fixture for outside-in tests"
)]
fn gene() -> GeneRef {
    GeneRef::new(None, None, None, "BRAF".parse::<GeneSymbol>().unwrap()).unwrap()
}

#[test]
fn subject_keeps_each_source_level_on_the_wire() {
    let variant = Subject::from(VariantRef::protein(
        source_gene(),
        Text::new("V600E").unwrap(),
    ));
    let gene = Subject::gene(gene());
    let region = Subject::region(
        GenomicInterval::new(
            Build::GRCh38,
            Chromosome::new(7).unwrap(),
            Span0::new(10, 20).unwrap(),
        ),
        Text::new("dosage region").unwrap(),
        None,
    )
    .unwrap();
    for (subject, level) in [(variant, "protein"), (gene, "gene"), (region, "region")] {
        let wire = serde_json::to_value(&subject).unwrap();
        assert_eq!(wire["level"], level);
        assert_eq!(serde_json::from_value::<Subject>(wire).unwrap(), subject);
    }
    let absent_build = serde_json::json!({"level":"region", "interval": {"chromosome":"7", "span":{"start":10,"end":20}}, "label":"dosage region"});
    assert!(serde_json::from_value::<Subject>(absent_build).is_err());
}

#[test]
fn source_claim_and_counts_keep_stated_qualifiers() {
    let mut attribution =
        serde_json::from_value::<Attribution>(serde_json::json!({"source":"ClinGen"})).unwrap();
    attribution.approved = Some("2024-03-01".parse().unwrap());
    attribution.published = Some("2024-04".parse().unwrap());
    let mut statement = Statement::new(
        Subject::gene(gene()),
        Proposition::GeneDiseaseValidity,
        Term {
            label: Text::new("Definitive").unwrap(),
            codings: Box::new([]),
        },
        attribution,
    );
    statement.method = Some(Method {
        label: Text::new("SOP").unwrap(),
        version: Some(Text::new("11").unwrap()),
    });
    statement.criteria_not_met = Box::new([Text::new("criterion B").unwrap()]);
    statement.mode_of_inheritance = Some(Term {
        label: Text::new("autosomal dominant").unwrap(),
        codings: Box::new([]),
    });
    statement.status = Some(Term {
        label: Text::new("flagged").unwrap(),
        codings: Box::new([]),
    });
    statement.predicate = Some(Predicate::IsCausalFor);
    statement.citations = Box::new([Pmid::new(123456).unwrap()]);
    statement.condition = Some(biodata::Concept {
        id: Some("MONDO:0005105".parse().unwrap()),
        label: Text::new("condition").unwrap(),
    });
    let wire = serde_json::to_value(&statement).unwrap();
    assert_eq!(wire["subject"]["level"], "gene");
    assert_eq!(wire["method"]["version"], "11");
    assert_eq!(wire["attribution"]["approved"], "2024-03-01");
    assert_eq!(wire["criteria_not_met"], serde_json::json!(["criterion B"]));
    assert_eq!(wire["predicate"], "is_causal_for");
    assert_eq!(wire["citations"], serde_json::json!(["123456"]));
    assert_eq!(wire["condition"]["id"], "MONDO:0005105");
    assert_eq!(
        serde_json::from_value::<Statement>(wire).unwrap(),
        statement
    );

    let cohort = CohortObservation {
        subject: VariantRef::allele_hgvs("NC_000007.14:g.140753336A>T".parse().unwrap()),
        dataset: Text::new("genie").unwrap(),
        dataset_version: Some(Text::new("17").unwrap()),
        unit: CohortUnit::Samples,
        sample_count: 7,
        profiled_sample_count: Some(100),
        cohort_sample_count: Some(1000),
        by_condition: Vec::new(),
        attribution: serde_json::from_value::<Attribution>(serde_json::json!({"source":"GENIE"}))
            .unwrap(),
    };
    let wire = serde_json::to_value(&cohort).unwrap();
    assert_eq!(wire["sample_count"], 7);
    assert_eq!(wire["profiled_sample_count"], 100);
    assert_eq!(wire["cohort_sample_count"], 1000);
    assert!(wire.get("frequency").is_none());
    assert_eq!(
        serde_json::from_value::<CohortObservation>(wire).unwrap(),
        cohort
    );
}

#[test]
fn score_and_population_keep_source_qualifiers() {
    let subject = VariantRef::allele_hgvs("NC_000007.14:g.140753336A>T".parse().unwrap());
    let score = PredictorScore {
        subject: subject.clone(),
        method: Text::new("REVEL").unwrap(),
        score: FiniteNumber::new(0.72).unwrap(),
        label: None,
        gene: Some(source_gene()),
        transcript: Some(TranscriptId::Base("NM_004333".parse().unwrap())),
        attribution: serde_json::from_value::<Attribution>(serde_json::json!({"source":"REVEL"}))
            .unwrap(),
    };
    let wire = serde_json::to_value(&score).unwrap();
    assert_eq!(wire["gene"]["label"], "BRAF");
    assert_eq!(wire["transcript"], "NM_004333");
    assert_eq!(
        serde_json::from_value::<PredictorScore>(wire).unwrap(),
        score
    );
    assert_eq!(
        FiniteNumber::new(f64::NAN).unwrap_err().code(),
        "number.value.non_finite"
    );

    let mut wire = population_wire(subject);
    wire["subset"] = serde_json::json!("non_ukb");
    wire["heterozygote_count"] = serde_json::json!(4);
    wire["grpmax_group"] = serde_json::json!("afr");
    let population = serde_json::from_value::<PopulationFrequency>(wire.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(population).unwrap(),
        wire,
        "source qualifiers survive the wire round trip"
    );
    wire["homoplasmic_count"] = serde_json::json!(2);
    frequency_error(wire.clone(), "population.mt_field.misplaced");
    wire.as_object_mut().unwrap().remove("homoplasmic_count");
    wire["ancestry_group"] = serde_json::json!("nfe");
    frequency_error(wire, "population.grpmax.misplaced");
}

#[test]
fn checked_source_identifiers_and_predicates_use_agreed_refusals() {
    let curie: Curie = "MONDO:0005105".parse().unwrap();
    let pmid: Pmid = "123456".parse().unwrap();
    let predicate: Predicate = serde_json::from_str("\"predicts_sensitivity_to\"").unwrap();
    assert_eq!(serde_json::to_string(&curie).unwrap(), "\"MONDO:0005105\"");
    assert_eq!(serde_json::to_string(&pmid).unwrap(), "\"123456\"");
    assert_eq!(
        serde_json::to_string(&predicate).unwrap(),
        "\"predicts_sensitivity_to\""
    );
    for invalid in ["MONDO", ":123", "MONDO:", "MONDO:bad value"] {
        assert_eq!(
            invalid.parse::<Curie>().unwrap_err().code(),
            "concept.curie.invalid"
        );
    }
    for invalid in ["0", "01", "-1", "abc"] {
        assert_eq!(
            invalid.parse::<Pmid>().unwrap_err().code(),
            "id.article.invalid"
        );
    }
    assert!(serde_json::from_str::<Predicate>("\"predicts_unknown\"").is_err());
}

#[test]
fn mitochondrial_fields_use_only_stated_chromosome_identity() {
    let mut wire = population_wire(VariantRef::allele_hgvs(
        "NC_000007.13:g.140453136A>T".parse().unwrap(),
    ));
    wire["homoplasmic_count"] = serde_json::json!(1);
    // The frozen validator checks the stated build only; this GRCh37 accession
    // does not identify a chromosome under the stated GRCh38 build.
    assert!(
        serde_json::from_value::<PopulationFrequency>(wire.clone()).is_ok(),
        "a reference unresolved in the stated build does not infer a chromosome"
    );
    wire["subject"] = serde_json::to_value(VariantRef::allele_hgvs(
        "NC_000007.14:g.1A>T".parse().unwrap(),
    ))
    .unwrap();
    frequency_error(wire.clone(), "population.mt_field.misplaced");
    wire["subject"] = serde_json::to_value(VariantRef::allele_hgvs(
        "NC_012920.1:m.1A>G".parse().unwrap(),
    ))
    .unwrap();
    assert!(
        serde_json::from_value::<PopulationFrequency>(wire.clone()).is_ok(),
        "the stated mitochondrial accession permits mitochondrial fields"
    );
    wire["subject"] = serde_json::to_value(VariantRef::allele(VariantId::Ca(
        CaId::new(123643).unwrap(),
    )))
    .unwrap();
    assert!(
        serde_json::from_value::<PopulationFrequency>(wire).is_ok(),
        "an allele identifier makes no chromosome claim"
    );
}

#[test]
fn population_keeps_every_stated_mitochondrial_fact_on_the_wire() {
    let mut wire = population_wire(VariantRef::allele_hgvs(
        "NC_012920.1:m.1A>G".parse().unwrap(),
    ));
    let fields = serde_json::json!({"data_type":"genomes","sex":"xx","subset":"non_ukb","genotype_filter":"raw","grpmax_group":"afr","allele_count":2,"allele_number":100,"allele_frequency":0.02,"homozygote_count":1,"heterozygote_count":2,"hemizygote_count":3,"homoplasmic_count":4,"heteroplasmic_count":5,"homoplasmic_frequency":0.04,"heteroplasmic_frequency":0.05,"faf95":0.01,"filters":["PASS"]});
    wire.as_object_mut()
        .unwrap()
        .extend(fields.as_object().unwrap().clone());
    let row = serde_json::from_value::<PopulationFrequency>(wire.clone()).unwrap();
    let wire = serde_json::to_value(&row).unwrap();
    for field in [
        "data_type",
        "sex",
        "subset",
        "genotype_filter",
        "grpmax_group",
        "allele_count",
        "allele_number",
        "allele_frequency",
        "homozygote_count",
        "heterozygote_count",
        "hemizygote_count",
        "homoplasmic_count",
        "heteroplasmic_count",
        "homoplasmic_frequency",
        "heteroplasmic_frequency",
        "faf95",
        "filters",
    ] {
        assert!(wire.get(field).is_some(), "missing {field}");
    }
    assert_eq!(
        serde_json::from_value::<PopulationFrequency>(wire).unwrap(),
        row
    );
}

#[expect(
    clippy::unwrap_used,
    reason = "the fixed public gene fixture must parse"
)]
fn source_gene() -> biodata::SourceGeneRef {
    serde_json::from_value(serde_json::json!({"label":"BRAF"})).unwrap()
}
fn population_wire(subject: VariantRef) -> serde_json::Value {
    serde_json::json!({"subject":subject,"build":"GRCh38","dataset":"gnomad","dataset_version":"4.1","attribution":{"source":"gnomad"}})
}
#[expect(
    clippy::unwrap_used,
    reason = "the negative wire contract must return a refusal"
)]
fn frequency_error(wire: serde_json::Value, code: &str) {
    let error = serde_json::from_value::<PopulationFrequency>(wire).unwrap_err();
    assert!(
        error.to_string().contains(code),
        "frequency refusal preserves {code}: {error}"
    );
}
