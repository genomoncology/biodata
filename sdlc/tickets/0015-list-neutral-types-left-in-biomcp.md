# 0015 — list-neutral-types-left-in-biomcp

Status: COMPLETE.
Landed: 509a4bbb6a7168b0a41a1348ce8b870ab04457a0

Milestone: 1

## Outcome

`sdlc/designs/2026-11-01-neutral-types-left-in-biomcp.md` lists every public type in BioMCP's types and adapters crates that holds a neutral value BioData's boundary covers, with its BioMCP path, the BioData module it would join, the BioData type it duplicates or extends if any, and the BioMCP types that depend on it. Every other public type in those crates is counted as BioMCP's own, by kind: request, reply, paging, source identity or raw source row.

## Why

BioData is to be the universal core for every consumer, and a neutral value left in BioMCP is one each other consumer must rebuild.

## Proof

- Proof: a second fresh reviewer samples at least one type of each kind from the BioMCP-owned counts and every listed candidate, and finds none misfiled. The list's totals match a count of `pub struct` and `pub enum` items in the two crates at the named commit.

## Starts from

- Starts from: BioMCP's main at `5ac79116c` or later, the commit the review names, crates `biomcp-types` and `biomcp-adapters`, read only. BioData's side is `crates/biodata` at the 0006 pin.
- BioData's boundary is `CLAUDE.md`'s Boundary section, sharpened by [Ian's pure content ruling](../decisions/2026-11-01-biodata-holds-pure-content.md). BioMCP's own split is its 2026-10-23 decision that BioData holds neutral entity values, identifiers, coordinates, notation and vocabularies, and `biomcp-types` holds BioMCP requests, replies, source identities and their schemas.
- Both crates hold folders that share BioData's topic names: adverse events, articles, authors, cell lines, drugs, GWAS, pharmacogenomics, phenotypes, proteins and studies. Start there.
- PM decision 2026-11-01: this is a read-only review, run beside 0006 without delaying it. Nothing moves before BioData 1.0. The PM rules on the list, files the moves as tickets for after 1.0, and asks BioMCP's PM to accept each one.

## Keeps

- Keeps: BioMCP's repository untouched. The list quotes no source data, and it names no private project.

## Defers

- Defers: moving any type, and any change to BioMCP, to tickets the PM files from the list. The second consumer's own types wait for a separate review once it has switched.

## Changes

- Changes: `sdlc/designs/2026-11-01-neutral-types-left-in-biomcp.md`
