# Reconcile the two BioData crates

Status: COMPLETE.
Landed: de4ef81bc35f148c1f404a626332889bb22f1486

Milestone: 1

## Outcome

`sdlc/designs/0001-reconciliation.md` gives every public item of BioMCP's and the second consumer's `biodata` crates one merge rule, so the extraction can merge them mechanically while BioMCP's replies stay byte-identical, its `biodata.*` schemas change only by optional additions the record lists, and the second consumer's hot-path values stay unchanged.

## Why

The merge must be exact and quick when BioMCP's PM names its extraction commit, so the conflicts get found and ruled on now.

## Proof

- Proof: each row of the record names the item and its file in both crates at the commits the record states. A short script in `sdlc/scripts/` lists every `pub` item in each crate's source and fails when an item has no row. BioMCP's PM and the second consumer's PM each confirm the record by mail.

## Starts from

- Starts from: BioMCP's crate is `crates/biodata` in BioMCP's development repository, at 8,627 lines in 83 files. The second consumer's crate is `crates/biodata` in its own repository, at 11,542 lines in 57 files. The PM mails the coordinator both locations. Read both at current main and record the commits.
- Both crates use Rust 1.95.0, edition 2024 and winnow 1.0.4. BioMCP always derives serde and puts schemars behind a `schema` feature. The second consumer puts serde behind a `serde` feature and tests with proptest. Their workspace lint sets differ, and BioMCP's adds a clippy deny list.
- The crates overlap in a narrow band: `Accession`, `Chromosome`, `Base`, the variant model, notation and statement modules, the article ID, source text and population frequency. BioMCP alone holds the entity values (trial, drug, gene, disease, pathway, protein, adverse event and others). The second consumer alone holds full HGVS, cytogenetic and alteration notation, coordinates, transcripts and VCF values.
- BioMCP's published schemas live in `crates/biomcp-types/schemas/`, and its compatibility gate is `crates/biomcp-types/tests/schemas/compatibility.rs`. 91 files in `biomcp-types` use the crate.
- The second consumer's hot-path values are `Accession` and `Base` (`value.rs`), `Position` (`hgvs/nucleotide.rs`), `Chromosome` and `parse_chromosome` (`reference.rs`), `AlleleKey` (`coordinate.rs`), `Span` (`outcome.rs`) and the HGVS parser (`hgvs/`).
- Known conflicts to rule on: the variant ID spelling (BioMCP chose published-text IDs, and the second consumer needs no change), the Ensembl gene ID (BioMCP keeps an optional version, and the second consumer uses an unversioned ID), the population frequency fields, the coding systems, and the tags on the HGVS change enums.
- BioMCP lands more fixes to its crate before its signal, so the record states the commits it read and gets refreshed at the signal.

## Decisions

- PM decision 2026-10-28: the second consumer's hot-path values are the base for layout, parse behavior and cost.
- PM decision 2026-10-29: BioMCP's 1.0 wire is the baseline, and BioMCP adds no wrapper types. Where the second consumer's shape is wider, BioData's type writes BioMCP's wire and carries the wider fields and cases as optional additions, omitted when absent. The record lists each addition by schema.
- PM decision 2026-10-28: a value with one meaning in both crates becomes one type. Values whose meanings differ stay apart under distinct names.
- PM decision 2026-10-28: the record lists every change to a hot-path value, with the measurement the second consumer runs. The target is no change.
- PM decision 2026-10-28: the record ends with a proposal for each architecture decision in `sdlc/plan.md`, each with options and a recommendation, for the PM to rule on.

## Keeps

- Keeps: both consumers' repositories stay untouched. This ticket writes a record and a check script only.

## Defers

- Defers: the import and the merge wait for BioMCP's extraction commit and go to the next ticket.

## Changes

- Changes: `sdlc/designs/`, `sdlc/scripts/`
