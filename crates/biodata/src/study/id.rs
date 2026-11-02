fn native(value: &str) -> bool {
    value
        .bytes()
        .next()
        .is_some_and(|b| b.is_ascii_alphanumeric())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
}
fn blob(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
}
fn import_time(value: &str) -> bool {
    let Some((date, time)) = value.split_once(' ') else {
        return false;
    };
    let parts: Vec<_> = time.split(':').collect();
    date.len() == 10
        && date.parse::<crate::PartialDate>().is_ok()
        && parts.len() == 3
        && parts.iter().zip([23, 59, 59]).all(|(v, max)| {
            v.len() == 2
                && v.bytes().all(|b| b.is_ascii_digit())
                && v.parse::<u8>().is_ok_and(|v| v <= max)
        })
}

crate::checked_id!(
    StudyId,
    StudyValueInvalid,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Opaque case-sensitive cBioPortal study identity, without repair."
);
crate::checked_id!(
    StudyProfileId,
    StudyValueInvalid,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Exact cBioPortal molecular profile identity."
);
crate::checked_id!(
    StudySampleListId,
    StudyValueInvalid,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Exact cBioPortal sample-list identity."
);
crate::checked_id!(
    CbioportalCancerTypeId,
    StudyValueInvalid,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Native cBioPortal cancer-type code, with no OncoTree equivalence."
);
crate::checked_id!(
    StudyResourceId,
    StudyValueInvalid,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Exact cBioPortal resource identity scoped to one study."
);
crate::checked_id!(
    StudyBlobSha,
    StudyValueInvalid,
    blob,
    "^[0-9a-f]{40}$",
    "GitHub repository blob revision, not a data-content checksum."
);
crate::checked_id!(
    StudyImportTime,
    StudyValueInvalid,
    import_time,
    "^(?!0000)[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01]) ([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$",
    "Native study import timestamp with unstated timezone; never an acquisition time. The parser additionally validates the calendar day, including leap years."
);
