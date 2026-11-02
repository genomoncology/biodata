//! Published schema for `GenomicInterval`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::GenomicInterval {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GenomicInterval".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::Build>();
        let _ = generator.subschema_for::<crate::Chromosome>();
        let _ = generator.subschema_for::<crate::Span0>();
        let _ = generator.subschema_for::<crate::Strand>();
        schemars::json_schema!({
          "description": "An assembly-qualified zero-based half-open source interval.",
          "properties": {
            "build": {
              "anyOf": [
                {
                  "$ref": "#/$defs/Build"
                },
                {
                  "type": "null"
                }
              ],
              "description": "Source-stated assembly, omitted when absent."
            },
            "chromosome": {
              "$ref": "#/$defs/Chromosome",
              "description": "Canonical primary chromosome."
            },
            "span": {
              "$ref": "#/$defs/Span0",
              "description": "Checked zero-based half-open bounds; an empty span remains valid."
            },
            "strand": {
              "anyOf": [
                {
                  "$ref": "#/$defs/Strand"
                },
                {
                  "type": "null"
                }
              ],
              "description": "Source-stated strand, if available."
            }
          },
          "required": [
            "chromosome",
            "span"
          ],
          "type": "object"
        })
    }
}
