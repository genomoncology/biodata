//! Published schema for `Span0`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Span0 {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Span0".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A zero-based half-open span with ordered bounds; empty spans are valid.",
          "properties": {
            "end": {
              "format": "uint32",
              "maximum": u32::MAX,
              "minimum": 0,
              "type": "integer"
            },
            "start": {
              "format": "uint32",
              "maximum": u32::MAX,
              "minimum": 0,
              "type": "integer"
            }
          },
          "required": [
            "start",
            "end"
          ],
          "type": "object"
        })
    }
}
