//! Published schema for `RcvId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::RcvId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RcvId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": " A ClinVar variant-and-condition aggregate accession.",
          "pattern": format!(r"^RCV(?!000000000)[0-9]{{9}}(\.{})?$", crate::schema_pattern::decimal(u32::MAX.into())),
          "type": "string"
        })
    }
}
impl crate::RcvId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
