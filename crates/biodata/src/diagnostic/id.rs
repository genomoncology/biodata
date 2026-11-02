fn digits(value: &str, width: usize) -> bool {
    value.len() == width && value.bytes().all(|b| b.is_ascii_digit())
}
fn gtr(value: &str) -> bool {
    value
        .strip_prefix("GTR")
        .and_then(|v| v.split_once('.'))
        .is_some_and(|(id, version)| {
            digits(id, 9)
                && !version.starts_with('0')
                && !version.is_empty()
                && version.bytes().all(|b| b.is_ascii_digit())
        })
}
fn who(value: &str) -> bool {
    let mut parts = value.split('-');
    parts.next().is_some_and(|v| digits(v, 4) || digits(v, 5))
        && parts.next().is_some_and(|v| digits(v, 3) || digits(v, 5))
        && parts.next().is_some_and(|v| digits(v, 2))
        && parts.next().is_none()
}
fn pma(value: &str) -> bool {
    value.strip_prefix('P').is_some_and(|v| digits(v, 6))
}
fn supplement(value: &str) -> bool {
    value
        .strip_prefix('S')
        .is_some_and(|v| digits(v, 3) && v != "000")
}
fn native_supplement(value: &str) -> bool {
    value.is_empty() || supplement(value)
}
fn drug(value: &str) -> bool {
    value
        .strip_prefix("NDA")
        .or_else(|| value.strip_prefix("BLA"))
        .is_some_and(|v| digits(v, 6))
}
fn device(value: &str) -> bool {
    if let Some((base, sub)) = value.split_once('/') {
        return pma(base) && supplement(sub);
    }
    pma(value)
        || value
            .strip_prefix('K')
            .or_else(|| value.strip_prefix("DEN"))
            .or_else(|| value.strip_prefix('H'))
            .is_some_and(|v| digits(v, 6))
}

crate::checked_id!(
    GtrTestId,
    DiagnosticIdInvalid,
    gtr,
    "^GTR[0-9]{9}\\.[1-9][0-9]*$",
    "Complete native GTR accession and positive version."
);
crate::checked_id!(
    WhoProductId,
    DiagnosticIdInvalid,
    who,
    "^[0-9]{4,5}-([0-9]{3}|[0-9]{5})-[0-9]{2}$",
    "WHO product listing identity, independent of product codes."
);
crate::checked_id!(
    PmaNumber,
    DiagnosticIdInvalid,
    pma,
    "^P[0-9]{6}$",
    "Native FDA PMA base application number."
);
crate::checked_id!(
    PmaSupplement,
    DiagnosticIdInvalid,
    supplement,
    "^S(00[1-9]|0[1-9][0-9]|[1-9][0-9]{2})$",
    "Explicit native PMA supplement selector; no invented base alias."
);
crate::checked_id!(
    FdaDrugApplication,
    DiagnosticIdInvalid,
    drug,
    "^(NDA|BLA)[0-9]{6}$",
    "Cell-owned drug application preserving native NDA/BLA prefix and leading zeros."
);
crate::checked_id!(
    DiagnosticDeviceId,
    DiagnosticIdInvalid,
    device,
    "^(P[0-9]{6}(/S(00[1-9]|0[1-9][0-9]|[1-9][0-9]{2}))?|K[0-9]{6}|DEN[0-9]{6}|H[0-9]{6})$",
    "Source device identifier; DEN and H identifiers imply no executable get."
);

crate::checked_id!(
    PmaSupplementCell,
    DiagnosticIdInvalid,
    native_supplement,
    "^($|S(00[1-9]|0[1-9][0-9]|[1-9][0-9]{2}))$",
    "Native PMA supplement cell; empty denotes base and never becomes S000."
);

fn recall(value: &str) -> bool {
    value
        .strip_prefix("Z-")
        .and_then(|v| v.split_once('-'))
        .is_some_and(|(number, year)| digits(number, 4) && digits(year, 4))
}
crate::checked_id!(
    DeviceRecallNumber,
    DiagnosticIdInvalid,
    recall,
    "^Z-[0-9]{4}-[0-9]{4}$",
    "Native FDA product recall number, independent of its event number."
);

fn drug_recall(value: &str) -> bool {
    value
        .strip_prefix("D-")
        .and_then(|v| v.split_once('-'))
        .is_some_and(|(number, year)| digits(number, 4) && digits(year, 4))
}
crate::checked_id!(
    DrugRecallNumber,
    DiagnosticIdInvalid,
    drug_recall,
    "^D-[0-9]{4}-[0-9]{4}$",
    "Native FDA drug recall number, independent of its event number."
);
