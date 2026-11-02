//! Feature pathway-types: public no-I/O identity and consumer contracts.
use biodata::{Pathway, PathwayEventRef, PathwayId, PathwayMember, WikiPathwaysId};
use serde_json::{Value, json};

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(
    v: Value,
) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(v)?)
}

#[test]
fn identities_preserve_native_namespaces_and_refuse_foreign_species() {
    for id in ["WP382", "WP1"] {
        assert_eq!(
            roundtrip::<WikiPathwaysId>(json!(id)).expect("public round-trip"),
            id
        );
    }
    for id in ["WP", "WP0", "WP0382", "wp382", "WP382_r140476", "hsa:04010"] {
        assert!(serde_json::from_value::<WikiPathwaysId>(json!(id)).is_err());
    }
    for id in [
        json!({"namespace":"reactome","id":"R-HSA-5673001"}),
        json!({"namespace":"wikipathways","id":"WP382"}),
    ] {
        assert_eq!(
            roundtrip::<PathwayId>(id.clone()).expect("public round-trip"),
            id
        );
    }
    for id in [
        json!({"source":"reactome","id":"R-HSA-5673001"}),
        json!({"namespace":"reactome","id":"R-MMU-5673001"}),
        json!({"namespace":"reactome","id":"R-HSA-5673001.12"}),
        json!({"namespace":"kegg","id":"hsa04010"}),
    ] {
        assert!(serde_json::from_value::<PathwayId>(id).is_err());
    }
}

#[test]
fn neutral_records_keep_absence_membership_and_unresolved_event_identity() {
    let base = json!(
        {"id":{"namespace":"reactome","id":"R-HSA-5673001"},"name":"RAF/MAP kinase cascade",
        "species":"Homo sapiens","version":"R-HSA-5673001.12"}
    );
    assert_eq!(
        roundtrip::<Pathway>(base.clone()).expect("public round-trip"),
        base
    );
    let mut empty = base.clone();
    empty
        .as_object_mut()
        .expect("object")
        .insert("description".into(), json!(""));
    assert_eq!(
        roundtrip::<Pathway>(empty.clone()).expect("public round-trip"),
        empty
    );
    let mut foreign = base;
    foreign
        .as_object_mut()
        .expect("object")
        .insert("species".into(), json!("Mus musculus"));
    assert!(serde_json::from_value::<Pathway>(foreign).is_err());
    for accession in ["P01116-1", "P01116-2"] {
        let member = json!(
            {"reference":{"namespace":"uniprot","id":accession},"name":format!("UniProt:{accession} KRAS"),"gene":"HGNC:6407",
            "schema_class":"ReferenceIsoform","physical_context_count":1,"participants":[{"db_id":{"namespace":"Reactome","id":5672718},
            "name":"p21 RAS:GTP:activated RAF homo/heterodimer complexes [plasma membrane]","schema_class":"Complex"}]}
        );
        assert_eq!(
            roundtrip::<PathwayMember>(member.clone()).expect("public round-trip"),
            member
        );
    }
    let unknown =
        json!({"reference":{"namespace":"chebi","id":"15377"},"physical_context_count":0});
    assert_eq!(
        roundtrip::<PathwayMember>(unknown.clone()).expect("public round-trip"),
        unknown
    );
    for event in [
        json!({"kind":"database","db_id":{"namespace":"Reactome","id":5673001}}),
        json!({"kind":"object","db_id":{"namespace":"Reactome","id":5668599},"name":"Native reaction","schema_class":"Reaction"}),
        json!({"kind":"object","db_id":{"namespace":"Reactome","id":5673001},"id":"R-HSA-5673001",
        "version":"R-HSA-5673001.12","name":"Native pathway","schema_class":"Pathway"}),
    ] {
        assert_eq!(
            roundtrip::<PathwayEventRef>(event.clone()).expect("public round-trip"),
            event
        );
    }
    assert!(
        serde_json::from_value::<PathwayEventRef>(
            json!({"kind":"database","db_id":{"namespace":"Reactome","id":0}})
        )
        .is_err()
    );
}

#[test]
fn source_summation_multiplicity_and_pathway_citations_stay_independent() {
    // Minimal source-field projection; text/title and the second summation are synthetic controls.
    let pathway = json!(
        {"id":{"namespace":"reactome","id":"R-HSA-5673001"},"name":"RAF/MAP kinase cascade","species":"Homo sapiens",
        "summations":[{"db_id":{"namespace":"Reactome","id":5683030},"text":"Complete retained source text."},
        {"db_id":{"namespace":"Reactome","id":5683031},"text":"Second source summation."}],
        "references":[{"db_id":{"namespace":"Reactome","id":5672144},"pubmed":"24937142","title":"Native reference"},
        {"db_id":{"namespace":"Reactome","id":208231},"pubmed":"17496910"}]}
    );
    assert_eq!(
        roundtrip::<Pathway>(pathway.clone()).expect("public round-trip"),
        pathway
    );
    let wiki = json!(
        {"id":{"namespace":"wikipathways","id":"WP382"},"name":"MAPK signaling pathway","species":"Homo sapiens","revision":"2025-08-29",
        "description":"Native catalog excerpt ...","cited_in":""}
    );
    assert_eq!(
        roundtrip::<Pathway>(wiki.clone()).expect("public round-trip"),
        wiki
    );
}
