# 0020 — release-suite-parses-the-civic-monthly-release

Status: WITHDRAWN. Ian set BioData idle on 2026-11-04; work in progress kept on the ticket branches for a later restart

Milestone: 2

## Outcome

`make release-suite` reads a pinned CIViC monthly release from the data root and fails unless every variant, molecular profile, disease, therapy and citation in its TSV files parses into a BioData 1.0 value or is refused with a named reason a reviewed row accepts. It prints per-file counts of parsed, refused and reviewed rows, and per-gene variant counts that match the TSV.

## Why

The plan's replication set checks every 1.0 value against real public data on each release, and CIViC is the CC0 source that covers the most of them.

## Proof

- Proof: one `pm job` runs `make fetch-corpora` and `make release-suite` against the pinned release and passes. The same job with one reviewed row removed fails and names that row. A second `make fetch-corpora` downloads nothing and matches the recorded checksums. `make gate` still passes and never reads the data root.

## Starts from

- Starts from: `crates/biodata/tests/public_corpora.rs`, `tests/corpora/reviewed-differences.tsv` and `tests/corpora/provenance.toml`, which already run the small corpora inside the gate.
- Source: https://civicdb.org nightly and monthly TSV downloads, CC0. Pin the latest monthly release by date and SHA-256.
- PM decision 2026-11-04: release inputs never enter Git. `make fetch-corpora` downloads them into `$(BIODATA_CORPUS_ROOT)`, default `$(HOME)/workspace/data/biodata-corpora`, and writes a provenance README per source with its URL, release, date fetched, license and checksums. `make release-suite` makes no network call.
- PM decision 2026-11-04: the release suite runs before each BioData release and outside `make gate`. Reviewed refusals live in a tracked file with the existing review columns. CIViC is CC0, so reviewed rows may quote it with its notice kept.

## Keeps

- Keeps: `make gate` and its run time unchanged. No parser change ships in this ticket; a wrong value found here becomes its own ticket.

## Defers

- Defers: ClinVar to 0021, which reuses this target.

## Changes

- Changes: `Makefile`, `crates/biodata/tests/`, `scripts/`
