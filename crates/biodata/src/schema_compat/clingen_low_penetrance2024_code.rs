//! Published schema for `ClingenLowPenetrance2024Code`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ClingenLowPenetrance2024Code {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ClingenLowPenetrance2024Code".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Checked ClingenLowPenetrance2024 codes.",
          "enum": [
            "pathogenic_low_penetrance",
            "likely_pathogenic_low_penetrance",
            "established_risk_allele",
            "likely_risk_allele",
            "uncertain_risk_allele"
          ],
          "type": "string"
        })
    }
}
