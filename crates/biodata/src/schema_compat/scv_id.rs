//! Published schema for `ScvId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ScvId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ScvId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": " A ClinVar submission accession.",
          "pattern": format!(r"^SCV(?!000000000)[0-9]{{9}}(\.{})?$", crate::schema_pattern::decimal(u32::MAX.into())),
          "type": "string"
        })
    }
}
impl crate::ScvId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
