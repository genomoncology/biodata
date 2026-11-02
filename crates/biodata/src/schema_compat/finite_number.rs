//! Published schema for `FiniteNumber`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::FiniteNumber {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FiniteNumber".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A finite decimal that JSON can represent without losing its value to null.",
          "format": "double",
          "type": "number"
        })
    }
}
