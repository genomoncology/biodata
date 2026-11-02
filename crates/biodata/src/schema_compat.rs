//! Preserve schemas while shared values retain their compact storage.
mod accession;
mod accession_base;
mod acmg_amp2015_code;
mod allele;
mod amino_acids;
mod amp_asco_cap2017_level_code;
mod amp_asco_cap2017_tier_code;
mod base;
mod bound;
mod build;
mod ca_id;
mod chromosome;
mod civic_evidence_level_code;
mod clin_var_allele_id;
mod clin_var_variation_id;
mod clingen_cgc_vicc2022_code;
mod clingen_low_penetrance2024_code;
mod contig;
mod cosmic_id;
mod expression;
mod expression_syntax;
mod finite_number;
mod genomic_interval;
mod hgnc_id;
mod hgvs_text;
mod mane;
mod mane_status;
mod ncbi_gene_id;
mod omim_id;
mod oncokb_level_code;
mod oncokb_oncogenicity_code;
mod pa_id;
mod partial_date;
mod pmid;
mod position1;
mod rcv_id;
mod representation;
mod rs_id;
mod scv_id;
mod sequence;
mod span0;
mod strand;
mod text;
mod transcript_reference;
mod uncertain_span0;
mod uni_prot_feature;
mod uni_prot_isoform;
mod variant;
mod variant_id;
mod vcv_id;
mod vrs_id;

/// Preserve documented unit choices as one string enum instead of a union.
pub(crate) fn closed_string_enum(schema: &mut schemars::Schema) {
    let Some(branches) = schema.get("oneOf").and_then(|value| value.as_array()) else {
        return;
    };
    if branches.is_empty()
        || !branches.iter().all(|branch| {
            branch.get("type").is_some_and(|value| value == "string")
                && branch.get("const").is_some_and(|value| value.is_string())
                && branch
                    .get("description")
                    .is_some_and(|value| value.is_string())
                && branch.as_object().is_some_and(|object| {
                    object
                        .keys()
                        .all(|key| matches!(key.as_str(), "type" | "const" | "description"))
                })
        })
    {
        return;
    }
    let values: Vec<_> = branches
        .iter()
        .filter_map(|branch| branch.get("const").cloned())
        .collect();
    let descriptions: Vec<_> = branches
        .iter()
        .filter_map(|branch| branch.get("description").cloned())
        .collect();
    let replacement = schemars::json_schema!({
        "type": "string", "enum": values, "x-enum-descriptions": descriptions
    });
    schema.remove("oneOf");
    if let Some(fields) = replacement.as_object() {
        for (key, value) in fields {
            schema.insert(key.clone(), value.clone());
        }
    }
}
