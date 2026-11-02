# Milestones

## 1. BioData 1.0: one crate for two consumers

Delivers one crate that merges BioMCP's and the second consumer's crates. BioMCP 1.0 ships on it, and the second consumer switches to it.

Serves the [2026-10-28 ruling](../decisions/2026-10-28-biodata-1-0-merges-two-crates.md).

Done when BioMCP shows an empty reply diff against its last in-tree build, with `biodata.*` schemas that differ only by listed optional additions, the second consumer shows identical output, identical index bytes and no lookup slowdown, both pin the 1.0.0 tag, and Ian approves publication with BioMCP 1.0.

## 2. Replication set

Delivers public and synthetic datasets and published results that BioData's values and BioMCP's commands reproduce, run as a release suite.

Serves Ian's rule that the first users are datasets, experiments and papers we try to replicate.

Done when each chosen dataset or paper has a run that reproduces its named result, or a ticket that owns the gap.

The [replication corpus](../designs/2026-10-28-replication-corpus.md) names the datasets and results and who runs each. Before the cut, BioMCP runs the runs that test its values and commands, and the second consumer runs the HGVS corpora, because each owns that code until then. After the merge, the parse corpora move into BioData's release suite. Break BioData's part into tickets when the merged crate lands.

## 3. BioData 1.1: clinical values from FHIR and OMOP

Delivers the small shared values that patient data in FHIR R4 and the OMOP CDM needs: times with their precision, decimals, units and quantities, coded values with source and standard concepts, identifiers, record keys, sources and losses. Consumers' adapters build records from them and report what each mapping keeps, changes and loses ([ruling](../decisions/2026-10-30-clinical-milestone-builds-small-values-only.md)).

Serves the 2026-10-28 ruling that BioData supports FHIR, OMOP and other extensions to BioMCP.

Done when the replication set's first clinical slice round-trips through both formats in a consumer's adapter built on these values, with its losses reported.

Still unknown: which FHIR profiles (mCODE, Genomics Reporting) and which OMOP extensions come first. Break this into tickets after the 1.0 cut.

Two experiments tested whether BioData should also ship clinical records. Neither found that one record model fits both backends or that shared records cost less for each added format, so the 2026-10-30 ruling builds the small values only. The [patient data design](../designs/2026-10-29-patient-data.md) stays as reference for the values and the on-premises rule.

## 4. SDKs in Python and TypeScript

Delivers BioData values in Python and TypeScript, backed by the Rust crate.

Serves Ian's aim of first-class SDKs in each language.

Done when a Python and a TypeScript user can parse, validate and serialize every 1.0 value with the same results as Rust.

Still unknown: the Python distribution name and the binding approach. Break this into tickets after milestone 3's values settle.
