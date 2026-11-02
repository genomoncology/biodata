//! Frozen schemas describe shapes; these approved rules remain parser-only.
#![cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

#[expect(
    clippy::expect_used,
    reason = "an invalid frozen schema must fail the boundary witness"
)]
fn parser_only<T: DeserializeOwned + JsonSchema>(wire: Value) {
    let schema = schemars::schema_for!(T).to_value();
    let validator = jsonschema::validator_for(&schema).expect("the frozen schema compiles");
    let errors: Vec<_> = validator
        .iter_errors(&wire)
        .map(|error| error.to_string())
        .collect();
    assert!(
        errors.is_empty(),
        "the frozen shape permits this boundary: {errors:?}"
    );
    assert!(
        serde_json::from_value::<T>(wire).is_err(),
        "the public parser enforces the recorded rule"
    );
}

macro_rules! witness {
    ($name:ident, $ty:ty, $wire:expr) => {
        #[test]
        fn $name() {
            parser_only::<$ty>($wire);
        }
    };
}

witness!(span_order, biodata::Span0, json!({"start":2,"end":1}));
witness!(bound_order, biodata::Bound, json!({"min":2,"max":1}));
witness!(
    bound_needs_a_nonnull_limit,
    biodata::Bound,
    json!({"min":null})
);
witness!(
    bound_integer_storage_limit,
    biodata::Bound,
    json!({"min":4294967296_u64})
);
witness!(
    uncertain_span_order,
    biodata::UncertainSpan0,
    json!({"start":{"min":2},"end":{"max":1}})
);
witness!(
    allele_reference_length,
    biodata::Allele,
    json!({"location":{"sequence":"NM_000001.1","span":{"start":0,"end":2}},"reference":"A","alternate":"T"})
);
witness!(
    allele_needs_a_nonempty_side,
    biodata::Allele,
    json!({"location":{"sequence":"NM_000001.1","span":{"start":0,"end":0}},"reference":"","alternate":""})
);
witness!(
    expression_syntax_matches_value,
    biodata::Expression,
    json!({"syntax":"hgvs.g","value":"NM_000001.1:c.1A>T"})
);
witness!(
    expression_requires_a_supported_form,
    biodata::Expression,
    json!({"syntax":"spdi","value":"invalid"})
);
witness!(
    mane_version_storage_limit,
    biodata::Mane,
    json!({"status":"select","refseq":"NM_000001.65536","ensembl":"ENST00000000001.1"})
);
witness!(
    mane_refseq_canonical_width,
    biodata::Mane,
    json!({"status":"select","refseq":"NM_000000001.1","ensembl":"ENST00000000001.1"})
);

#[test]
fn accession_resource_budget() {
    let source = format!("LRG_{}1", "0".repeat(biodata::MAX_LEN));
    parser_only::<biodata::Accession>(json!(source));
    parser_only::<biodata::AccessionBase>(json!(source));
    parser_only::<biodata::HgvsText>(json!(format!("{source}:g.1A>T")));
}

#[test]
fn variant_leaf_needs_an_identity() {
    parser_only::<biodata::VariantRef>(json!({"level":"allele"}));
    parser_only::<biodata::VariantRef>(json!({"level":"protein","gene":{"label":"GENE"}}));
}

fn haplotype() -> Value {
    json!({"level":"haplotype","kind":"Haplotype","id":"clinvar:1","accession":"VCV000000001","version":"1","number_of_chromosomes":"1","members":[{"id":"clinvar:2","allele_id":"2"}]})
}

#[test]
fn haplotype_accession_is_unversioned() {
    let mut wire = haplotype();
    wire["accession"] = json!("VCV000000001.1");
    parser_only::<biodata::VariantRef>(wire);
}

#[test]
fn haplotype_has_members() {
    let mut wire = haplotype();
    wire["members"] = json!([]);
    parser_only::<biodata::VariantRef>(wire);
}

#[test]
fn haplotype_has_a_parseable_separate_version() {
    let mut wire = haplotype();
    wire["version"] = json!("invalid");
    parser_only::<biodata::VariantRef>(wire);
}

fn member() -> Value {
    json!({"operator":"member","record":"civic:1"})
}

witness!(
    profile_source_grammar,
    biodata::ProfileExpression,
    json!({"source":"#VID0","tree":member()})
);
witness!(
    profile_source_tree_agreement,
    biodata::ProfileExpression,
    json!({"source":"#VID2","tree":member()})
);

#[test]
fn profile_expression_depth_budget() {
    parser_only::<biodata::ProfileExpression>(
        json!({"source":format!("{}#VID1{}", "(".repeat(65), ")".repeat(65)),"tree":member()}),
    );
}

#[test]
fn profile_expression_node_budget() {
    let source = vec!["#VID1"; 4097].join(" AND ");
    parser_only::<biodata::ProfileExpression>(
        json!({"source":source,"tree":{"operator":"and","operands":vec![member();4097]}}),
    );
}

fn profile(members: Value) -> Value {
    json!({"level":"profile","record":"1","label":"source","members":members,"expression":{"source":"#VID1","tree":member()}})
}

#[test]
fn profile_members_have_source_records() {
    parser_only::<biodata::VariantRef>(profile(json!([{"level":"allele","id":"rs1"}])));
}

#[test]
fn profile_member_records_are_unique() {
    let value = json!({"level":"source","record":"civic:1","feature":"source"});
    parser_only::<biodata::VariantRef>(profile(json!([value, value])));
}

#[test]
fn profile_members_cover_expression_references() {
    parser_only::<biodata::VariantRef>(profile(json!([])));
}

fn frequency() -> Value {
    json!({"subject":{"level":"allele","hgvs":"NC_000001.11:g.1A>T"},"build":"GRCh38","dataset":"source","dataset_version":"1","attribution":{"source":"source"}})
}

#[test]
fn population_count_does_not_exceed_number() {
    let mut wire = frequency();
    wire["allele_count"] = json!(2);
    wire["allele_number"] = json!(1);
    parser_only::<biodata::PopulationFrequency>(wire);
}

#[test]
fn population_group_labels_are_exclusive() {
    let mut wire = frequency();
    wire["ancestry_group"] = json!("source");
    wire["grpmax_group"] = json!("source");
    parser_only::<biodata::PopulationFrequency>(wire);
}

#[test]
fn population_fields_match_the_stated_accession_build() {
    let mut wire = frequency();
    wire["homoplasmic_count"] = json!(1);
    parser_only::<biodata::PopulationFrequency>(wire);
}

fn categorical(category: &str) -> Value {
    json!({"level":"categorical","gene":{"label":"GENE"},"category":category,"label":"source"})
}

#[test]
fn categorical_partner_requires_fusion() {
    let mut wire = categorical("amplification");
    wire["partner"] = json!({"label":"OTHER"});
    parser_only::<biodata::VariantRef>(wire);
}

#[test]
fn categorical_region_or_edit_requires_exon_mutation() {
    let mut wire = categorical("amplification");
    wire["edit"] = json!({"system":"so","code":"SO:0001583"});
    parser_only::<biodata::VariantRef>(wire);
}

#[test]
fn categorical_edit_uses_the_edit_vocabulary() {
    let mut wire = categorical("exon_mutation");
    wire["edit"] = json!({"system":"acmg_amp_2015","code":"pathogenic"});
    parser_only::<biodata::VariantRef>(wire);
}

#[test]
fn categorical_site_is_present_exactly_for_residue() {
    parser_only::<biodata::VariantRef>(categorical("residue"));
    let mut wire = categorical("amplification");
    wire["site"] = json!({"gene":{"label":"GENE"},"position":1});
    parser_only::<biodata::VariantRef>(wire);
}

#[test]
fn categorical_site_names_the_same_gene() {
    let mut wire = categorical("residue");
    wire["site"] = json!({"gene":{"label":"OTHER"},"position":1});
    parser_only::<biodata::VariantRef>(wire);
}

witness!(
    protein_site_order,
    biodata::ProteinSite,
    json!({"gene":{"label":"GENE"},"position":2,"end":1})
);
witness!(
    region_subject_needs_build,
    biodata::Subject,
    json!({"level":"region","label":"source","interval":{"chromosome":"1","span":{"start":0,"end":1}}})
);
