//! Published schema for `Sequence`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Sequence {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Sequence".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A literal IUPAC DNA sequence; empty sequences express insertions or deletions.",
          "pattern": "^[ACGTNRYSWKMBDHV]*$",
          "type": "string"
        })
    }
}
impl crate::Sequence {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
