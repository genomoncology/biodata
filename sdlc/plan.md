# BioData plan

Owner: BioData PM. This plan says what to build, why and in what order. It never says what is done. Read status from `pm daily`, `pm next`, `pm item` and `pm inbox`.

Rulings: [decisions](decisions/). Milestones: [milestones](planning/milestones.md).

## Outcome

BioData is one free Rust crate of typed biomedical values that BioMCP and other consumers share. Version 1.0 merges BioMCP's crate with a second consumer's crate and ships with BioMCP 1.0 ([ruling](decisions/2026-10-28-biodata-1-0-merges-two-crates.md)). Later releases add patient-level values read from FHIR and OMOP, then SDKs in other languages. BioData releases follow BioMCP releases.

## What 1.0 must keep

- BioMCP's replies show an empty diff against its last in-tree build. Its `biodata.*` schemas change only by optional additions the reconciliation record lists, where the second consumer's shape is wider.
- BioMCP pins an exact revision that resolves on clean hosted runners and in its offline release build.
- The second consumer's hot-path values keep their byte layout, parse behavior and cost. Its output and index bytes stay identical, and its lookups get no slower.
- The crate makes no network call, holds no credential, reads no database and makes no clinical interpretation.

## Order

1. Reconcile the two crates on paper before code arrives. The record gives every public item a merge rule and proposes the architecture decisions below.
2. When BioMCP's PM names its extraction commit, import both crates as snapshots and merge them by the record. From that commit until BioMCP switches, BioMCP leaves its crate alone.
3. Each consumer switches to the merged crate in its own repository and proves its constraint above.
4. Before BioMCP cuts its release candidate, the old repository is renamed and archived, this one takes the name `biodata` and becomes public with Ian's approval, and 1.0.0 is tagged. BioMCP 1.0 ships on it after Ian approves publication.

Code fixes before step 2 go to BioMCP's PM as asks, because BioMCP owns its crate until then.

## Architecture decisions to settle

The PM rules on each in the import ticket before step 2:

- Crate layout: one crate with features, or a small workspace.
- Serialization: BioMCP derives it always, and the second consumer keeps it behind a feature. JSON schema export sits behind a feature.
- Lints: the union of both consumers' lint sets, with each exception named.
- Schema ownership: which `biodata.*` schema IDs BioData owns and which belong to BioMCP.
- Distribution: how BioMCP pins a revision that resolves offline.
- Which rules from the old repository's architecture records carry forward, such as parsing without interpreting, typed refusals, and private fields with validating constructors.
- The Python distribution name, because `biodata` is taken on PyPI. This waits for the SDK milestone.

## Pressure testing

Ian's rule: the first users are public datasets, experiments and papers that we try to replicate. A replication failure in a 1.0 value is a fix for 1.0. The replication set milestone holds this work.

## Checks

`make gate`, run through `pm job`.
