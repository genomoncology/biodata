//! Published schema for `Representation`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Representation {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Representation".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::Accession>();
        let _ = generator.subschema_for::<crate::Allele>();
        let _ = generator.subschema_for::<crate::Build>();
        let _ = generator.subschema_for::<crate::Expression>();
        let _ = generator.subschema_for::<crate::ExpressionSyntax>();
        let _ = generator.subschema_for::<crate::Sequence>();
        let _ = generator.subschema_for::<crate::SequenceLocation>();
        let _ = generator.subschema_for::<crate::Span0>();
        let _ = generator.subschema_for::<crate::Text>();
        schemars::json_schema!({
          "description": "One source's own expression, assembly and optional source-stated interbase allele.",
          "properties": {
            "allele": {
              "anyOf": [
                {
                  "$ref": "#/$defs/Allele"
                },
                {
                  "type": "null"
                }
              ],
              "description": "Source-supplied allele, without local normalization or equivalence inference."
            },
            "build": {
              "anyOf": [
                {
                  "$ref": "#/$defs/Build"
                },
                {
                  "type": "null"
                }
              ],
              "description": "Assembly stated by the source, omitted when absent."
            },
            "expression": {
              "$ref": "#/$defs/Expression",
              "description": "Published notation kept unchanged."
            },
            "source": {
              "$ref": "#/$defs/Text",
              "description": "Source asserting this form."
            }
          },
          "required": [
            "source",
            "expression"
          ],
          "type": "object"
        })
    }
}
