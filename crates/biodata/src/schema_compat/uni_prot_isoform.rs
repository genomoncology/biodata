//! Published schema for `UniProtIsoform`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::UniProtIsoform {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UniProtIsoform".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A stated UniProt accession with an optional positive isoform number; never imputes -1.",
          "pattern": "^([OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9]([A-Z][A-Z0-9]{2}[0-9]){1,2})(-[1-9][0-9]*)?$",
          "type": "string"
        })
    }
}
impl crate::UniProtIsoform {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
