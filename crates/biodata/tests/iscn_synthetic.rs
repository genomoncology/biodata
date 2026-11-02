//! Cases composed from the public grammar without adapting a source example.
use biodata::{Outcome, iscn};

#[test]
fn synthetic_clone_preserves_count_events_and_cells() {
    let source = "49,XY,+8,+14,+18[113]";
    let Outcome::Parsed(value) = iscn::parse(source).outcome else {
        panic!("the grammar-composed clone parses");
    };
    assert_eq!(
        value.to_string(),
        source,
        "canonical rendering preserves the stated clone"
    );
    assert_eq!(value.clones.len(), 1, "the source states one clone");
    let encoded = serde_json::to_value(&value).unwrap();
    assert_eq!(
        encoded["clones"].as_array().unwrap().len(),
        1,
        "the wire preserves the clone count"
    );
    assert_eq!(
        encoded["clones"][0]["events"].as_array().unwrap().len(),
        3,
        "the wire preserves the three written events"
    );
}

#[test]
fn synthetic_clone_refuses_missing_sex_and_trailing_text() {
    for source in ["49,,+8,+14,+18[113]", "49,XY,+8,+14,+18[113] trailing"] {
        assert!(
            !matches!(iscn::parse(source).outcome, Outcome::Parsed(_)),
            "the whole clone must follow the grammar"
        );
    }
}
