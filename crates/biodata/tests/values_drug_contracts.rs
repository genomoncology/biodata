//! Drug public API contracts, before any command implementation exists.
use biodata::DrugId;
use serde_json::json;
#[test]
fn published_ids_roundtrip_without_source_prefixes() {
    for value in [
        "CHEBI:756472",
        "CHEMBL941",
        "EMEA/H/C/000406",
        "NDA021335",
        "741ff3e3-dc1a-45a6-84e5-2481b27131aa",
        "DDInter1744",
        "5",
        "KTUFNOKKBVMGRW-UHFFFAOYSA-N",
    ] {
        let id: DrugId = value.parse().expect("published ID");
        assert_eq!(id.to_string(), value);
        assert_eq!(
            serde_json::from_value::<DrugId>(json!(value)).expect("serde ID"),
            id
        );
    }
    for value in [
        "chembl:CHEMBL941",
        "ema:EMEA/H/C/000406",
        "openfda-label:741ff3e3-dc1a-45a6-84e5-2481b27131aa",
        "",
        "Gleevec tablets",
        "1:foo",
        "CHEBI:",
        "CHEBI::1",
        "CHEBI:/1",
        " CHEMBL941",
    ] {
        assert!(value.parse::<DrugId>().is_err(), "{value}");
        assert!(serde_json::from_value::<DrugId>(json!(value)).is_err());
    }
}
#[test]
fn native_text_and_regulator_date_roles_roundtrip_without_interpretation() {
    use biodata::{AuthorizationEvent, DrugLabelSection};
    let event = json!(
        {"record":"EMEA/H/C/000406","jurisdiction":"native jurisdiction","status":"source status","dates":[{"role":"original authorization",
        "date":"2001-11-07"},{"role":"other date","date":"2001-11-06"}],"attribution":{"source":"ema","record":"EMEA/H/C/000406"}}
    );
    let decoded: AuthorizationEvent = serde_json::from_value(event.clone()).expect("event");
    assert_eq!(serde_json::to_value(decoded).expect("event JSON"), event);
    let label = json!({"label":{"id":"741ff3e3-dc1a-45a6-84e5-2481b27131aa",
"attribution":{"source":"openfda-label"}},
"heading":"drug_interactions",
"text":"  original text\nsecond line  "});
    let decoded: DrugLabelSection = serde_json::from_value(label.clone()).expect("label");
    assert_eq!(serde_json::to_value(decoded).expect("label JSON"), label);
}
#[test]
fn native_target_identity_does_not_require_an_invented_gene() {
    use biodata::{DrugNativeReference, DrugNativeTarget, DrugTargetRelation};
    let value = json!(
        {"drug":{"id":"CHEMBL1","namespace":"chembl","name":"synthetic drug","identity_level":"unknown"},"native_target":{"namespace":"chembl",
        "id":"CHEMBL999","label":"synthetic target complex","url":"https://example.org/target"},"relation":"native action",
        "mechanism":"source mechanism wording","references":[{"ref_type":"native publication","ref_id":"synthetic-1",
        "ref_url":"https://example.org/reference"}],"attribution":{"source":"chembl"}}
    );
    let parsed: DrugTargetRelation =
        serde_json::from_value(value.clone()).expect("native target without gene");
    assert!(parsed.target().is_none());
    assert_eq!(
        parsed.relation().expect("native relation").as_str(),
        "native action"
    );
    assert_eq!(
        parsed.mechanism().expect("native mechanism").as_str(),
        "source mechanism wording"
    );
    let native = DrugNativeTarget {
        namespace: "chembl".parse().expect("source"),
        id: "CHEMBL999".parse().expect("id"),
        label: Some("synthetic target complex".parse().expect("label")),
        url: Some("https://example.org/target".parse().expect("link")),
    };
    let reference = DrugNativeReference {
        ref_type: "native publication".parse().expect("type"),
        ref_id: Some("synthetic-1".parse().expect("id")),
        ref_url: Some("https://example.org/reference".parse().expect("link")),
    };
    assert_eq!(parsed.references(), &[reference]);
    let rebuilt = DrugTargetRelation::new(
        parsed.drug().clone(),
        None,
        Some(native),
        parsed.relation().cloned(),
        parsed.mechanism().cloned(),
        parsed.references().into(),
        parsed.attribution().clone(),
    )
    .expect("checked target constructor");
    assert_eq!(rebuilt, parsed);
    assert!(
        DrugTargetRelation::new(
            parsed.drug().clone(),
            None,
            None,
            parsed.relation().cloned(),
            None,
            Box::default(),
            parsed.attribution().clone()
        )
        .is_err()
    );
    let gene = biodata::SourceGeneRef::new("source gene".parse().expect("label"), None, None, None)
        .expect("native gene");
    let gene_only = DrugTargetRelation::new(
        parsed.drug().clone(),
        Some(gene.clone()),
        None,
        parsed.relation().cloned(),
        None,
        Box::default(),
        parsed.attribution().clone(),
    )
    .expect("gene-only target");
    assert_eq!(gene_only.target(), Some(&gene));
    let gene_json = serde_json::to_value(&gene_only).expect("gene-only JSON");
    assert!(gene_json.get("native_target").is_none());
    assert_eq!(gene_json["target"], json!({"label":"source gene"}));
    assert_eq!(
        serde_json::from_value::<DrugTargetRelation>(gene_json).expect("gene-only decode"),
        gene_only
    );
    assert_eq!(serde_json::to_value(parsed).expect("target JSON"), value);
    let mut missing = value;
    missing
        .as_object_mut()
        .expect("target object")
        .remove("native_target");
    assert!(serde_json::from_value::<DrugTargetRelation>(missing).is_err());
}
#[test]
fn native_indication_ids_keep_their_own_labels_stage_and_references() {
    use biodata::{DrugDiseaseReference, DrugIndicationClaim, DrugStage};
    let value = json!(
        {"drug":{"id":"CHEMBL1","namespace":"chembl","name":"synthetic drug","identity_level":"unknown"},
        "indication":"source indication wording","disease_references":[{"namespace":"EFO","id":"EFO:synthetic",
        "label":"native EFO wording"},{"namespace":"MeSH","id":"Dsynthetic","label":"different native MeSH wording"}],
        "stage":{"field":"max_phase_for_ind","value":"3.0"},"references":[{"ref_type":"native trial","ref_id":"synthetic-trial",
        "ref_url":"https://example.org/trial"}],"attribution":{"source":"chembl"}}
    );
    let parsed: DrugIndicationClaim =
        serde_json::from_value(value.clone()).expect("native indication");
    assert_eq!(
        parsed.disease_references.first(),
        Some(&DrugDiseaseReference {
            namespace: "EFO".parse().expect("namespace"),
            id: "EFO:synthetic".parse().expect("id"),
            label: Some("native EFO wording".parse().expect("label"))
        })
    );
    assert_eq!(
        parsed.stage,
        Some(DrugStage {
            field: "max_phase_for_ind".into(),
            value: "3.0".into()
        })
    );
    assert_eq!(
        serde_json::to_value(parsed).expect("indication JSON"),
        value
    );
    let stage: DrugStage =
        serde_json::from_value(json!({"field":" native_field ","value":" 3.0 \n"}))
            .expect("unchanged stage strings");
    assert_eq!(stage.field.as_ref(), " native_field ");
    assert_eq!(stage.value.as_ref(), " 3.0 \n");
}
#[test]
fn interaction_keeps_both_bundle_origins_and_refuses_duplicate_attribution() {
    use biodata::DrugInteractionClaim;
    let value = json!(
        {"anchor":{"id":"DDInter1","namespace":"ddinter","name":"synthetic anchor","identity_level":"unknown"},"partner":{"id":"DDInter2",
        "namespace":"ddinter","name":"synthetic partner","identity_level":"unknown"},"level":"native level","attribution":{"source":"ddinter",
        "record":"B-synthetic"},"additional_attributions":[{"source":"ddinter","record":"L-synthetic"}]}
    );
    let parsed: DrugInteractionClaim =
        serde_json::from_value(value.clone()).expect("both bundle origins");
    assert_eq!(parsed.anchor().id.to_string(), "DDInter1");
    assert_eq!(parsed.partner().id.to_string(), "DDInter2");
    assert_eq!(parsed.level().map(|v| v.as_str()), Some("native level"));
    assert_eq!(
        parsed
            .attribution()
            .record
            .as_ref()
            .expect("primary")
            .as_str(),
        "B-synthetic"
    );
    let additional: Box<[biodata::Attribution]> = serde_json::from_value(
        serde_json::to_value(&parsed).expect("serialize interaction")["additional_attributions"]
            .clone(),
    )
    .expect("checked secondary origins");
    assert_eq!(
        additional[0]
            .record
            .as_ref()
            .expect("secondary record")
            .as_str(),
        "L-synthetic"
    );
    let rebuilt = DrugInteractionClaim::new(
        parsed.anchor().clone(),
        parsed.partner().clone(),
        parsed.level().cloned(),
        parsed.attribution().clone(),
        additional,
    )
    .expect("checked interaction constructor");
    assert_eq!(rebuilt, parsed);
    assert!(
        DrugInteractionClaim::new(
            parsed.anchor().clone(),
            parsed.partner().clone(),
            parsed.level().cloned(),
            parsed.attribution().clone(),
            vec![parsed.attribution().clone()].into()
        )
        .is_err()
    );
    assert_eq!(
        serde_json::to_value(parsed).expect("interaction JSON"),
        value
    );
    let mut duplicate = value.clone();
    duplicate["additional_attributions"] = json!([value["attribution"].clone()]);
    assert!(serde_json::from_value::<DrugInteractionClaim>(duplicate).is_err());
    let mut duplicate = value.clone();
    duplicate["additional_attributions"] = json!([
        value["additional_attributions"][0].clone(),
        value["additional_attributions"][0].clone()
    ]);
    assert!(serde_json::from_value::<DrugInteractionClaim>(duplicate).is_err());
    let mut legacy = value;
    legacy
        .as_object_mut()
        .expect("interaction object")
        .remove("additional_attributions");
    let parsed: DrugInteractionClaim =
        serde_json::from_value(legacy.clone()).expect("primary origin alone");
    assert_eq!(
        serde_json::to_value(parsed).expect("legacy interaction JSON"),
        legacy
    );
}
