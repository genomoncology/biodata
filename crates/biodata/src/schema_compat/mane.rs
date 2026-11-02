//! Published schema for `Mane`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Mane {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Mane".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::ManeStatus>();
        schemars::json_schema!({
          "description": "A source-stated MANE pair with versioned sequence accessions.",
          "properties": {
            "ensembl": {
              "description": "Versioned Ensembl accession.",
              "pattern": "^ENST[0-9]{11}\\.[1-9][0-9]*$",
              "type": "string"
            },
            "refseq": {
              "description": "Versioned RefSeq accession.",
              "pattern": "^(NM_|NR_)([0-9]{6}|[0-9]{9})\\.[1-9][0-9]*$",
              "type": "string"
            },
            "status": {
              "$ref": "#/$defs/ManeStatus",
              "description": "Source-stated transcript status."
            }
          },
          "required": [
            "status",
            "refseq",
            "ensembl"
          ],
          "type": "object"
        })
    }
}
