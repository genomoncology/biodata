# Check fixture provenance in the gate

Status: COMPLETE.
Landed: 058387508b07efdaeab04ff09546d3fbe7d97254

Milestone: 1

## Outcome

`make gate` fails when a data file under `crates/biodata/tests/` has no provenance record, when its hash differs from its record, or when its license falls outside the allowed list. Licensed or derived data then cannot enter this public repository unnoticed when the import moves fixtures in.

## Why

`AGENTS.md` requires every fixture to record its origin and license, and the licensed-data decision forbids licensed or derived data here. Nothing checks either yet.

## Proof

- Proof: outside-in tests run the check on temporary test trees and assert its exit status and message in these cases: a listed CC0 file passes; a synthetic file marked `synthetic` passes; an unlisted file fails and names it; a changed file fails on its hash; a file whose license is outside the list fails and names the license; a record that lists a missing file fails. `make gate` runs the check on the real tree and passes.

## Starts from

- Starts from: `crates/biodata/tests/` does not exist yet. The import brings in the second consumer's `tests/corpora`, `tests/fixtures` and `tests/cases/public`.
- The second consumer's corpora already use a `provenance.toml` per folder, in `tests/corpora/yen-2017/provenance.toml` and `tests/corpora/biocommons-hgvs/1.5.4/provenance.toml`. Each record has top-level `origin`, `license`, `license_basis` and `license_file` keys and one `[[file]]` table per file with `path`, `sha256`, `rows` and `transformation`. Adopt that format. Its public case files and `tests/fixtures/variant/common.json` have no record yet, and 0006 writes theirs.
- Data files are every file under `crates/biodata/tests/` except Rust sources, `provenance.toml` files and the license files they name.
- PM decision 2026-10-29: the allowed licenses are `synthetic`, `CC0-1.0`, `LicenseRef-public-domain`, `MIT`, `Apache-2.0`, `BSD-2-Clause` and `BSD-3-Clause`. A record marked `synthetic` needs no origin URL. Any other license needs the PM's ruling first, because `AGENTS.md` admits only licenses whose sole condition is keeping a notice.
- PM decision 2026-10-29: the check is a script in `scripts/` that `make gate` runs. It reads only the checkout and makes no network call.

## Keeps

- Keeps: the gate's existing format, lint and test steps.

## Defers

- Defers: records for the imported files go to 0006. The private-name check and the ISCN overlap check run outside this repository, because their inputs are private.

## Changes

- Changes: `scripts/`, `Makefile`
