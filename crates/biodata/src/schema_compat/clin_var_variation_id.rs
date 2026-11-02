//! Published schema for `ClinVarVariationId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ClinVarVariationId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClinVarVariationId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": " A ClinVar variation record, which may represent a compound.",
          "pattern": format!("^clinvar:{}$", crate::schema_pattern::decimal(u32::MAX.into())),
          "type": "string"
        })
    }
}
impl crate::ClinVarVariationId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
