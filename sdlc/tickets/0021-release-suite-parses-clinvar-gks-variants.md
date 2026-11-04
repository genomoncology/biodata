# 0021 — release-suite-parses-clinvar-gks-variants

Status: WITHDRAWN. Ian set BioData idle on 2026-11-04; work in progress kept on the ticket branches for a later restart

Milestone: 2

## Outcome

`make release-suite` also reads a pinned ClinVar-GKS release from the data root and fails unless every variant's identifiers and HGVS text parse into BioData 1.0 values or are refused with a named reason a reviewed row accepts, and the variant IDs BioData prints match ClinVar's own. It prints counts per kind of parsed, refused and reviewed values.

## Why

ClinVar is the widest free source of real HGVS text and variant identifiers, so it catches parser misses the small corpora cannot.

## Proof

- Proof: one `pm job` runs `make fetch-corpora` and `make release-suite` and passes. The same job with one reviewed row removed fails and names that row. Peak memory stays bounded because the reader streams the release instead of loading it whole.

## Starts from

- Starts from: the `fetch-corpora` and `release-suite` targets from 0020.
- Source: https://dataexchange.clinicalgenome.org/clinvar-gks/ monthly JSON release, free with VRS identities. Pin one release by date and SHA-256.
- PM decision 2026-11-04: the same data-root and review rules as 0020.

## Keeps

- Keeps: `make gate` unchanged, and every 0020 check.

## Defers

- Defers: comparing VRS identities to BioData's computed ones, which waits for a ticket once BioData computes them.

## Depends on

Depends on: 0020

## Changes

- Changes: `Makefile`, `crates/biodata/tests/`, `scripts/`
