//! Published schema for `Position1`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Position1 {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Position1".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A positive one-based position, without an assembly or biological meaning.",
          "format": "uint32",
          "maximum": u32::MAX,
          "minimum": 1,
          "type": "integer"
        })
    }
}
