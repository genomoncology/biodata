//! Published schema for `OncokbOncogenicityCode`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::OncokbOncogenicityCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OncokbOncogenicityCode".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Source-defined OncoKB vocabulary.",
          "enum": [
            "oncogenic",
            "likely_oncogenic",
            "likely_neutral",
            "inconclusive",
            "resistance",
            "unknown"
          ],
          "type": "string"
        })
    }
}
