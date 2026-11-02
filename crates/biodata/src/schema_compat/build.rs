//! Published schema for `Build`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Build {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Build".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "The assembly stated by the source; no default is inferred.",
          "enum": [
            "GRCh37",
            "GRCh38"
          ],
          "type": "string",
          "x-enum-descriptions": [
            " Genome Reference Consortium human build 37.",
            " Genome Reference Consortium human build 38."
          ],
          "x-enum-open": true
        })
    }
}
