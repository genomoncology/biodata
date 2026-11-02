//! Published schema for `ClinVarAlleleId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ClinVarAlleleId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClinVarAlleleId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": " An allele identifier inside a ClinVar record, not a variant identifier.",
          "pattern": format!("^{}$", crate::schema_pattern::decimal(u32::MAX.into())),
          "type": "string"
        })
    }
}
impl crate::ClinVarAlleleId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
