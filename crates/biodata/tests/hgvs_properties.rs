//! Generated canonical HGVS descriptions preserve their spelling and value.
#![allow(
    clippy::indexing_slicing,
    reason = "the generated indices stay inside the four literal DNA bases"
)]
use biodata::{Outcome, hgvs};
use proptest::prelude::*;

#[expect(
    clippy::disallowed_methods,
    reason = "the test alone reads its case-count setting"
)]
fn cases() -> u32 {
    std::env::var("HGVS_PROPERTY_CASES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(32)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(cases()))]
    #[test]
    fn canonical_hgvs_parse_then_format_returns_the_input(
        position in 1u32..100_000,
        from_index in 0usize..4,
        to_offset in 1usize..4,
        accession in prop_oneof![Just("NC_000007.14"), Just("NM_004333.6")],
    ) {
        let bases = ['A', 'C', 'G', 'T'];
        let from = bases[from_index];
        let to = bases[(from_index + to_offset) % bases.len()];
        let molecule = if accession.starts_with("NC_") { 'g' } else { 'c' };
        let input = format!("{accession}:{molecule}.{position}{from}>{to}");
        let Outcome::Parsed(parsed) = hgvs::parse(&input).outcome else {
            prop_assert!(false, "generated canonical HGVS must parse: {input}");
            return Ok(());
        };
        prop_assert_eq!(parsed.to_string(), input);
    }
}

#[test]
fn canonical_hgvs_law_detects_a_broken_renderer() {
    let input = "NC_000007.14:g.1A>T";
    let Outcome::Parsed(parsed) = hgvs::parse(input).outcome else {
        panic!("canonical witness must parse");
    };
    let broken = parsed.to_string().replace("A>T", "A>G");
    assert_ne!(
        broken, input,
        "one changed allele is a minimal counterexample"
    );
}
