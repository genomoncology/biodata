//! Pattern fragments for checked decimal storage bounds.
/// Positive canonical decimal text from one through `maximum`.
pub fn decimal(maximum: u64) -> String {
    let digits = maximum.to_string();
    let mut alternatives = vec![digits.clone()];
    if digits.len() > 1 {
        alternatives.push(format!("[1-9][0-9]{{0,{}}}", digits.len() - 2));
    }
    for (index, digit) in digits.bytes().enumerate() {
        let first = if index == 0 { b'1' } else { b'0' };
        if digit > first {
            alternatives.push(format!(
                "{}[{}-{}][0-9]{{{}}}",
                &digits[..index],
                char::from(first),
                char::from(digit - 1),
                digits.len() - index - 1
            ));
        }
    }
    format!("(?:{})", alternatives.join("|"))
}

/// Canonical positive decimal text with a minimum display width.
pub fn padded_decimal(maximum: u64, width: usize) -> String {
    let decimal = decimal(maximum);
    if width == 0 {
        return decimal;
    }
    format!(
        "(?=[0-9]{{{width},}}(?:[^0-9]|$))(?:(?=[0-9]{{{width}}}(?:[^0-9]|$))0*{decimal}|{decimal})"
    )
}

/// Decimal text that permits zero and any source-supplied leading zeros.
pub fn source_decimal(maximum: u64) -> String {
    format!("0*(?:0|{})", decimal(maximum))
}

/// Rust's Unicode White_Space alphabet, shared with trim and is_whitespace.
pub const WHITESPACE: &str =
    r"\x09-\x0d\x20\x85\xa0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000";
