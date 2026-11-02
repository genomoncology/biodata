//! Published schema for `PartialDate`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::PartialDate {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PartialDate".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "A Gregorian date preserving year, month or day precision. The parser additionally validates the calendar day, including leap years.",
          "pattern": "^(?!0000)[0-9]{4}(-(0[1-9]|1[0-2])(-(0[1-9]|[12][0-9]|3[01]))?)?$",
          "type": "string"
        })
    }
}
impl crate::PartialDate {
    /// Accepted spelling in this value's published schema.
    pub fn pattern() -> String {
        <Self as JsonSchema>::json_schema(&mut SchemaGenerator::default())
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_owned()
    }
}
