//! Published schema for `Contig`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Contig {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Contig".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A source chromosome or scaffold name without spelling normalization.",
          "pattern": "^[\\x21-\\x7e]{1,255}$",
          "type": "string"
        })
    }
}
impl crate::Contig {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
