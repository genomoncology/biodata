//! Published schema for `VrsId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::VrsId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "VrsId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A source-supplied VRS allele identifier; construction never computes a digest.",
          "pattern": "^ga4gh:VA\\.[A-Za-z0-9_-]{32}$",
          "type": "string"
        })
    }
}
impl crate::VrsId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
