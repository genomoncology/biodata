//! Published schema for `Allele`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Allele {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Allele".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::Accession>();
        let _ = generator.subschema_for::<crate::Sequence>();
        let _ = generator.subschema_for::<crate::SequenceLocation>();
        let _ = generator.subschema_for::<crate::Span0>();
        schemars::json_schema!({
          "description": "Source-stated interbase replacement; equality never claims normalized equivalence.",
          "properties": {
            "alternate": {
              "$ref": "#/$defs/Sequence"
            },
            "location": {
              "$ref": "#/$defs/SequenceLocation"
            },
            "reference": {
              "$ref": "#/$defs/Sequence"
            }
          },
          "required": [
            "location",
            "reference",
            "alternate"
          ],
          "type": "object"
        })
    }
}
