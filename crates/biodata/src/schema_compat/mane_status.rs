//! Published schema for `ManeStatus`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ManeStatus {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ManeStatus".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "The source-stated MANE transcript status.",
          "enum": [
            "select",
            "plus_clinical"
          ],
          "type": "string",
          "x-enum-descriptions": [
            " MANE Select transcript pair.",
            " MANE Plus Clinical transcript pair."
          ],
          "x-enum-open": true
        })
    }
}
