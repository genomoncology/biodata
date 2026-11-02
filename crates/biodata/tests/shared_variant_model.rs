//! The shared variant wire matches the canonical source-qualified entity.
#[test]
fn variant_fixture_preserves_exact_wire_and_accepts_extra_fields() {
    let fixture = r#"{"representations":[{"source":"source","expression":{"syntax":"hgvs.g","value":"NC_000007.14:g.140753336A>T"},"build":"GRCh38"}],"genes":[{"label":"BRAF"}]}"#;
    let variant: biodata::Variant = serde_json::from_str(fixture).unwrap();
    assert_eq!(serde_json::to_string(&variant).unwrap(), fixture);
    let extra = r#"{"ids":[],"representations":[],"genes":[],"unsupported":true}"#;
    assert!(serde_json::from_str::<biodata::Variant>(extra).is_ok());
}
