crate::checked_id!(
    UniProtAcc,
    UniProtAccInvalid,
    uniprot,
    "^([OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9]([A-Z][A-Z0-9]{2}[0-9]){1,2})$",
    " A whole UniProt base accession, without an isoform suffix."
);
crate::checked_id!(
    Cytoband,
    CytobandInvalid,
    cytoband,
    "^([1-9]|1[0-9]|2[0-2]|X|Y)[pq][1-9][0-9](\\.[0-9]+)?$",
    " A single nuclear cytogenetic band, without a range or location descriptor."
);
fn uniprot(value: &str) -> bool {
    let bytes = value.as_bytes();
    let Some((&first, rest)) = bytes.split_first() else {
        return false;
    };
    if !rest.first().is_some_and(u8::is_ascii_digit) {
        return false;
    }
    if matches!(first, b'O' | b'P' | b'Q') {
        return bytes.len() == 6
            && bytes.get(2..5).is_some_and(|part| {
                part.iter().all(u8::is_ascii_alphanumeric)
                    && part.iter().all(|ch| !ch.is_ascii_lowercase())
            })
            && bytes.last().is_some_and(u8::is_ascii_digit);
    }
    first.is_ascii_uppercase()
        && !matches!(first, b'O' | b'P' | b'Q')
        && matches!(bytes.len(), 6 | 10)
        && bytes.get(2..).is_some_and(|tail| {
            tail.chunks_exact(4).all(|part| match part {
                [a, b, c, d] => {
                    a.is_ascii_uppercase()
                        && [b, c]
                            .iter()
                            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
                        && d.is_ascii_digit()
                }
                _ => false,
            })
        })
}
fn cytoband(value: &str) -> bool {
    let Some((chromosome, band)) = value.split_once(['p', 'q']) else {
        return false;
    };
    let chromosome_ok = matches!(chromosome, "X" | "Y")
        || chromosome
            .parse::<u8>()
            .is_ok_and(|number| (1..=22).contains(&number) && number.to_string() == chromosome);
    let (major, minor) = band
        .split_once('.')
        .map_or((band, None), |(major, minor)| (major, Some(minor)));
    chromosome_ok
        && major.len() == 2
        && !major.starts_with('0')
        && major.bytes().all(|ch| ch.is_ascii_digit())
        && minor
            .is_none_or(|minor| !minor.is_empty() && minor.bytes().all(|ch| ch.is_ascii_digit()))
}
