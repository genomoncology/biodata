//! Published schema for `Chromosome`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Chromosome {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Chromosome".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A primary chromosome in the caller's reference build.",
          "pattern": "^([1-9]|1[0-9]|2[0-2]|X|Y|MT)$",
          "type": "string"
        })
    }
}
impl crate::Chromosome {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
