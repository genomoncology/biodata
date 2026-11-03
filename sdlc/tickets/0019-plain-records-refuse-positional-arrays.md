# 0019 — plain-records-refuse-positional-arrays

Status: COMPLETE.
Landed: e79f36c13a0214cf2039ef5d2a722e4db977b834

Milestone: 1

## Outcome

Every public plain record in `crates/biodata` that derives `Deserialize` refuses a JSON array in place of its object and still accepts the object its schema states. The records share one declaration for this, which is the path the validated values already use.

## Why

A record's JSON schema states an object, but serde's derived reader also accepts a positional array, so the reader admits input the schema forbids.

## Proof

- Proof: `crates/biodata/tests/object_inputs.rs` gains a test that walks every public plain record deriving `Deserialize`. For each one, it parses a synthetic object, refuses the same values as a positional array, and checks that the object round trip is unchanged. The test fails on the 1.0.0 tag. The walk counts the records from the source, so a new plain record that skips the shared declaration fails the test. One `pm job` runs `make gate` and passes.

## Starts from

- Starts from: `crates/biodata/src/serde_object.rs` and its `object_deserialize` macro, and the validated-value witnesses in `crates/biodata/tests/object_inputs.rs`.
- PM decision 2026-11-03: the refusal applies only to human-readable formats, as `serde_object::deserialize` does now. Binary formats keep the derived reader, so a consumer's binary index bytes do not change.
- PM decision 2026-11-03: this ships in a 1.x release. Each record's schema already forbids an array, so the change only enforces the stated contract. Serialized output stays byte-identical.

## Keeps

- Keeps: every field name, default, `rename` and `skip` attribute, every schema, and every serialized byte. The parse path for `biodata::alteration::parse` stays untouched.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biodata/src/`, `crates/biodata/tests/object_inputs.rs`
