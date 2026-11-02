//! Published schema for `ClingenCgcVicc2022Code`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ClingenCgcVicc2022Code {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClingenCgcVicc2022Code".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Checked ClingenCgcVicc2022 codes.",
          "enum": [
            "oncogenic",
            "likely_oncogenic",
            "uncertain_significance",
            "likely_benign",
            "benign"
          ],
          "type": "string"
        })
    }
}
