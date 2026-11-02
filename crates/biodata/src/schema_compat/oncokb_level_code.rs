//! Published schema for `OncokbLevelCode`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::OncokbLevelCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OncokbLevelCode".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Source-defined OncoKB vocabulary.",
          "enum": [
            "level_1",
            "level_2",
            "level_3a",
            "level_3b",
            "level_4",
            "level_r1",
            "level_r2",
            "level_dx1",
            "level_dx2",
            "level_dx3",
            "level_px1",
            "level_px2",
            "level_px3"
          ],
          "type": "string"
        })
    }
}
