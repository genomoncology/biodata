//! Published schema for `UncertainSpan0`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::UncertainSpan0 {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UncertainSpan0".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::Bound>();
        schemars::json_schema!({
          "description": "Zero-based interbase span retaining uncertain boundary ranges.",
          "properties": {
            "end": {
              "$ref": "#/$defs/Bound"
            },
            "start": {
              "$ref": "#/$defs/Bound"
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
