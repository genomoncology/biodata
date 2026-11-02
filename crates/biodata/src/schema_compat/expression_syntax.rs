//! Published schema for `ExpressionSyntax`.
use schemars::{JsonSchema, Schema, SchemaGenerator};
impl JsonSchema for crate::ExpressionSyntax {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ExpressionSyntax".into()
    }
    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let _ = generator;
        schemars::json_schema!({
          "description": "Published syntax label for a source-preserved expression.",
          "enum": [
            "hgvs.g",
            "hgvs.c",
            "hgvs.n",
            "hgvs.r",
            "hgvs.p",
            "hgvs.m",
            "hgvs.o",
            "spdi",
            "gnomad",
            "vcf",
            "iscn"
          ],
          "type": "string",
          "x-enum-descriptions": [
            " HGVS genomic description.",
            " HGVS coding description.",
            " HGVS noncoding transcript description.",
            " HGVS RNA description.",
            " HGVS protein description.",
            " HGVS mitochondrial description.",
            " HGVS circular genomic description.",
            " NCBI SPDI interbase description.",
            " gnomAD chromosome-position-reference-alternate identifier.",
            " Padded VCF chromosome-position-reference-alternate text.",
            " Source-preserved cytogenetic notation."
          ],
          "x-enum-open": true
        })
    }
}
