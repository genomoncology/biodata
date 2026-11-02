//! Static literal construction preserves the checked runtime text contract.
use biodata::{Text, TextLiteral};

#[test]
fn static_literal_validation_matches_runtime_boundaries() {
    for value in [
        "",
        " ",
        "\n",
        "\nready",
        "ready\n",
        "safe\u{85}",
        "ready\u{200e}",
        " \u{3000}ready\u{2029} ",
        "\u{061c}",
        "ready\u{a0}",
        "a\u{200b}b",
        "e\u{301}",
        "🧬",
    ] {
        assert_eq!(
            TextLiteral::new(value).map(Text::from),
            Text::new(value),
            "literal and runtime validation agree for {value:?}"
        );
    }
    for ch in [
        '\u{9}', '\u{a}', '\u{b}', '\u{c}', '\u{d}', ' ', '\u{85}', '\u{a0}', '\u{1680}',
        '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
        '\u{2007}', '\u{2008}', '\u{2009}', '\u{200a}', '\u{2028}', '\u{2029}', '\u{202f}',
        '\u{205f}', '\u{3000}',
    ] {
        let value = Box::leak(format!("{ch}ready{ch}").into_boxed_str());
        assert_eq!(
            TextLiteral::new(value).map(Text::from),
            Text::new(value),
            "the Unicode trim boundary agrees for {ch:?}"
        );
    }
    for length in [Text::MAX_CHARS, Text::MAX_CHARS + 1] {
        let value = Box::leak("é".repeat(length).into_boxed_str());
        assert_eq!(
            TextLiteral::new(value).map(Text::from),
            Text::new(value),
            "length counts Unicode scalars"
        );
    }
    assert_eq!(
        biodata::text!("  ready  ").as_str(),
        "ready",
        "a valid literal constructs canonical Text"
    );
}
