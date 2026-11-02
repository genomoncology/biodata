//! pgx-types: checked public requests and neutral records, with no I/O.
use serde_json::json;

fn roundtrip<T: serde::Serialize + serde::de::DeserializeOwned>(
    value: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(serde_json::from_value::<T>(value)?)
}
fn recommendation() -> serde_json::Value {
    json!(
        {"recommendationid":{"namespace":"CPIC","id":42},"lookupkey":{"TPMT":"synthetic first","NUDT15":"synthetic second"},"drugname":"synthetic drug",
        "guidelinename":"synthetic guide","guidelineurl":"https://example.invalid/guide","implications":{"TPMT":"synthetic implication"},
        "drugrecommendation":"  Synthetic source text.\n","classification":"synthetic classification",
        "phenotypes":{"TPMT":"synthetic phenotype"},"activityscore":{"TPMT":"n/a","NUDT15":"0.5"},"population":"synthetic population",
        "comments":null}
    )
}
fn diplotype() -> serde_json::Value {
    json!(
        {"genesymbol":"TPMT","diplotype":"*3A/*1","function1":"synthetic","function2":"synthetic","activityvalue1":"n/a",
        "activityvalue2":"n/a","totalactivityscore":"n/a","description":"synthetic","diplotypekey":{"TPMT":{"*3A":1,"*1":1}},
        "generesult":"synthetic result","ehrpriority":"synthetic priority","consultationtext":"  Synthetic consultation.\n",
        "lookupkey":{"TPMT":"synthetic"}}
    )
}
fn annotation() -> serde_json::Value {
    json!(
        {"id":{"namespace":"ClinPGx","id":42},"accessionId":"PA1234","levelOfEvidence":{"id":{"namespace":"ClinPGx",
        "id":3},"resource":"synthetic","term":"synthetic evidence",
        "termId":"synthetic-term"},"location":{"id":{"namespace":"ClinPGx","id":7},"copyNumber":"synthetic",
        "diplotypes":[{"id":{"namespace":"ClinPGx","id":99}}],"displayName":"synthetic location",
        "genes":[{"objCls":"Gene","id":"PA1","symbol":"TPMT","name":"synthetic"}],"gpPosition":-1,"haplotypes":[{"objCls":"Haplotype",
        "id":"PA2","symbol":"*3A","name":"synthetic"}],"linkedObjects":[{"objCls":"Haplotype","id":"PA2","symbol":"*3A",
        "name":"synthetic"}],"refSeqPosition":-1,"tagGene":false,"type":"synthetic"},"name":"synthetic annotation",
        "relatedChemicals":[{"objCls":"Chemical","id":"PA3","name":"synthetic first"},{"objCls":"Chemical","id":"PA4",
        "name":"synthetic second"}],"relatedChemicalsLogic":"synthetic relation",
        "types":["synthetic first","synthetic second"]}
    )
}
#[test]
fn native_records_keep_joint_context_source_text_and_literal_not_applicable()
-> Result<(), Box<dyn std::error::Error>> {
    use biodata::{
        PgxDiplotypeResult, PgxDrugRecommendation, PgxGuidelineReference, PgxPair,
        PgxRecommendation, Statement,
    };
    for value in [recommendation(), {
        let mut v = recommendation();
        v["comments"] = json!("  Synthetic comments.\n");
        v
    }] {
        let mut expected = value.clone();
        if expected
            .get("comments")
            .is_some_and(serde_json::Value::is_null)
        {
            expected.as_object_mut().ok_or("object")?.remove("comments");
        }
        assert_eq!(roundtrip::<PgxRecommendation>(value.clone())?, expected);
        assert!(serde_json::from_value::<Statement>(value).is_err());
    }
    let d = diplotype();
    assert_eq!(roundtrip::<PgxDiplotypeResult>(d.clone())?, d);
    assert!(serde_json::from_value::<Statement>(d).is_err());
    let mut table = recommendation();
    let object = table.as_object_mut().ok_or("object")?;
    for key in [
        "recommendationid",
        "drugname",
        "guidelinename",
        "guidelineurl",
    ] {
        object.remove(key);
    }
    object.extend(serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(json!(
        {"comments":"synthetic comments","id":{"namespace":"CPIC","id":91},"guidelineid":{"namespace":"CPIC",
        "id":3},"drugid":"RxNorm:10324","allelestatus":{},"version":4,
        "dosinginformation":true,"alternatedrugavailable":false,"otherprescribingguidance":false}
    ))?);
    assert_eq!(roundtrip::<PgxDrugRecommendation>(table.clone())?, table);
    assert!(serde_json::from_value::<PgxRecommendation>(table).is_err());
    let pair = json!(
        {"pairid":{"namespace":"CPIC","id":3},"drugid":"RxNorm:10324","drugname":"synthetic","genesymbol":"TPMT","guidelinename":null,"guidelineurl":null,
        "cpiclevel":"B/C","clinpgxlevel":"synthetic distinct level","pgxtesting":null,"pmids":null,"usedforrecommendation":"No",
        "provisional":false}
    );
    let mut expected = pair.clone();
    for name in ["guidelinename", "guidelineurl", "pgxtesting", "pmids"] {
        expected.as_object_mut().ok_or("object")?.remove(name);
    }
    assert_eq!(roundtrip::<PgxPair>(pair)?, expected);
    let guide = json!(
        {"guideline_name":"synthetic","guideline_url":"https://example.invalid/guide","drugs":["second","first","second"],
        "genes":[{"url":"https://example.invalid/gene","symbol":"TPMT"}]}
    );
    assert_eq!(roundtrip::<PgxGuidelineReference>(guide.clone())?, guide);
    Ok(())
}
#[test]
fn annotation_tiers_preserve_native_arrays_missing_locations_and_base_history()
-> Result<(), Box<dyn std::error::Error>> {
    use biodata::{PgxAnnotationDetail, PgxAnnotationHeader, PgxAnnotationReference};
    let header = annotation();
    assert_eq!(roundtrip::<PgxAnnotationHeader>(header.clone())?, header);
    let mut detail = header.clone();
    let object = detail.as_object_mut().ok_or("object")?;
    object.extend(serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(json!(
        {"allelePhenotypes":[{"id":{"namespace":"ClinPGx","id":1},"allele":"synthetic allele","limitedEvidence":false,"phenotype":"  Synthetic text.\n"}],
        "conflictingVariantAnnotationIds":[{"namespace":"ClinPGx","id":12},{"namespace":"ClinPGx",
        "id":13}],"history":[{"id":{"namespace":"ClinPGx","id":1},"date":"2026-01-01","type":"synthetic",
        "version":3},{"id":{"namespace":"ClinPGx","id":2},
        "date":"2026-02-01","type":"synthetic","version":4,"description":"synthetic change"}],"objCls":"ClinicalAnnotation",
        "overrideLevel":false,"pediatric":false,"relatedAlleles":[{"objCls":"Haplotype","id":"PA2","name":"synthetic","symbol":"*1"}],
        "score":1.5,"scoreDetails":"synthetic details"}
    ))?);
    assert_eq!(roundtrip::<PgxAnnotationDetail>(detail.clone())?, detail);
    assert_eq!(roundtrip::<PgxAnnotationHeader>(detail)?, header);
    let reference = json!({"objCls":"GuidelineAnnotation","id":"PA5","name":"synthetic",});
    assert_eq!(
        roundtrip::<PgxAnnotationReference>(reference.clone())?,
        reference
    );
    Ok(())
}

#[test]
fn declared_expressions_and_source_map_keys_keep_their_exact_spelling()
-> Result<(), Box<dyn std::error::Error>> {
    use biodata::{PgxDiplotypeExpression, PgxGeneKey};
    for spelling in ["*3A/*1", " *2x2/*4 ", "*1/*3+*6"] {
        let v: PgxDiplotypeExpression = spelling.parse()?;
        assert_eq!(v.as_str(), spelling);
        assert_eq!(v.to_string(), spelling);
        assert_eq!(
            roundtrip::<PgxDiplotypeExpression>(json!(spelling))?,
            spelling
        );
    }
    assert!(" ".parse::<PgxDiplotypeExpression>().is_err());
    let upper: PgxGeneKey = "TPMT".parse()?;
    let lower: PgxGeneKey = "tpmt".parse()?;
    assert_ne!(upper, lower);
    assert_eq!(upper.as_str(), "TPMT");
    assert_eq!(lower.to_string(), "tpmt");
    assert!("not a gene key".parse::<PgxGeneKey>().is_err());
    assert_eq!(roundtrip::<PgxGeneKey>(json!("TPMT"))?, "TPMT");
    Ok(())
}
