//! Feature: phenotype-types. Public checked request and neutral record contracts.
use biodata::HpId;
use serde_json::json;

#[test]
fn hp_identity_round_trips_and_rejects_other_namespaces() {
    for value in ["HP:0001250", "HP:0000000", "HP:9999999"] {
        let id: HpId = value.parse().unwrap();
        assert_eq!(id.to_string(), value);
        assert_eq!(serde_json::from_value::<HpId>(json!(value)).unwrap(), id);
    }
    for value in [
        "HP:1250",
        "HP:00001250",
        "hp:0001250",
        "MONDO:0001250",
        "HGNC:1",
        " HP:0001250",
        "HP:00012a0",
    ] {
        assert!(value.parse::<HpId>().is_err(), "{value}");
    }
}

#[test]
fn native_cards_keep_empty_values_independent_sources_and_count_scopes() {
    use biodata::{PhenotypeCard, PhenotypeSynonym};
    let hpo = json!(
        {"source":"hpo","requested_id":"HP:0001250","returned_id":"HP:0001250","label":"Seizure","definition":"","comment":"",
        "hpo_descendant_count":346,"attribution":{"source":"hpo"}}
    );
    let card: PhenotypeCard = serde_json::from_value(hpo.clone()).unwrap();
    assert_eq!(serde_json::to_value(card).unwrap(), hpo);
    let absent: PhenotypeCard = serde_json::from_value(json!(
        {"source":"hpo","requested_id":"HP:0001250","returned_id":"HP:0001250","label":"Seizure","attribution":{"source":"hpo"}}
    ))
    .unwrap();
    assert!(
        serde_json::to_value(absent)
            .unwrap()
            .get("definition")
            .is_none()
    );
    let monarch = json!(
        {"source":"monarch","requested_id":"HP:0001676","returned_id":"HP:0001676","label":"obsolete source label","deprecated":true,
        "monarch_descendant_count":1,"association_counts":[{"key":"biolink:DiseaseToPhenotypicFeatureAssociation",
        "category":"biolink:DiseaseToPhenotypicFeatureAssociation","label":"Disease to Phenotype","count":5350,"count_direct":2561}],
        "attribution":{"source":"monarch"}}
    );
    assert_eq!(
        serde_json::to_value(serde_json::from_value::<PhenotypeCard>(monarch.clone()).unwrap())
            .unwrap(),
        monarch
    );
    let mut merged = hpo.clone();
    merged
        .as_object_mut()
        .unwrap()
        .insert("deprecated".into(), json!(false));
    assert_eq!(
        serde_json::to_value(serde_json::from_value::<PhenotypeCard>(merged).unwrap()).unwrap(),
        hpo
    );
    for relationship in [
        None,
        Some("exact"),
        Some("broad"),
        Some("narrow"),
        Some("related"),
    ] {
        let mut row = json!({"value":"","attribution":{"source":"synthetic"}});
        if let Some(r) = relationship {
            row.as_object_mut()
                .unwrap()
                .insert("relationship".into(), json!(r));
        }
        assert_eq!(
            serde_json::to_value(serde_json::from_value::<PhenotypeSynonym>(row.clone()).unwrap())
                .unwrap(),
            row
        );
    }
}

#[test]
fn associations_reuse_native_endpoints_and_never_deserialize_as_statements() {
    use biodata::{DiseaseAssociation, Statement};
    let row = json!(
        {"requested_id":"HP:0001250","subject":{"id":"HGNC:29529","label":"synthetic gene"},"subject_taxon":"NCBITaxon:9606",
        "object":{"id":"HP:0001250","label":"synthetic term"},"predicate":"biolink:has_phenotype","attribution":{"source":"synthetic"},
        }
    );
    let association: DiseaseAssociation = serde_json::from_value(row.clone()).unwrap();
    assert_eq!(serde_json::to_value(association).unwrap(), row);
    assert!(serde_json::from_value::<Statement>(row).is_err());
}
