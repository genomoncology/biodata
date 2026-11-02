//! Byte payloads retain source, refusal spans and independent parse outcomes.
#![cfg(feature = "serde")]
#![allow(
    clippy::unwrap_used,
    reason = "serialization failures fail the public API contract test"
)]

use biodata::{MAX_LEN, hgvs, parse_bytes};
use serde_json::{Value, json};

#[test]
fn byte_payload_boundaries_have_named_refusals() {
    for (input, outcome, code, span) in [
        (b"g.1A>T".to_vec(), "parsed", None, Value::Null),
        (
            b"NC_\xff:g.1A>T".to_vec(),
            "invalid",
            Some("input.utf8.invalid"),
            json!([3, 4]),
        ),
        (
            b"g.1A>T\xe2\x82".to_vec(),
            "invalid",
            Some("input.utf8.invalid"),
            json!([6, 8]),
        ),
        (
            vec![b'A'; MAX_LEN + 1],
            "limit_exceeded",
            Some("input.length.limit"),
            Value::Null,
        ),
        (
            vec![0xff; MAX_LEN + 1],
            "limit_exceeded",
            Some("input.length.limit"),
            Value::Null,
        ),
        (
            b"g.1A>T\r".to_vec(),
            "invalid",
            Some("input.trailing.invalid"),
            json!([6, 7]),
        ),
        (
            b"g.1A>T\n".to_vec(),
            "invalid",
            Some("input.trailing.invalid"),
            json!([6, 7]),
        ),
        (b"g.1A>T".to_vec(), "parsed", None, Value::Null),
    ] {
        let parsed = parse_bytes(&input, hgvs::parse);
        let value = serde_json::to_value(parsed).unwrap();
        assert_eq!(value["source"], String::from_utf8_lossy(&input).as_ref());
        assert_eq!(value["outcome"], outcome);
        assert_eq!(value["code"], json!(code));
        assert_eq!(value["span"], span);
    }
}

#[test]
fn payload_length_excludes_caller_owned_line_terminators() {
    let text = format!("g.1_2ins{}", "A".repeat(MAX_LEN - 8));
    for (payload, expected) in [
        (text.clone(), "parsed"),
        (format!("{text}A"), "limit_exceeded"),
        ("g.1A>T".into(), "parsed"),
    ] {
        let parsed = parse_bytes(payload.as_bytes(), hgvs::parse);
        let value = serde_json::to_value(parsed).unwrap();
        assert_eq!(value["source"], payload);
        assert_eq!(value["outcome"], expected);
        if expected == "limit_exceeded" {
            assert_eq!(value["code"], "input.length.limit");
        }
    }
}
