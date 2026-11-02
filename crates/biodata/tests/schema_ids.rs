//! Each inherited BioData schema ID names one public value type.
#![cfg(feature = "schema")]
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize)]
struct Owner {
    #[serde(rename = "type")]
    public_type: String,
    schema_name: String,
}

#[test]
fn inherited_schema_ids_resolve_to_unique_public_value_types() {
    let owners: BTreeMap<String, Owner> = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../schemas/ids.json"
    )))
    .unwrap();
    let mut schemas = schema_group_0();
    schemas.extend(schema_group_1());
    schemas.extend(schema_group_2());
    assert_eq!(
        owners.len(),
        schemas.len(),
        "every inherited ID has one declared public type"
    );
    let mut seen = BTreeSet::new();
    for (id, owner) in owners {
        let (family, version) = id.split_once('/').unwrap();
        assert!(
            family.starts_with("biodata.") && !family.trim_start_matches("biodata.").is_empty(),
            "schema IDs belong to this public crate"
        );
        assert!(
            version.parse::<u32>().is_ok_and(|value| value > 0),
            "schema IDs carry a positive contract version"
        );
        assert!(
            seen.insert(owner.public_type.clone()),
            "each public type owns one inherited schema ID"
        );
        let schema = schemas
            .get(owner.public_type.as_str())
            .expect("the ID describes a public BioData value with a schema");
        assert_eq!(
            schema.get("title").and_then(|title| title.as_str()),
            Some(owner.schema_name.as_str()),
            "schema names survive the import for {id}"
        );
    }
}

macro_rules! group {
    ($($ty:ident),+ $(,)?) => { BTreeMap::from([ $((stringify!($ty), schemars::schema_for!(biodata::$ty).to_value())),+ ]) };
}

fn schema_group_0() -> BTreeMap<&'static str, serde_json::Value> {
    group!(
        Accession,
        AccessionBase,
        Allele,
        AuthorizationDate,
        AuthorizationEvent,
        Base,
        Build,
        CaId,
        Chromosome,
        ClinVarAlleleId,
        ClinVarVariationId,
        CodingChange,
        Contig,
        CosmicId,
        Cytoband,
        DeviceRecallNumber,
        DiseaseAssociation,
        DiseaseEndpoint,
        DiseaseRelation,
        DiseaseStatistic,
        DiseaseSynonym,
        Doi,
        DrugDiseaseReference,
        DrugId,
        DrugIdentity,
        DrugIdentityLevel,
        DrugIdentityRelation,
        DrugIndicationClaim,
        DrugInteractionClaim,
        DrugLabelReference
    )
}

fn schema_group_1() -> BTreeMap<&'static str, serde_json::Value> {
    group!(
        DrugLabelSection,
        DrugNativeReference,
        DrugNativeTarget,
        DrugProduct,
        DrugRecallNumber,
        DrugStage,
        DrugTargetRelation,
        EnsemblGeneId,
        EuropePmcId,
        Expression,
        ExpressionSyntax,
        Gene,
        GeneLocation,
        GeneStatus,
        GeneSymbol,
        GenomicInterval,
        HgncId,
        HgncStatus,
        HgvsText,
        LocusGroup,
        LocusType,
        Mane,
        Markdown,
        MatchKind,
        MondoId,
        NcbiGeneId,
        NcitId,
        NctId,
        OmimId,
        PaId
    )
}

fn schema_group_2() -> BTreeMap<&'static str, serde_json::Value> {
    group!(
        PartialDate,
        Pmcid,
        Pmid,
        PopulationFrequency,
        Position1,
        RcvId,
        Representation,
        RsId,
        ScvId,
        Sequence,
        SequenceLocation,
        SourceGeneRef,
        Span0,
        Statement,
        Strand,
        Summary,
        SummarySource,
        Text,
        UniProtAcc,
        Variant,
        VariantId,
        VcvId,
        VrsId
    )
}
