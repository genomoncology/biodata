//! Published schema for `AmpAscoCap2017TierCode`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::AmpAscoCap2017TierCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AmpAscoCap2017TierCode".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Checked AmpAscoCap2017Tier codes.",
          "enum": [
            "tier_i",
            "tier_ii",
            "tier_iii",
            "tier_iv"
          ],
          "type": "string"
        })
    }
}
