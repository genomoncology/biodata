//! Published schema for `UniProtFeature`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::UniProtFeature {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UniProtFeature".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::AcmgAmp2015Code>();
        let _ = generator.subschema_for::<crate::AminoAcids>();
        let _ = generator.subschema_for::<crate::AmpAscoCap2017LevelCode>();
        let _ = generator.subschema_for::<crate::AmpAscoCap2017TierCode>();
        let _ = generator.subschema_for::<crate::Bound>();
        let _ = generator.subschema_for::<crate::CivicEvidenceLevelCode>();
        let _ = generator.subschema_for::<crate::ClingenCgcVicc2022Code>();
        let _ = generator.subschema_for::<crate::ClingenLowPenetrance2024Code>();
        let _ = generator.subschema_for::<crate::AlleleOrigin>();
        let _ = generator.subschema_for::<crate::ReviewStatus>();
        let _ = generator.subschema_for::<crate::Coding>();
        let _ = generator.subschema_for::<crate::OncokbLevelCode>();
        let _ = generator.subschema_for::<crate::OncokbOncogenicityCode>();
        let _ = generator.subschema_for::<crate::Term>();
        let _ = generator.subschema_for::<crate::Text>();
        let _ = generator.subschema_for::<crate::UncertainSpan0>();
        let _ = generator.subschema_for::<crate::UniProtIsoform>();
        feature_body()
    }
}

fn feature_body() -> Schema {
    schemars::json_schema!({
      "description": "Shared native feature contract without source label remapping.",
      "properties": {
        "alternative": {
          "description": "Every stated alternative, without translation.",
          "items": {
            "$ref": "#/$defs/AminoAcids"
          },
          "type": "array"
        },
        "description": {
          "anyOf": [
            {
              "$ref": "#/$defs/Text"
            },
            {
              "type": "null"
            }
          ],
          "description": "Complete source description."
        },
        "evidence": {
          "description": "All original source evidence references.",
          "items": {
            "$ref": "#/$defs/Text"
          },
          "type": "array"
        },
        "id": {
          "anyOf": [
            {
              "$ref": "#/$defs/Text"
            },
            {
              "type": "null"
            }
          ],
          "description": "Native feature identifier."
        },
        "kind": {
          "$ref": "#/$defs/Term",
          "description": "Source feature label; coding only for documented exact feature types."
        },
        "original": {
          "anyOf": [
            {
              "$ref": "#/$defs/AminoAcids"
            },
            {
              "type": "null"
            }
          ],
          "description": "Source original residues, when supplied."
        },
        "protein": {
          "$ref": "#/$defs/UniProtIsoform",
          "description": "Exact source sequence identity."
        },
        "span": {
          "$ref": "#/$defs/UncertainSpan0",
          "description": "Zero-based interbase coordinates retaining uncertain ranges."
        }
      },
      "required": [
        "protein",
        "span",
        "kind"
      ],
      "type": "object"
    })
}
