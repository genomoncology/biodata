//! Published schema for `CosmicId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::CosmicId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "CosmicId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A COSMIC genomic, legacy transcript or noncoding variant identifier; its kind is retained.",
          "pattern": format!("^COS[VMN]{}$", crate::schema_pattern::decimal(u64::MAX)),
          "type": "string"
        })
    }
}
impl crate::CosmicId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
