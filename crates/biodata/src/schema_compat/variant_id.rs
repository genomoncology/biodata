//! Published schema for `VariantId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::VariantId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "VariantId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Published variant-level identifiers, with each identifier's level retained.",
          "pattern": format!("({}|{}|{}|{}|{}|{}|{})", crate::RsId::pattern(), crate::CaId::pattern(), crate::PaId::pattern(), crate::VcvId::pattern(), crate::ClinVarVariationId::pattern(), crate::CosmicId::pattern(), crate::VrsId::pattern()),
          "type": "string"
        })
    }
}
impl crate::VariantId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
