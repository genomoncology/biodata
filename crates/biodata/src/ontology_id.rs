//! Publisher ontology identifiers preserve their exact source spelling.
fn digits(value: &str, prefix: &str, width: Option<usize>) -> bool {
    value.strip_prefix(prefix).is_some_and(|digits| {
        !digits.is_empty()
            && digits.bytes().all(|v| v.is_ascii_digit())
            && width.is_none_or(|width| digits.len() == width)
    })
}
crate::checked_id!(
    MondoId,
    CurieInvalid,
    |value| digits(value, "MONDO:", Some(7)),
    "^MONDO:[0-9]{7}$",
    "A publisher MONDO identifier with seven digits."
);
crate::checked_id!(
    NcitId,
    CurieInvalid,
    |value| digits(value, "C", None),
    "^C[0-9]+$",
    "A native NCI Thesaurus concept code."
);
