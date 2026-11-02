//! Published schema for `CivicEvidenceLevelCode`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::CivicEvidenceLevelCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "CivicEvidenceLevelCode".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Source-defined CIViC evidence levels.",
          "enum": [
            "a",
            "b",
            "c",
            "d",
            "e"
          ],
          "type": "string"
        })
    }
}
