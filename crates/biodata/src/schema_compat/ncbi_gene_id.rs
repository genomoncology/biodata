//! Published schema for `NcbiGeneId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::NcbiGeneId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NcbiGeneId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A positive NCBI Gene identifier in canonical decimal text.",
          "pattern": "^(?:4294967295|[1-9][0-9]{0,8}|[1-3][0-9]{9}|4[0-1][0-9]{8}|42[0-8][0-9]{7}|429[0-3][0-9]{6}|4294[0-8][0-9]{5}|42949[0-5][0-9]{4}|429496[0-6][0-9]{3}|4294967[0-1][0-9]{2}|42949672[0-8][0-9]{1}|429496729[0-4][0-9]{0})$",
          "type": "string"
        })
    }
}
impl crate::NcbiGeneId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
