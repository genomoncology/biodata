//! Published schema for `Text`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Text {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Text".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Bounded nonempty text with surrounding Unicode whitespace removed.",
          "maxLength": 1048576,
          "minLength": 1,
          "pattern": "^(?![\\s\\S]*[\\x00-\\x1f\\x7f-\\x9f\\u061c\\u200e\\u200f\\u202a-\\u202e\\u2066-\\u2069])",
          "type": "string"
        })
    }
}
impl crate::Text {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
