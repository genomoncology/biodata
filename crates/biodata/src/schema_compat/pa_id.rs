//! Published schema for `PaId`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::PaId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PaId".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": " A ClinGen canonical protein allele identifier.",
          "pattern": "^PA(?:18446744073709551615|[1-9][0-9]{0,18}|1[0-7][0-9]{18}|18[0-3][0-9]{17}|184[0-3][0-9]{16}|1844[0-5][0-9]{15}|18446[0-6][0-9]{14}|184467[0-3][0-9]{13}|1844674[0-3][0-9]{12}|184467440[0-6][0-9]{10}|1844674407[0-2][0-9]{9}|18446744073[0-6][0-9]{8}|1844674407370[0-8][0-9]{6}|18446744073709[0-4][0-9]{5}|184467440737095[0-4][0-9]{4}|1844674407370955[0-0][0-9]{3}|18446744073709551[0-5][0-9]{2}|184467440737095516[0-0][0-9]{1}|1844674407370955161[0-4][0-9]{0})$",
          "type": "string"
        })
    }
}
impl crate::PaId {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
