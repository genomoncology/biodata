//! Published schema for `Bound`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Bound {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Bound".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "anyOf": [
            {
              "required": [
                "min"
              ]
            },
            {
              "required": [
                "max"
              ]
            }
          ],
          "description": "A bounded source range; equal min and max denote an exact boundary.",
          "properties": {
            "max": {
              "format": "uint32",
              "minimum": 0,
              "type": [
                "integer",
                "null"
              ]
            },
            "min": {
              "format": "uint32",
              "minimum": 0,
              "type": [
                "integer",
                "null"
              ]
            }
          },
          "type": "object"
        })
    }
}
