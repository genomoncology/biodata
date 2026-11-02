//! Published schema for `AcmgAmp2015Code`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::AcmgAmp2015Code {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AcmgAmp2015Code".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Checked AcmgAmp2015 codes.",
          "enum": [
            "pathogenic",
            "likely_pathogenic",
            "uncertain_significance",
            "likely_benign",
            "benign"
          ],
          "type": "string"
        })
    }
}
