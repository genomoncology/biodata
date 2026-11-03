# 0017 — gate-fails-on-unused-dependencies

Status: COMPLETE.
Landed: 62ffcafd18a08b897aa0a1a600754b8bdbb096ba

Milestone: 1

## Outcome

`make gate` fails when any crate in the workspace declares a dependency it does not use, and names the crate and the dependency.

## Why

The shared rule in `skills/deslop-code.md` requires every Rust routine gate to fail on an unused dependency, and BioData's gate has no such check.

## Proof

- Proof: one `pm job` on main with a temporary unused dependency added to `crates/biodata/Cargo.toml` fails `make gate` at the new step and names that dependency. The same job without it passes. `git status` is clean after the check runs.

## Starts from

- Starts from: the `gate` target in `Makefile`, and `rust-toolchain.toml`.
- Use `cargo shear`, as `skills/deslop-code.md` names, at a pinned version. The build machines do not have it, so the gate installs or finds that exact version itself, with `--locked`, and never relies on a copy already on the machine.
- Never pass `--fix`. Treat each hit as a lead, as `skills/deslop-code.md` says: check `#[path]` includes, build scripts, macros, renamed packages and features first. Record a confirmed false positive with its reason in `[package.metadata.cargo-shear] ignored`.

## Keeps

- Keeps: every existing gate step and its order before the new step.

## Defers

- Defers: nothing.

## Depends on

Depends on: 0006

## Changes

- Changes: `Makefile`, `crates/biodata/Cargo.toml`
