//! Published schema for `Expression`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Expression {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Expression".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::ExpressionSyntax>();
        let _ = generator.subschema_for::<crate::Text>();
        schemars::json_schema!({
          "description": "Source expression checked for its reference and syntax only, without a full parser or normalization.",
          "properties": {
            "syntax": {
              "$ref": "#/$defs/ExpressionSyntax"
            },
            "syntax_version": {
              "anyOf": [
                {
                  "$ref": "#/$defs/Text"
                },
                {
                  "type": "null"
                }
              ]
            },
            "value": {
              "$ref": "#/$defs/Text"
            }
          },
          "required": [
            "syntax",
            "value"
          ],
          "type": "object"
        })
    }
}
