//! Published schema for `AmpAscoCap2017LevelCode`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::AmpAscoCap2017LevelCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AmpAscoCap2017LevelCode".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Checked AmpAscoCap2017Level codes.",
          "enum": [
            "level_a",
            "level_b",
            "level_c",
            "level_d"
          ],
          "type": "string"
        })
    }
}
