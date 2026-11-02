//! Source subjects expose one level-discriminated schema without changing their wire.
#![cfg(feature = "schema")]
use biodata::{Subject, VariantRef};
use serde_json::json;

#[test]
fn subject_preserves_variant_branches_and_adds_explicit_source_levels()
-> Result<(), Box<dyn std::error::Error>> {
    let subject = schemars::schema_for!(Subject).to_value();
    let variant = schemars::schema_for!(VariantRef).to_value();
    let branches = subject
        .get("oneOf")
        .ok_or("Subject oneOf absent")?
        .as_array()
        .ok_or("Subject oneOf absent")?;
    let variants = variant
        .get("oneOf")
        .ok_or("VariantRef oneOf absent")?
        .as_array()
        .ok_or("VariantRef oneOf absent")?;
    assert_eq!(
        branches
            .get(..variants.len())
            .ok_or("variant branches absent")?,
        variants,
        "every VariantRef branch stays byte-identical and ordered"
    );
    assert_eq!(
        branches.len(),
        variants.len() + 2,
        "only gene and region branches are added"
    );
    for (branch, level) in branches
        .get(variants.len()..)
        .ok_or("source branches absent")?
        .iter()
        .zip(["gene", "region"])
    {
        assert_eq!(
            branch
                .pointer("/properties/level")
                .ok_or("level property absent")?,
            &json!({"const":level,"type":"string"}),
            "source branch has a direct discriminator"
        );
        assert_eq!(
            branch.get("additionalProperties"),
            Some(&json!(false)),
            "source object field policy stays unchanged"
        );
    }
    assert!(
        subject.get("anyOf").is_none(),
        "Subject has no outer untagged union"
    );
    let validator = jsonschema::validator_for(&subject)?;
    for wire in [
        json!({"level":"categorical","gene":{"label":"SYNTH1"},"category":"deletion","label":"synthetic deletion"}),
        json!({"level":"gene","label":"SYNTH1"}),
        json!({"level":"region","interval":{"build":"GRCh38","chromosome":"1","span":{"start":0,"end":1}},"label":"synthetic region"}),
    ] {
        let parsed: Subject = serde_json::from_value(wire.clone())?;
        assert_eq!(
            serde_json::to_value(parsed)?,
            wire,
            "source subject wire stays unchanged"
        );
        assert!(validator.is_valid(&wire), "schema accepts the source level");
    }
    for wire in [
        json!({"label":"SYNTH1"}),
        json!({"level":"other","label":"SYNTH1"}),
        json!({"level":"gene","label":"SYNTH1","extra":true}),
    ] {
        assert!(
            !validator.is_valid(&wire),
            "schema refuses malformed source level"
        );
        assert!(
            serde_json::from_value::<Subject>(wire).is_err(),
            "parser preserves its refusal"
        );
    }
    Ok(())
}

#[test]
fn subject_uses_canonical_types_after_schema_name_collisions()
-> Result<(), Box<dyn std::error::Error>> {
    macro_rules! collision {
        ($ty:ident, $name:literal) => {
            struct $ty;
            impl schemars::JsonSchema for $ty {
                fn schema_name() -> std::borrow::Cow<'static, str> {
                    $name.into()
                }
                fn schema_id() -> std::borrow::Cow<'static, str> {
                    concat!("subject-test-collision-", $name).into()
                }
                fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                    schemars::json_schema!({"type":"string"})
                }
            }
        };
    }
    collision!(VariantCollision, "VariantRef");
    collision!(GeneCollision, "GeneSubjectWire");
    collision!(RegionCollision, "RegionSubjectWire");
    let mut generator = schemars::SchemaGenerator::default();
    generator.subschema_for::<VariantCollision>();
    generator.subschema_for::<GeneCollision>();
    generator.subschema_for::<RegionCollision>();
    let subject = generator.root_schema_for::<Subject>().to_value();
    let branches = subject
        .get("oneOf")
        .and_then(serde_json::Value::as_array)
        .ok_or("Subject oneOf absent")?;
    let variants = schemars::schema_for!(VariantRef).to_value();
    let variants = variants
        .get("oneOf")
        .and_then(serde_json::Value::as_array)
        .ok_or("VariantRef oneOf absent")?;
    assert_eq!(
        branches.len(),
        variants.len() + 2,
        "consumer schema names cannot replace canonical Subject branches"
    );
    let validator = jsonschema::validator_for(&subject)?;
    for wire in [
        json!({"level":"protein","gene":{"label":"SYNTH1"},"label":"synthetic protein"}),
        json!({"level":"gene","label":"SYNTH1"}),
        json!({"level":"region","interval":{"build":"GRCh38","chromosome":"1","span":{"start":0,"end":1}},"label":"synthetic region"}),
    ] {
        assert!(
            validator.is_valid(&wire),
            "consumer schema names cannot alter Subject validation"
        );
    }
    Ok(())
}
