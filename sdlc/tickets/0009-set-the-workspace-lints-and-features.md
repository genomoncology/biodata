# Set the workspace lints and features before the import

Status: COMPLETE.
Landed: 98a880763b1b23195d848ce084aa85980ce4e020

Milestone: 1

## Outcome

The workspace enforces the lint set that 0006's lint decision names, and `crates/biodata` declares the `serde` and `schema` features, so the import lands into the gate it must pass.

## Why

The import then only moves code, and each lint failure shows up in the file that causes it.

## Proof

- Proof: `make gate` passes through `pm job`. The developer's report shows `cargo clippy` refusing a scratch file that breaks each lint this ticket adds, and `cargo check` failing on a scratch `schemars` derive built without the `schema` feature. The scratch files are not committed.

## Starts from

- Starts from: the root `Cargo.toml`, which today sets only four Rust lints, all at `deny`, and `crates/biodata/Cargo.toml`, which declares no features.
- The Lints section of `sdlc/designs/2026-10-28-reconciliation.md` and 0006's lint decision give the rule: the union of both consumers' sets, at the stricter level of each.
- Rust lints: `unsafe_code`, `unreachable_pub` and `missing_debug_implementations` at `forbid`, as the second consumer has them, and `missing_docs` at `deny`.
- Clippy lints: BioMCP's workspace `Cargo.toml` table is the superset. It denies `allow_attributes`, `allow_attributes_without_reason`, `wildcard_imports`, `missing_assert_message`, `fn_params_excessive_bools`, `ref_option`, `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout`, `print_stderr`, `too_many_lines`, `disallowed_methods` and `disallowed_types`. The second consumer's table is a subset of it. The PM mails the coordinator both repositories' locations.
- `clippy.toml` copies BioMCP's `crates/biodata/clippy.toml`: the line threshold of 80, the four in-test allowances, and its disallowed types and methods. The second consumer's `clippy.toml` holds only the four in-test allowances, which BioMCP's file already has. Copy the lists as they stand, reasons included. Drop the comment that names BioMCP's own base file and ratchet.
- Features: `serde` is off by default and turns on the `serde` dependency. `schema` turns on `serde` and the `schemars` dependency. Take each dependency's version and feature set from BioMCP's workspace `Cargo.toml` and lockfile.
- `wildcard_imports` is allowed in tests and in modules named `prelude`, as `skills/rust.md` says. A module with child modules only declares and re-exports them, and the reviewer checks this.

## Keeps

- Keeps: no code beyond the crate's one doc line. Version, edition and toolchain stay as they are.

## Defers

- Defers: the five `#[expect(clippy::too_many_lines, reason = ...)]` exceptions arrive with their functions in 0006.

## Changes

- Changes: `Cargo.toml`, `Cargo.lock`, `clippy.toml`, `crates/biodata/Cargo.toml`
