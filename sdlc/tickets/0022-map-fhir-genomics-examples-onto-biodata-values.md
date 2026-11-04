# 0022 — map-fhir-genomics-examples-onto-biodata-values

Status: WITHDRAWN. Ian set BioData idle on 2026-11-04; work in progress kept on the ticket branches for a later restart

Milestone: 3

## Outcome

`sdlc/designs/2026-11-04-fhir-genomics-value-map.md` lists every coded or measured element in the mCODE STU4 genomics examples and the Genomics Reporting STU3 examples, each with the BioData 1.0 value that holds it or the gap if none does. It also lists the 47 semantic objects of the JAMIA Open 2026 somatic report study the same way. The gaps form the value list for the 1.1 clinical milestone.

## Why

Milestone 3 needs a checked list of the small values FHIR genomics data uses before its tickets are cut.

## Proof

- Proof: a fresh reviewer samples at least ten mapped elements and every listed gap against the published examples and BioData's source at the named commit, and finds none misfiled. The design counts how many of the 47 objects BioData holds, against the study's target of 41.

## Starts from

- Starts from: item 12 and item 13 in `sdlc/designs/2026-10-28-replication-corpus.md`, the [clinical values ruling](../decisions/2026-10-30-clinical-milestone-builds-small-values-only.md), and `crates/biodata` on main.
- Sources: https://hl7.org/fhir/us/mcode/examples_genomics.html, https://build.fhir.org/ig/HL7/genomics-reporting/downloads.html, and https://academic.oup.com/jamiaopen/article/doi/10.1093/jamiaopen/ooag022/8508642. Name elements and objects; quote no article text.

## Keeps

- Keeps: code unchanged. This is a read-only mapping.

## Defers

- Defers: building any missing value, to milestone 3 tickets the PM files from the gaps.

## Changes

- Changes: `sdlc/designs/2026-11-04-fhir-genomics-value-map.md`
