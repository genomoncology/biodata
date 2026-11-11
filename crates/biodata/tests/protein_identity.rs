//! A protein identity covers one site, a range, or the whole protein.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail the test when a fixture does not build or parse"
)]
use biodata::hgvs::{self, Change, ProteinChange, ProteinRange, ProteinRangeError, ProteinSite};
use biodata::{Outcome, Position1, Residue};

fn site(residue: Residue, position: u32) -> ProteinSite {
    ProteinSite {
        residue,
        position: Position1::new(position).unwrap(),
    }
}

fn parsed(text: &str) -> ProteinChange {
    let Outcome::Parsed(value) = hgvs::parse(text).outcome else {
        panic!("{text} must parse");
    };
    let Change::Protein(change) = value.change else {
        panic!("{text} must be a protein change");
    };
    change
}

#[test]
fn a_constructed_range_identity_formats_as_the_standard_text_and_parses_back() {
    let range = ProteinRange::new(site(Residue::Ala, 2), site(Residue::Gly, 3)).unwrap();
    let same = ProteinChange::Same(Some(range));
    assert_eq!(same.to_string(), "Ala2_Gly3=", "three-letter text");
    assert_eq!(format!("{same:#}"), "A2_G3=", "one-letter text");
    assert_eq!(parsed("p.Ala2_Gly3="), same, "parse matches construction");
}

#[test]
fn a_range_must_end_after_it_starts() {
    for (first, last) in [(3, 2), (2, 2)] {
        assert_eq!(
            ProteinRange::new(site(Residue::Ala, first), site(Residue::Gly, last)),
            Err(ProteinRangeError),
            "{first}_{last}"
        );
    }
    assert_eq!(
        ProteinRangeError.code(),
        "hgvs.protein_range.order",
        "stable code"
    );
}

#[test]
fn one_site_is_written_without_a_last_site() {
    let same = ProteinChange::Same(Some(site(Residue::Leu, 858).into()));
    assert_eq!(same.to_string(), "Leu858=", "one-site text");
    assert_eq!(parsed("p.Leu858="), same, "parse matches construction");
}
