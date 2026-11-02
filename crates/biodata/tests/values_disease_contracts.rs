//! Disease public wire contracts precede provider implementations.
use biodata::{DiseaseAssociation, Statement};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

fn roundtrip<T: DeserializeOwned + Serialize>(value: Value) -> Result<Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(value)?)
}

#[test]
fn association_keeps_parent_request_and_native_child_without_becoming_statement() {
    let row = json!(
        {"requested_id":"MONDO:0019003","subject":{"id":"HGNC:9967","label":"RET"},"object":{"id":"MONDO:0008234","label":"MEN2A"},
        "predicate":"biolink:gene_associated_with_condition","original_subject":"NCBIGene:5979","original_object":"Orphanet:247698",
        "aggregator_knowledge_source":["infores:monarchinitiative"],"attribution":{"source":"synthetic"},
        "object_closure":["MONDO:0019003"]}
    );
    assert_eq!(roundtrip::<DiseaseAssociation>(row.clone()).unwrap(), row);
    assert!(serde_json::from_value::<Statement>(row).is_err());
}

#[test]
fn native_statistics_keep_requested_and_returned_dimensions_and_missing_observations() {
    let value = json!(
        {"requested_site":"55","requested_selectors":[{"name":"dataset","code":"4"}],"returned_selectors":[{"name":"site","code":"55"},
        {"name":"year_range","code":"31"}],"year":2022,"measure":"rel_rate","modeled":{"value":{"value":93.690345}},
        "attribution":{"source":"synthetic"}}
    );
    let result = roundtrip::<biodata::DiseaseStatistic>(value.clone()).unwrap();
    assert_eq!(result, value);
    assert!(result.get("observed").is_none());
    assert_eq!(result["requested_selectors"][0]["code"], "4");
    assert_eq!(result["returned_selectors"][1]["code"], "31");
}

#[test]
fn association_aggregators_keep_native_null_and_omit_empty_lists() {
    let base = json!(
        {"requested_id":"MONDO:0019003","subject":{"id":"HGNC:9967","label":"Synthetic"},"object":{"id":"MONDO:0007958",
        "label":"Synthetic child"},"predicate":"biolink:associated_with","attribution":{"source":"synthetic"},
        }
    );
    assert_eq!(roundtrip::<DiseaseAssociation>(base.clone()).unwrap(), base);
    for sources in [
        json!(null),
        json!([]),
        json!(["infores:monarchinitiative", "infores:hpo-annotations"]),
    ] {
        let mut row = base.clone();
        let empty = sources == json!([]);
        row["aggregator_knowledge_source"] = sources;
        let output = roundtrip::<DiseaseAssociation>(row.clone()).unwrap();
        if empty {
            row.as_object_mut()
                .unwrap()
                .remove("aggregator_knowledge_source");
        }
        assert_eq!(output, row);
    }
    for sources in [json!("infores:monarchinitiative"), json!(["invalid"])] {
        let mut row = base.clone();
        row["aggregator_knowledge_source"] = sources;
        assert!(serde_json::from_value::<DiseaseAssociation>(row).is_err());
    }
}

#[test]
fn association_native_numbers_are_independent_and_never_derived() {
    let base = json!(
        {"requested_id":"MONDO:0019003","subject":{"id":"MONDO:0019003","label":"Synthetic"},"object":{"id":"HP:0000001",
        "label":"Synthetic feature"},"predicate":"biolink:has_phenotype","attribution":{"source":"synthetic"},
        }
    );
    for (field, value) in [
        ("has_count", json!(0)),
        ("has_total", json!(35)),
        ("has_percentage", json!(77.14286)),
        ("has_quotient", json!(0.7714286)),
    ] {
        let mut row = base.clone();
        row[field] = value;
        assert_eq!(roundtrip::<DiseaseAssociation>(row.clone()).unwrap(), row);
    }
    let mut invalid = base;
    invalid["has_count"] = json!(-1);
    assert!(serde_json::from_value::<DiseaseAssociation>(invalid).is_err());
}

#[test]
fn association_references_allow_native_urls_without_invented_labels() {
    let row = json!(
        {"requested_id":"MONDO:0019003","subject":{"id":"MONDO:0019003","label":"Synthetic"},"object":{"id":"HP:0000001",
        "label":"Synthetic feature"},"predicate":"biolink:has_phenotype","attribution":{"source":"synthetic"},
        "evidence":[{"id":"ECO:0000501","url":"http://purl.obolibrary.org/obo/ECO_0000501"}],"publications":[{"id":"PMID:1",
        "url":"http://identifiers.org/pubmed/1"}],"qualifiers":[{"field":"frequency_qualifier","id":"HP:0040283",
        "label":"Synthetic frequency"}],}
    );
    assert_eq!(roundtrip::<DiseaseAssociation>(row.clone()).unwrap(), row);
}

#[test]
fn association_evidence_omits_empty_lists_and_keeps_populated_terms() {
    let base = json!(
        {"requested_id":"MONDO:0019003","subject":{"id":"MONDO:0019003","label":"Synthetic"},"object":{"id":"HP:0000001",
        "label":"Synthetic feature"},"predicate":"biolink:has_phenotype","attribution":{"source":"synthetic"},
        }
    );
    assert_eq!(roundtrip::<DiseaseAssociation>(base.clone()).unwrap(), base);
    let mut unavailable = base.clone();
    unavailable["evidence"] = json!(null);
    assert_eq!(roundtrip::<DiseaseAssociation>(unavailable).unwrap(), base);
    for evidence in [
        json!([]),
        json!([{"id":"ECO:0000501","label":"Synthetic evidence"}]),
    ] {
        let mut row = base.clone();
        let empty = evidence == json!([]);
        row["evidence"] = evidence;
        let output = roundtrip::<DiseaseAssociation>(row.clone()).unwrap();
        if empty {
            row.as_object_mut().unwrap().remove("evidence");
        }
        assert_eq!(output, row);
    }
}
