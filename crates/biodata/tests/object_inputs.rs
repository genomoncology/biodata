//! Validated object values refuse positional arrays at their public boundary.
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn refuses_array<T: DeserializeOwned + Serialize>(
    object: Value,
    order: &[&str],
) -> Result<(), serde_json::Error> {
    let parsed: T = serde_json::from_value(object.clone())?;
    let output = serde_json::to_value(parsed)?;
    assert!(output.is_object(), "object output shape");
    let array = Value::Array(
        order
            .iter()
            .map(|key| object.get(*key).cloned().unwrap_or(Value::Null))
            .collect(),
    );
    assert!(
        serde_json::from_value::<T>(array).is_err(),
        "{} must refuse positional arrays",
        std::any::type_name::<T>()
    );
    let roundtrip: T = serde_json::from_value(output.clone())?;
    assert_eq!(
        serde_json::to_value(roundtrip)?,
        output,
        "object output stays stable"
    );
    Ok(())
}

macro_rules! witness {
    ($name:ident, $ty:ty, $object:expr, [$($field:literal),*]) => {
        #[test]
        fn $name() -> Result<(), serde_json::Error> {
            refuses_array::<$ty>($object, &[$($field),*])
        }
    };
}

fn gene() -> Value {
    json!({"label":"SYNTH1"})
}
fn attribution() -> Value {
    json!({"source":"synthetic source"})
}
fn identity() -> Value {
    json!({"id":"CHEMBL1","namespace":"chembl","name":"synthetic drug","identity_level":"unknown"})
}
fn interval() -> Value {
    json!({"build":"GRCh38","chromosome":"1","span":{"start":0,"end":1}})
}

witness!(
    span,
    biodata::Span0,
    json!({"start":0,"end":1}),
    ["start", "end"]
);
witness!(
    bound,
    biodata::Bound,
    json!({"min":0,"max":1}),
    ["min", "max"]
);
witness!(
    uncertain_span,
    biodata::UncertainSpan0,
    json!({"start":{"min":0,"max":0},"end":{"min":1,"max":1}}),
    ["start", "end"]
);
witness!(
    symbol_gene,
    biodata::GeneRef,
    gene(),
    ["hgnc_id", "ncbi_gene_id", "ensembl_gene_id", "label"]
);
witness!(
    source_gene,
    biodata::SourceGeneRef,
    gene(),
    ["hgnc_id", "ncbi_gene_id", "ensembl_gene_id", "label"]
);
witness!(
    mane,
    biodata::Mane,
    json!({"status":"select","refseq":"NM_000001.1","ensembl":"ENST00000000001.1"}),
    ["status", "refseq", "ensembl"]
);
witness!(
    region,
    biodata::Region,
    json!({"kind":"exon","number":1,"total":2}),
    ["kind", "number", "total"]
);
witness!(
    protein_site,
    biodata::ProteinSite,
    json!({"gene":gene(),"position":1}),
    [
        "gene",
        "transcript",
        "protein",
        "position",
        "end",
        "residue"
    ]
);
witness!(
    coordinates,
    biodata::TrialCoordinates,
    json!({"latitude":1.0,"longitude":2.0}),
    ["latitude", "longitude"]
);
witness!(
    allele,
    biodata::Allele,
    json!({"location":{"sequence":"NC_000001.11","span":{"start":0,"end":1}},"reference":"A","alternate":"T"}),
    ["location", "reference", "alternate"]
);
witness!(
    expression,
    biodata::Expression,
    json!({"syntax":"hgvs.c","value":"NM_000001.1:c.1A>G"}),
    ["syntax", "value", "syntax_version"]
);
witness!(
    scientific_p_value,
    biodata::ScientificPValue,
    json!({"mantissa":"1","exponent":-2}),
    ["mantissa", "exponent"]
);
witness!(
    profile_expression,
    biodata::ProfileExpression,
    json!({"source":"#VID1","tree":{"operator":"member","record":"civic:1"}}),
    ["source", "tree"]
);
witness!(
    consequence,
    biodata::TranscriptConsequence,
    json!({"transcript":"NM_000001.1","consequences":[{"system":"so","code":"SO:0001583"}],"attribution":attribution()}),
    [
        "transcript",
        "gene",
        "consequences",
        "coding_hgvs",
        "protein_hgvs",
        "region",
        "attribution"
    ]
);
witness!(
    interaction,
    biodata::DrugInteractionClaim,
    json!({"anchor":identity(),"partner":{"id":"CHEMBL2","namespace":"chembl","name":"synthetic partner","identity_level":"unknown"},"attribution":attribution(),"additional_attributions":[]}),
    [
        "anchor",
        "partner",
        "level",
        "attribution",
        "additional_attributions"
    ]
);
witness!(
    target_relation,
    biodata::DrugTargetRelation,
    json!({"drug":identity(),"target":gene(),"native_target":{"namespace":"chembl","id":"CHEMBL3"},"references":[],"attribution":attribution()}),
    [
        "drug",
        "target",
        "native_target",
        "relation",
        "mechanism",
        "references",
        "attribution"
    ]
);
witness!(
    population,
    biodata::PopulationFrequency,
    json!({"subject":{"level":"protein","gene":gene(),"label":"p.A1T"},"build":"GRCh38","dataset":"synthetic dataset","dataset_version":"1","filters":[],"attribution":attribution()}),
    [
        "subject",
        "build",
        "dataset",
        "dataset_version",
        "data_type",
        "ancestry_group",
        "sex",
        "subset",
        "allele_count",
        "allele_number",
        "allele_frequency",
        "homozygote_count",
        "hemizygote_count",
        "faf95",
        "filters",
        "genotype_filter",
        "grpmax_group",
        "heterozygote_count",
        "homoplasmic_count",
        "heteroplasmic_count",
        "homoplasmic_frequency",
        "heteroplasmic_frequency",
        "attribution"
    ]
);
witness!(
    gene_subject,
    biodata::Subject,
    json!({"level":"gene","label":"SYNTH1"}),
    [
        "level",
        "hgnc_id",
        "ncbi_gene_id",
        "ensembl_gene_id",
        "label"
    ]
);
witness!(
    region_subject,
    biodata::Subject,
    json!({"level":"region","interval":interval(),"label":"synthetic region"}),
    ["level", "interval", "label", "record"]
);

fn method_relation() -> Value {
    json!({"kind":"method","relation":{"test_id":"GTR000000001.1","list":"methods","index":0,"value":"synthetic method"}})
}
witness!(
    coding,
    biodata::Coding,
    json!({"system":"so","code":"SO:0001583"}),
    ["system", "code"]
);
witness!(
    pathway_id,
    biodata::PathwayId,
    json!({"namespace":"wikipathways","id":"WP1"}),
    ["namespace", "id"]
);
witness!(
    gtr_relation,
    biodata::GtrTestRelation,
    method_relation(),
    ["kind", "relation"]
);
witness!(
    diagnostic_record,
    biodata::DiagnosticRecord,
    json!({"kind":"gtr_test_relation","record":method_relation()}),
    ["kind", "record"]
);
