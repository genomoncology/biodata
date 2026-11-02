//! Protein database accessions.
fn interpro(value: &str) -> bool {
    value
        .strip_prefix("IPR")
        .is_some_and(|v| v.len() == 6 && v.bytes().all(|b| b.is_ascii_digit()))
}
fn complex(value: &str) -> bool {
    value.strip_prefix("CPX-").is_some_and(|v| {
        !v.is_empty() && !v.starts_with('0') && v.bytes().all(|b| b.is_ascii_digit())
    })
}

crate::checked_id!(
    InterProId,
    InterProIdInvalid,
    interpro,
    "^IPR[0-9]{6}$",
    "An integrated InterPro entry accession, distinct from a member signature."
);

crate::checked_id!(
    ComplexPortalId,
    ComplexPortalIdInvalid,
    complex,
    "^CPX-[1-9][0-9]*$",
    "An exact Complex Portal accession."
);
