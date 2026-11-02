//! Published schema for `Strand`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Strand {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Strand".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Source-stated forward or reverse strand.",
          "enum": [
            "+",
            "-"
          ],
          "type": "string",
          "x-enum-descriptions": [
            " Forward strand.",
            " Reverse strand."
          ],
          "x-enum-open": true
        })
    }
}
