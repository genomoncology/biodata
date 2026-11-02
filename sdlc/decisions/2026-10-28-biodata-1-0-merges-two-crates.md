# BioData 1.0 merges two consumers' crates

Date: 2026-10-28

Decided by Ian.

## Ruling

In Ian's words, cleaned from dictation, with a private project's name replaced by "the second consumer":

"The biodata repo that we currently have should be archived and started from scratch with new architectural decisions, bootstrapped with the merger of BioMCP's biodata crate and the second consumer's biodata crate. That becomes biodata 1.0, from which both projects work. Tie the biodata releases to BioMCP releases, at least for the near future. Biodata then supports the addition of FHIR, OMOP and other extensions to BioMCP. The biodata part that knows the data model, the object types and fields, is free."

"I am in no rush to get 1.0 out the door. My goal is to get the right thing out the door." Types already in BioMCP 1.0 that are incomplete or wrong get fixed before the cut. FHIR and OMOP wait until after 1.0.

Ian chose two options the same day:

1. BioData 1.0 is cut before BioMCP 1.0, and BioMCP 1.0 ships on it.
2. The name stays BioData.

Ian approved creating this repository under a working name. At the cut, the old repository is renamed and archived, and this one takes the name `biodata`.

## Why

Two consumers each grew their own crate for the same values. Two copies drift, and every fix lands twice. One free crate lets both consumers and outside users share one model, and it gives FHIR and OMOP readers one place to land.

## Options considered

- Keep separate crates and merge later. Ian's ruling of 2026-10-24 chose this. This ruling replaces it.
- Merge after BioMCP 1.0. Rejected because BioMCP's published schemas would then freeze types that the merge changes.
- Restart the old repository in place. Rejected because Ian wants new architecture decisions with no inherited history.

## Rules that follow

- BioMCP extracts its crate from one commit that its PM names. The merged crate must give BioMCP an empty reply diff against its last in-tree build. BioMCP's 1.0 wire is the baseline, and its `biodata.*` schemas change only by optional additions where the second consumer's shape is wider.
- The second consumer's hot-path values (accession, position, chromosome and its parser, allele key, span, base and the HGVS parser) are the base. They keep their byte layout, parse behavior and cost. Any change is listed and measured by that consumer before it lands.
- Code arrives as a snapshot. No other repository's history is imported.

## Replaces

The 2026-10-24 ruling that the two consumers build separate crates with no merge planned.
