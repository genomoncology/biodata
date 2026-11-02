//! Published schema for `Variant`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::Variant {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Variant".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator.subschema_for::<crate::Accession>();
        let _ = generator.subschema_for::<crate::Allele>();
        let _ = generator.subschema_for::<crate::Build>();
        let _ = generator.subschema_for::<crate::EnsemblGeneId>();
        let _ = generator.subschema_for::<crate::Expression>();
        let _ = generator.subschema_for::<crate::ExpressionSyntax>();
        let _ = generator.subschema_for::<crate::SourceGeneRef>();
        let _ = generator.subschema_for::<crate::HgncId>();
        let _ = generator.subschema_for::<crate::NcbiGeneId>();
        let _ = generator.subschema_for::<crate::Representation>();
        let _ = generator.subschema_for::<crate::Sequence>();
        let _ = generator.subschema_for::<crate::SequenceLocation>();
        let _ = generator.subschema_for::<crate::Span0>();
        let _ = generator.subschema_for::<crate::Text>();
        let _ = generator.subschema_for::<crate::VariantId>();
        schemars::json_schema!({
          "description": "Variant entity with source-stated identities, forms and genes; no disclosure policy.",
          "properties": {
            "genes": {
              "description": "Genes stated by the sources.",
              "items": {
                "$ref": "#/$defs/GeneRef"
              },
              "type": "array"
            },
            "ids": {
              "description": "All source identifiers, without inferred aliases.",
              "items": {
                "$ref": "#/$defs/VariantId"
              },
              "type": "array"
            },
            "representations": {
              "description": "Every source-qualified representation.",
              "items": {
                "$ref": "#/$defs/Representation"
              },
              "type": "array"
            }
          },
          "type": "object"
        })
    }
}
