//! Published schema for `Accession`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Accession {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Accession".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A reference sequence accession: RefSeq or Ensembl with its version, or LRG.",
          "pattern": "^(?:(?:N[CGMPRTW]|X[MPR])_(?:[0-9]{6}|(?!000)[0-9]{9})\\.(?:65535|[1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]{1}|6553[0-4][0-9]{0})|ENS[GTP][0-9]{11}\\.(?:65535|[1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]{1}|6553[0-4][0-9]{0})|LRG_0*(?:0|(?:4294967295|[1-9][0-9]{0,8}|[1-3][0-9]{9}|4[0-1][0-9]{8}|42[0-8][0-9]{7}|429[0-3][0-9]{6}|4294[0-8][0-9]{5}|42949[0-5][0-9]{4}|429496[0-6][0-9]{3}|4294967[0-1][0-9]{2}|42949672[0-8][0-9]{1}|429496729[0-4][0-9]{0}))(?:[tp]0*(?:0|(?:65535|[1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]{1}|6553[0-4][0-9]{0})))?)$",
          "type": "string"
        })
    }
}
impl crate::Accession {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
