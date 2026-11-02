//! One API assertion for each case-table row.
#![cfg(feature = "serde")]
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "case assertions report invalid fixtures and wrong public API outcomes"
)]
use biodata::{Outcome, Parsed, Refusal};
use serde_json::Value;
use std::fmt::{Debug, Display};

pub(crate) fn check<T: PartialEq + Display + Debug + serde::Serialize>(
    row: &Value,
    parse: fn(&str) -> Parsed<T>,
    at: &str,
) {
    let input = row["input"].as_str().unwrap();
    let parsed = parse(input);
    assert_eq!(&*parsed.source, input, "{at}");
    let json = serde_json::to_value(&parsed).unwrap();
    assert_eq!(json["source"], input, "{at}");
    for key in ["outcome", "code", "span", "value"] {
        if key != "value" || row.get(key).is_some() {
            assert_eq!(json.get(key), row.get(key), "{key} at {at}");
        }
    }
    let (outcome, refusal) = match &parsed.outcome {
        Outcome::Parsed(value) => {
            let render = value.to_string();
            assert_eq!(render, row["render"].as_str().unwrap_or(input), "{at}");
            match parse(&render).outcome {
                Outcome::Parsed(again) => assert_eq!(&again, value, "{at}"),
                other => panic!("{at}: the render does not parse: {other:?}"),
            }
            ("parsed", None)
        }
        Outcome::Unsupported(refusal) => ("unsupported", Some(refusal)),
        Outcome::Invalid(refusal) => ("invalid", Some(refusal)),
        Outcome::LimitExceeded(refusal) => ("limit_exceeded", Some(refusal)),
    };
    assert_eq!(outcome, row["outcome"], "{at}");
    let code = refusal.map(|r: &Refusal| r.code.name());
    assert_eq!(code, row["code"].as_str(), "{at}");
    let span = refusal.and_then(|r| r.span).map(|s| [s.start, s.end]);
    let expected_span = row["span"]
        .as_array()
        .map(|s| [0, 1].map(|i| s[i].as_u64().unwrap() as u32));
    assert_eq!(span, expected_span, "{at}");
}
