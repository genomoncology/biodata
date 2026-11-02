# Tighten frozen scalar schema patterns that admit more than their parsers: Mane inline RefSeq and version, Bound null-only bound and u32 maximum, unbounded LRG zeros in Accession, AccessionBase and HgvsText references, under new schema versions

Status: open.

Kind: idea
When: After BioData 1.0 is tagged and both consumers have switched

## Problem

Tighten frozen scalar schema patterns that admit more than their parsers: Mane inline RefSeq and version, Bound null-only bound and u32 maximum, unbounded LRG zeros in Accession, AccessionBase and HgvsText references, under new schema versions. Each schema then states exactly what its parser accepts, so consumers can validate with the schema alone.
