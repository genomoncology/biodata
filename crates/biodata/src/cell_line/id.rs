use crate::Curie;

crate::checked_id!(
    CellosaurusAccession,
    CellLineIdInvalid,
    |v: &str| v.strip_prefix("CVCL_").is_some_and(|s| s.len() == 4
        && s.bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())),
    "^CVCL_[A-Z0-9]{4}$",
    "A checked Cellosaurus accession; primary identity is verified by the adapter."
);
crate::checked_id!(
    CellLineSourceId,
    CellLineIdInvalid,
    |v: &str| Curie::qualified_parts(v).is_some(),
    format!(
        r"^[A-Za-z0-9_.-]+:[^{}\x00-\x1f\x7f-\x9f]+$",
        crate::schema_pattern::WHITESPACE
    ),
    "An exact database-qualified source ID, preserving the database and native accession independently."
);
impl CellLineSourceId {
    /// Source database spelling before the first colon.
    pub fn database(&self) -> &str {
        self.as_str().split_once(':').map_or("", |v| v.0)
    }
    /// Native accession, which can itself contain a colon.
    pub fn accession(&self) -> &str {
        self.as_str().split_once(':').map_or("", |v| v.1)
    }
}
crate::checked_id!(
    ChemblCellId,
    CellLineIdInvalid,
    |v: &str| v
        .strip_prefix("CHEMBL")
        .is_some_and(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())),
    "^CHEMBL[0-9]+$",
    "A ChEMBL source-local cell ID, never a compound or target identity."
);
