//! Published schema for `Base`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Base {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Base".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "One nucleotide: the four DNA bases and the IUPAC ambiguity codes. It renders as its upper-case letter.",
          "pattern": "^[ACGTNRYSWKMBDHV]$",
          "type": "string"
        })
    }
}
impl crate::Base {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
