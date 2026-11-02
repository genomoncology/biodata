//! The shared public fixture fixes the wire form of joint variant values.
use biodata::{
    AlterationReferenceError, Bound, Category, Coding, CopyChange, CopyNumberChange,
    CopyNumberCount, GeneRef, HgvsText, Outcome, PartialDate, Position1, ProteinSite, Region,
    RegionKind, Text, TranscriptConsequence, UncertainSpan0, VariantRef, alteration,
};

#[test]
fn shared_bounds_keep_partial_uncertainty_and_refuse_impossible_ranges() {
    for (wire, expected) in [
        (r#"{"min":0}"#, true),
        (r#"{"max":12}"#, true),
        (r#"{"min":4,"max":4}"#, true),
        (r#"{}"#, false),
        (r#"{"min":5,"max":4}"#, false),
        (r#"{"min":0,"other":1}"#, true),
    ] {
        assert_eq!(
            serde_json::from_str::<Bound>(wire).is_ok(),
            expected,
            "{wire}"
        );
    }
    let overlapping = UncertainSpan0::new(
        Bound::new(Some(20), None).unwrap(),
        Bound::new(Some(15), None).unwrap(),
    );
    assert!(overlapping.is_ok());
    assert!(
        UncertainSpan0::new(
            Bound::new(Some(20), None).unwrap(),
            Bound::new(None, Some(15)).unwrap(),
        )
        .is_err()
    );
}

#[test]
fn relative_copy_change_codes_keep_the_vrs_efo_vocabulary() {
    for (value, wire, efo) in [
        (
            CopyChange::CompleteGenomicLoss,
            "complete_genomic_loss",
            "EFO:0030069",
        ),
        (CopyChange::HighLevelLoss, "high_level_loss", "EFO:0020073"),
        (CopyChange::LowLevelLoss, "low_level_loss", "EFO:0030068"),
        (CopyChange::Loss, "loss", "EFO:0030067"),
        (
            CopyChange::RegionalBasePloidy,
            "regional_base_ploidy",
            "EFO:0030064",
        ),
        (CopyChange::Gain, "gain", "EFO:0030070"),
        (CopyChange::LowLevelGain, "low_level_gain", "EFO:0030071"),
        (CopyChange::HighLevelGain, "high_level_gain", "EFO:0030072"),
    ] {
        assert_eq!(value.efo_code(), efo);
        assert_eq!(serde_json::to_value(value).unwrap(), wire);
        assert_eq!(
            serde_json::from_str::<CopyChange>(&format!("\"{wire}\"")).unwrap(),
            value
        );
    }
}

#[test]
fn coding_checks_each_named_system_and_keeps_so_edit_distinct() {
    for (system, code) in [
        ("acmg_amp_2015", "pathogenic"),
        ("clingen_low_penetrance_2024", "established_risk_allele"),
        ("amp_asco_cap_2017_tier", "tier_i"),
        ("amp_asco_cap_2017_level", "level_a"),
        ("clingen_cgc_vicc_2022", "oncogenic"),
        ("clinvar_review_status", "expert_panel"),
        ("civic_evidence_level", "a"),
        ("oncokb_level", "level_r1"),
        ("oncokb_oncogenicity", "likely_neutral"),
        ("clinvar_classification_type", "somatic_clinical_impact"),
        ("clinvar_allele_origin", "de_novo"),
        ("so", "SO:0001583"),
        ("hpo", "HP:0000006"),
        ("clingen_gene_validity_2017", "definitive"),
        ("clingen_dosage_score", "30"),
    ] {
        let coding = Coding::from_system_code(system, code).unwrap();
        let wire = serde_json::json!({"system": system, "code": code});
        assert_eq!(serde_json::to_value(coding).unwrap(), wire);
        assert_eq!(serde_json::from_value::<Coding>(wire).unwrap(), coding);
    }
    for (system, code) in [
        ("so", "SO:9999999"),
        ("oncokb_level", "level_0"),
        ("clingen_dosage_score", "5"),
        ("unknown", "x"),
    ] {
        assert!(Coding::from_system_code(system, code).is_err());
    }
}

#[test]
fn partial_dates_refuse_signed_components_and_keep_canonical_precision() {
    for date in ["2024", "2024-01", "2024-01-31"] {
        let parsed: PartialDate = date.parse().unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), date);
        assert_eq!(
            serde_json::from_value::<PartialDate>(serde_json::json!(date))
                .unwrap()
                .to_string(),
            date
        );
    }
    for date in ["+123", "2024-+1", "2024-01-+1", "2024-02-30"] {
        assert!(date.parse::<PartialDate>().is_err(), "{date}");
        assert!(serde_json::from_value::<PartialDate>(serde_json::json!(date)).is_err());
    }
}

#[test]
fn plain_text_refuses_empty_value_with_shared_code() {
    assert_eq!("".parse::<Text>().unwrap_err().code(), "text.empty");
}

#[test]
fn source_hgvs_text_keeps_reference_and_refuses_malformed_forms() {
    for (source, code) in [
        ("", "hgvs.text.empty"),
        ("NM_000001.1", "hgvs.reference.missing"),
        ("bad:c.1A>T", "hgvs.reference.invalid"),
        ("NM_000001.1(??):c.1A>T", "hgvs.inner_reference.invalid"),
        ("NM_000001.1(??:c.1A>T", "hgvs.inner_reference.invalid"),
        ("NM_000001.1:x.1A>T", "hgvs.syntax.unknown"),
        ("NM_000001.1:c.", "hgvs.body.empty"),
        ("NM_000001.1:c.1A>\nT", "hgvs.body.unprintable"),
    ] {
        assert_eq!(source.parse::<HgvsText>().unwrap_err().code(), code);
        assert!(serde_json::from_value::<HgvsText>(serde_json::json!(source)).is_err());
    }
    let source = "NM_000001.1(GENE):c.1A>T";
    let value: HgvsText = serde_json::from_value(serde_json::json!(source)).unwrap();
    assert_eq!(value.as_str(), source);
    assert_eq!(value.accession().unwrap().to_string(), "NM_000001.1");
}

#[test]
fn reference_leaf_and_profile_values_keep_source_identity() {
    let gene: biodata::SourceGeneRef =
        serde_json::from_value(serde_json::json!({"label":"GENE"})).unwrap();
    let allele = VariantRef::allele("CA123643".parse().unwrap());
    let notation = VariantRef::allele_hgvs("NM_000001.1:c.1A>T".parse().unwrap());
    let protein = VariantRef::protein(gene.clone(), Text::new("GENE p.Met1Leu").unwrap());
    assert!(
        serde_json::from_value::<VariantRef>(
            serde_json::json!({"level":"protein", "gene":{"label":"GENE"}})
        )
        .is_err(),
        "protein references require a stated expression"
    );
    for value in [&allele, &notation, &protein] {
        let wire = serde_json::to_value(value).unwrap();
        assert_eq!(
            serde_json::from_value::<VariantRef>(wire.clone()).unwrap(),
            *value
        );
        assert_eq!(
            serde_json::from_str::<VariantRef>(&serde_json::to_string(value).unwrap()).unwrap(),
            *value
        );
        assert!(matches!(wire["level"].as_str(), Some("allele" | "protein")));
    }
    let profile = VariantRef::profile(
        Text::new("profile-1").unwrap(),
        Text::new("source profile").unwrap(),
        vec![allele, notation, protein].into_boxed_slice(),
    )
    .unwrap();
    let wire = serde_json::to_value(&profile).unwrap();
    assert_eq!(wire["members"].as_array().unwrap().len(), 3);
    assert_eq!(serde_json::from_value::<VariantRef>(wire).unwrap(), profile);
    assert!(
        VariantRef::profile(
            Text::new("profile-2").unwrap(),
            Text::new("nested profile").unwrap(),
            vec![profile].into_boxed_slice(),
        )
        .is_err()
    );
}

#[test]
fn consequence_and_category_edges_refuse_unstated_details() {
    let base = serde_json::json!({
        "transcript": "NM_000001.1",
        "consequences": [{"system":"so","code":"SO:0001583"}],
        "attribution": {"source":"synthetic source"}
    });
    for (field, replacement, code) in [
        (
            "transcript",
            serde_json::json!("NC_000001.11"),
            "transcript.accession.invalid",
        ),
        (
            "consequences",
            serde_json::json!([]),
            "consequence.list.empty",
        ),
        (
            "consequences",
            serde_json::json!([{"system":"hpo","code":"HP:0000006"}]),
            "consequence.so.unknown",
        ),
    ] {
        let mut wire = base.clone();
        wire[field] = replacement;
        let error = serde_json::from_value::<TranscriptConsequence>(wire).unwrap_err();
        assert!(error.to_string().contains(code), "{error}");
    }
    let invalid_site = serde_json::json!({
        "gene":{"label":"GENE"},"position":600,"end":599
    });
    assert!(serde_json::from_value::<ProteinSite>(invalid_site).is_err());
    let gene: biodata::SourceGeneRef =
        serde_json::from_value(serde_json::json!({"label":"GENE"})).unwrap();
    let other: GeneRef = serde_json::from_value(serde_json::json!({"label":"OTHER"})).unwrap();
    let site =
        ProteinSite::new(other, None, None, Position1::new(600).unwrap(), None, None).unwrap();
    assert!(
        VariantRef::categorical(
            gene.clone(),
            Category::Residue,
            None,
            None,
            None,
            Some(site),
            Text::new("GENE residue 600").unwrap()
        )
        .is_err()
    );
    assert!(
        VariantRef::categorical(
            gene,
            Category::ExonMutation,
            None,
            None,
            Some(Coding::from_system_code("acmg_amp_2015", "pathogenic").unwrap()),
            None,
            Text::new("GENE exon mutation").unwrap()
        )
        .is_err()
    );
}

#[test]
fn categorical_and_consequence_deserialization_refuse_invented_details() {
    for wire in [
        r#"{"level":"categorical","gene":{"label":"GENE"},"category":"amplification","partner":{"label":"OTHER"},"label":"GENE Amplification"}"#,
        r#"{"level":"categorical","gene":{"label":"GENE"},"category":"fusion","region":{"kind":"exon","number":1,"total":2},"label":"GENE Fusion"}"#,
        r#"{"level":"categorical","gene":{"label":"GENE"},"category":"residue","label":"GENE residue"}"#,
        r#"{"level":"allele"}"#,
        r#"{"level":"protein","gene":{"label":"GENE"}}"#,
    ] {
        assert!(serde_json::from_str::<VariantRef>(wire).is_err(), "{wire}");
    }
    let consequence = serde_json::json!({
        "transcript": "NM_000001.1",
        "consequences": [{"system":"so","code":"SO:0001583"},{"system":"so","code":"SO:0001583"}],
        "attribution": {"source":"synthetic source"}
    });
    assert!(serde_json::from_value::<TranscriptConsequence>(consequence).is_err());
}

#[test]
fn alteration_categories_convert_only_with_their_stated_context() {
    let fixtures: Vec<serde_json::Value> = include_str!("cases/public/alteration-event.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let parsed = |source: &str| match alteration::parse(source).outcome {
        Outcome::Parsed(value) => value,
        other => panic!("{source}: {other:?}"),
    };
    let mut checked = 0;
    for case in fixtures.iter().filter(|case| case["outcome"] == "parsed") {
        let source = case["input"].as_str().unwrap();
        let conversion =
            VariantRef::from_alteration(&parsed(source), Text::new(source).unwrap(), None);
        match source {
            "GENE Amplification" => assert_eq!(
                conversion.unwrap().category(),
                Some(Category::Amplification)
            ),
            "GENE Deletion" => assert_eq!(conversion.unwrap().category(), Some(Category::Deletion)),
            "GENE Fusion" => assert_eq!(conversion.unwrap().category(), Some(Category::Fusion)),
            _ => assert_eq!(
                conversion.unwrap_err().code(),
                AlterationReferenceError::Unsupported.code(),
                "{source}"
            ),
        }
        checked += 1;
    }
    assert_eq!(checked, 43);
    let exon = parsed("EGFR Exon 19 Deletion");
    assert!(matches!(
        VariantRef::from_alteration(&exon, Text::new("EGFR Exon 19 Deletion").unwrap(), None),
        Err(AlterationReferenceError::RegionRequired)
    ));
    // The parsed label names exon 19 only; total 28 is synthetic caller context.
    let region = Region::new(
        RegionKind::Exon,
        Position1::new(19).unwrap(),
        Position1::new(28).unwrap(),
    )
    .unwrap();
    let value = VariantRef::from_alteration(
        &exon,
        Text::new("EGFR Exon 19 Deletion").unwrap(),
        Some(region),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(value).unwrap()["edit"],
        serde_json::json!({"system":"so","code":"SO:0000159"})
    );
}

#[test]
fn protein_site_range_updates_refuse_reversed_end_and_keep_fixture_wire() {
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/variant/joint.json")).unwrap();
    let fixture = cases
        .iter()
        .find(|case| case["name"] == "joint-categorical-residue-control")
        .unwrap();
    let mut site: ProteinSite = serde_json::from_value(fixture["value"]["site"].clone()).unwrap();
    assert_eq!(site.position().get(), 600);
    assert_eq!(site.end(), None);
    site.set_end(Some(Position1::new(601).unwrap())).unwrap();
    assert_eq!(
        site.set_position(Position1::new(602).unwrap())
            .unwrap_err()
            .to_string(),
        "protein.site.order"
    );
    assert_eq!(site.position().get(), 600);
    site.set_position(Position1::new(601).unwrap()).unwrap();
    assert_eq!(site.position().get(), 601);
    site.set_position(Position1::new(600).unwrap()).unwrap();
    site.set_end(None).unwrap();
    assert_eq!(
        serde_json::to_string(&site).unwrap(),
        r#"{"gene":{"label":"GENE"},"position":600}"#
    );
    assert_eq!(
        site.set_end(Some(Position1::new(599).unwrap()))
            .unwrap_err()
            .to_string(),
        "protein.site.order"
    );
    assert_eq!(site.end(), None);
    assert_eq!(
        serde_json::to_string(&site).unwrap(),
        r#"{"gene":{"label":"GENE"},"position":600}"#
    );
    assert!(
        serde_json::from_value::<ProteinSite>(serde_json::json!({
            "gene":{"label":"GENE"},"position":600,"end":599
        }))
        .is_err()
    );
}

#[test]
fn shared_fixture_round_trips_each_joint_value() {
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/variant/joint.json")).unwrap();
    let mut checked = [0usize; 5];
    for case in cases {
        let value = &case["value"];
        match case["kind"].as_str() {
            Some("joint_transcript_consequence") => {
                let parsed: TranscriptConsequence = serde_json::from_value(value.clone()).unwrap();
                assert_eq!(serde_json::to_value(parsed).unwrap(), *value);
                checked[0] += 1;
            }
            Some("joint_copy_count") => {
                let parsed: CopyNumberCount = serde_json::from_value(value.clone()).unwrap();
                assert_eq!(serde_json::to_value(parsed).unwrap(), *value);
                checked[1] += 1;
            }
            Some("joint_copy_change") => {
                let parsed: CopyNumberChange = serde_json::from_value(value.clone()).unwrap();
                assert_eq!(parsed.copy_change, CopyChange::Gain);
                assert_eq!(serde_json::to_value(parsed).unwrap(), *value);
                checked[2] += 1;
            }
            Some("joint_categorical") => {
                let parsed: VariantRef = serde_json::from_value(value.clone()).unwrap();
                assert!(parsed.category().is_some());
                assert_eq!(serde_json::to_value(parsed).unwrap(), *value);
                checked[3] += 1;
            }
            Some("joint_profile") => {
                let parsed: VariantRef = serde_json::from_value(value.clone()).unwrap();
                assert_eq!(serde_json::to_value(parsed).unwrap(), *value);
                checked[4] += 1;
            }
            _ => {}
        }
    }
    assert_eq!(checked, [1, 1, 1, 6, 1]);
}
