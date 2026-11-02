//! Published schema for `OmimId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::OmimId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OmimId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": " A six-digit OMIM identifier, preserving all six digits.",
          "pattern": format!("^[1-{}][0-9]{{{}}}$", crate::identifier::OMIM_MAX / crate::identifier::OMIM_MIN, crate::identifier::OMIM_MIN.ilog10()),
          "type": "string"
        })
    }
}
impl crate::OmimId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
