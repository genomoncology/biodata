//! The HGVS and alteration values keep their size and alignment.
//!
//! The second consumer parses these values on its hot path. A change to a
//! variant's payload must not grow any of them. The numbers were measured on
//! a 64-bit target before range identity became a value.
#![cfg(target_pointer_width = "64")]
use biodata::alteration::{Alteration, Change as AlterationChange};
use biodata::hgvs::{Change, Hgvs, ProteinChange, ProteinRange, ProteinSite};
use std::mem::{align_of, size_of};

#[test]
fn hgvs_values_keep_their_size_and_alignment() {
    let measured = [
        ("ProteinSite", size_of::<ProteinSite>(), align_of::<ProteinSite>()),
        ("ProteinRange", size_of::<ProteinRange>(), align_of::<ProteinRange>()),
        ("ProteinChange", size_of::<ProteinChange>(), align_of::<ProteinChange>()),
        ("hgvs::Change", size_of::<Change>(), align_of::<Change>()),
        ("Hgvs", size_of::<Hgvs>(), align_of::<Hgvs>()),
        (
            "alteration::Change",
            size_of::<AlterationChange>(),
            align_of::<AlterationChange>(),
        ),
        ("Alteration", size_of::<Alteration>(), align_of::<Alteration>()),
    ];
    let expected = [
        ("ProteinSite", 8, 4),
        ("ProteinRange", 16, 4),
        ("ProteinChange", 40, 8),
        ("hgvs::Change", 0, 8),
        ("Hgvs", 0, 8),
        ("alteration::Change", 0, 8),
        ("Alteration", 0, 8),
    ];
    assert_eq!(measured, expected, "measured {measured:?}");
}
