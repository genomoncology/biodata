//! The held HGNC complete set checks the public gene-symbol grammar.
#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    clippy::print_stderr,
    reason = "this external corpus test reads a held publisher file outside the biodata library"
)]
use biodata::GeneSymbol;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn every_approved_hgnc_symbol_parses() {
    let Some(path) = std::env::var_os("HGNC_COMPLETE_SET") else {
        eprintln!("HGNC_COMPLETE_SET absent; publisher corpus check skipped");
        return;
    };
    let file = File::open(&path).expect("open held HGNC complete set");
    let mut rows = BufReader::new(file).lines();
    let header = rows.next().expect("HGNC header").expect("read HGNC header");
    let columns: Vec<_> = header.split('\t').collect();
    let symbol_column = columns
        .iter()
        .position(|name| *name == "symbol")
        .expect("symbol column");
    let status_column = columns
        .iter()
        .position(|name| *name == "status")
        .expect("status column");
    let mut approved = 0usize;
    let mut rejected = 0usize;
    let mut rejected_characters = BTreeSet::new();
    for row in rows {
        let row = row.expect("read HGNC row");
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), columns.len(), "HGNC column count");
        if cells[status_column] != "Approved" {
            continue;
        }
        approved += 1;
        let symbol = cells[symbol_column];
        if symbol.parse::<GeneSymbol>().is_err() {
            rejected += 1;
            rejected_characters.extend(symbol.chars().filter(|ch| !ch.is_ascii_alphanumeric()));
        }
    }
    assert!(approved > 0, "HGNC file has no Approved rows");
    assert_eq!(
        rejected, 0,
        "{approved} approved symbols checked; {rejected} rejected; rejected non-alphanumeric characters: {rejected_characters:?}"
    );
    eprintln!("{approved} approved HGNC symbols passed");
}
