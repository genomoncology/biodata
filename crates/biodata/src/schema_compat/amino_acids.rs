//! Published schema for `AminoAcids`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::AminoAcids {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AminoAcids".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Nonempty uppercase standard residues plus U, O and X.",
          "pattern": "^[ACDEFGHIKLMNPQRSTVWYUOX]+$",
          "type": "string"
        })
    }
}
impl crate::AminoAcids {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
