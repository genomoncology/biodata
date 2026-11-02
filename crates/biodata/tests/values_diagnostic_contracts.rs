//! Feature diagnostic-types: checked source-owned records before adapters exist.
use biodata::{
    CompanionDiagnosticRelation, DiagnosticDeviceId, DiagnosticRecord, FdaDrugApplication,
    GtrConditionRelation, GtrGeneRelation, GtrMethod, GtrMethodList, GtrTestId, GtrTestType,
    GtrTestVersion, PmaNumber, PmaSupplement, Statement, WhoIvdProduct, WhoProductId,
};
use serde_json::{Value, json};

#[expect(
    clippy::unwrap_used,
    reason = "Contract setup must fail when synthetic public values cannot round-trip"
)]
fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(value: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(value).unwrap()).unwrap()
}

#[test]
fn native_id_grammars_refuse_wrappers_and_incomplete_identities() {
    for value in ["GTR000006692.3", "GTR000511179.13"] {
        assert_eq!(roundtrip::<GtrTestId>(json!(value)), value);
    }
    for value in [
        "GTR000006692",
        "gtr:GTR000006692.3",
        "GTR6692.3",
        "GTR000006692.0",
        "GTR000006692.03",
        "GTR000006692.+3",
    ] {
        assert!(serde_json::from_value::<GtrTestId>(json!(value)).is_err());
    }
    assert_eq!(
        roundtrip::<WhoProductId>(json!("0466-046-00")),
        "0466-046-00"
    );
    for id in ["13641-013-00", "10304-118-00", "10307-04545-00"] {
        assert_eq!(roundtrip::<WhoProductId>(json!(id)), id);
    }
    assert_eq!(roundtrip::<PmaNumber>(json!("P150013")), "P150013");
    assert_eq!(roundtrip::<PmaSupplement>(json!("S020")), "S020");
    assert_eq!(
        roundtrip::<FdaDrugApplication>(json!("NDA021335")),
        "NDA021335"
    );
    for value in ["021335", "NDA21335", "drugsfda:NDA021335", "nda021335"] {
        assert!(serde_json::from_value::<FdaDrugApplication>(json!(value)).is_err());
    }
    for value in ["P150013", "P150013/S020", "K173492", "DEN170019"] {
        assert_eq!(roundtrip::<DiagnosticDeviceId>(json!(value)), value);
    }
    for value in [
        "fda:P150013",
        "P150013S020",
        "P150013/S000",
        "DEN17",
        "K173492/S001",
    ] {
        assert!(serde_json::from_value::<DiagnosticDeviceId>(json!(value)).is_err());
    }
}

fn gene() -> Value {
    json!(
        {"test_id":"GTR000006692.3","test_type":"Clinical","gene":{"ncbi_gene_id":"672","label":"Synthetic symbol"},
        "native_medgen":"MedGen:C0376571","mim_number":"","object_name":"Synthetic native gene label"}
    )
}
fn condition() -> Value {
    json!(
        {"test_id":"GTR000006692.3","test_type":"Clinical","condition":{"id":"MedGen:C2676676","label":"Synthetic condition"},
        "mim_number":"604370","snomed_ct_id":""}
    )
}
fn method() -> Value {
    json!({"test_id":"GTR000006692.3","list":"methods","index":2,"value":"Synthetic unpaired method"})
}
fn who() -> Value {
    let rows: Vec<_> = (0..9)
        .map(|i| {
            json!(
                {"product_name":"Synthetic product","product_code":format!("code-{i}"),"assay_format":"Synthetic format",
                "manufacturer":"Synthetic manufacturer","marker":format!("native marker {i}"),"regulatory_version":"Uninterpreted source prose",
                "prequalification_year":"2024"}
            )
        })
        .collect();
    json!({"id":"0466-046-00","native_rows":rows})
}
fn companion(group: bool) -> Value {
    let headers = if group {
        vec![
            "Diagnostic Name\n(Manufacturer)",
            "Indication(s) - Sample Type",
            "PMA (Approval Date)",
            "Device Indication for a Specific Group of Oncology Therapeutic Products and Trade Name (Generic) – NDA/BLA",
        ]
    } else {
        vec![
            "Diagnostic Name (Manufacturer)",
            "Indication - Sample Type",
            "Drug Trade Name (Generic) NDA / BLA",
            "Biomarker(s)",
            "Biomarker(s) (Details)",
            "PMA / 510(k) /\n513(f)(2) / HDE (Approval / Clearance / Grant Date)",
        ]
    };
    let cells: serde_json::Map<_, _> = headers
        .into_iter()
        .map(|h| {
            (
                h.to_owned(),
                json!({"text":format!("Whole synthetic {h} cell"), "links":[{"href":"https://example.invalid/cell-owned"}]}),
            )
        })
        .collect();
    json!(
        {"kind":if group {"group"} else {"individual"},"snapshot":"synthetic-revision","index":7,"cells":cells,"device_ids":["P150013/S020",
        "K173492","DEN170019"],"drug_applications":[{"id":"NDA021335","url":"https://example.invalid/owned/NDA021335"},{"id":"BLA125514",
        "url":"https://example.invalid/owned/BLA125514"}]}
    )
}

#[test]
fn native_memberships_keep_namespace_and_cannot_become_statements() {
    for row in [gene(), condition(), method()] {
        assert!(serde_json::from_value::<Statement>(row).is_err());
    }
    assert_eq!(roundtrip::<GtrGeneRelation>(gene()), gene());
    assert_eq!(roundtrip::<GtrConditionRelation>(condition()), condition());
    assert_eq!(roundtrip::<GtrMethod>(method()), method());
    assert!(serde_json::from_value::<GtrGeneRelation>(condition()).is_err());
    assert!(serde_json::from_value::<GtrConditionRelation>(gene()).is_err());
    for kind in ["Clinical", "Research", "Clinical microbial"] {
        assert_eq!(roundtrip::<GtrTestType>(json!(kind)), kind);
    }
    assert!(serde_json::from_value::<GtrTestType>(json!("clinical")).is_err());
    for list in ["methods", "method_categories", "platforms"] {
        assert_eq!(roundtrip::<GtrMethodList>(json!(list)), list);
    }
    assert!(serde_json::from_value::<GtrMethodList>(json!("all")).is_err());
}

#[test]
fn native_test_rows_keep_empty_cells_and_all_five_status_flags() {
    let mut row = json!({"test_accession_ver":"GTR000551465.2"});
    for key in [
        "name_of_laboratory",
        "name_of_institution",
        "facility_state",
        "facility_postcode",
        "facility_country",
        "CLIA_number",
        "state_licenses",
        "state_license_numbers",
        "lab_test_id",
        "last_touch_date",
        "lab_test_name",
        "manufacturer_test_name",
        "test_development",
        "lab_id",
        "condition_identifiers",
        "indication_types",
        "inheritances",
        "method_categories",
        "methods",
        "platforms",
        "genes",
        "drug_responses",
        "now_current",
        "test_currStat",
        "test_pubStat",
        "lab_currStat",
        "lab_pubStat",
        "test_create_date",
        "test_deletion_date",
        "test_order_code",
    ] {
        row[key] = json!("");
    }
    for (key, value) in [
        ("now_current", "0"),
        ("test_currStat", "1"),
        ("test_pubStat", "3"),
        ("lab_currStat", "72"),
        ("lab_pubStat", "4"),
    ] {
        row[key] = json!(value);
    }
    let first = roundtrip::<GtrTestVersion>(row.clone());
    row["last_touch_date"] = json!("2026-06-29");
    let second = roundtrip::<GtrTestVersion>(row.clone());
    assert_ne!(first, second);
    assert_eq!(second["CLIA_number"], "");
    assert_eq!(second["lab_currStat"], "72");
    assert_eq!(second["test_pubStat"], "3");
    let records = [
        json!({"kind":"gtr_test_version","record":first}),
        json!({"kind":"gtr_test_version","record":second}),
    ];
    assert_eq!(
        roundtrip::<Vec<DiagnosticRecord>>(json!(records)),
        json!(records)
    );
}

#[test]
fn who_cards_preserve_every_complete_code_row_and_original_scope() {
    let value = who();
    let card = roundtrip::<WhoIvdProduct>(value.clone());
    assert_eq!(card, value);
    assert_eq!(card["native_rows"].as_array().unwrap().len(), 9);
    assert_eq!(card["native_rows"][8]["marker"], "native marker 8");
    assert!(serde_json::from_value::<Statement>(card).is_err());
}

#[test]
fn physical_companion_tables_keep_exact_headers_whole_cells_and_owned_ids() {
    for group in [false, true] {
        let row = companion(group);
        assert_eq!(roundtrip::<CompanionDiagnosticRelation>(row.clone()), row);
        assert!(serde_json::from_value::<Statement>(row.clone()).is_err());
        let mut invalid = row.clone();
        invalid["cells"]["invented column"] = json!({"text":"invented","links":[]});
        assert_eq!(roundtrip::<CompanionDiagnosticRelation>(invalid), row);
        let mut wrong_kind = row;
        wrong_kind["kind"] = json!(if group { "individual" } else { "group" });
        assert!(serde_json::from_value::<CompanionDiagnosticRelation>(wrong_kind).is_err());
    }
    let mut bad_id = companion(false);
    bad_id["drug_applications"][0]["id"] = json!("21335");
    assert!(serde_json::from_value::<CompanionDiagnosticRelation>(bad_id).is_err());
}
